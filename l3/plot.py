import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt
import numpy as np
from pathlib import Path

# Setup
data_path = Path('results.csv')
plots_dir = Path('plots')
plots_dir.mkdir(exist_ok=True)

df = pd.read_csv(data_path, sep=';')
df['optimum'] = np.ceil(df['item_sum'])
df['competitive_ratio'] = df['bin_count'] / df['optimum']

# Define the consistent order of strategies
strategy_order = ['NextFit', 'RandomFit', 'FirstFit', 'BestFit', 'WorstFit']

# Define a consistent color palette
palette = sns.color_palette("Set2", n_colors=len(strategy_order))
strategy_palette = dict(zip(strategy_order, palette))

sns.set(style="whitegrid")

def annotate_medians(ax, data, x_col, y_col, fmt="{:.2f}"):
    medians = data.groupby(x_col)[y_col].median()
    categories = [t.get_text() for t in ax.get_xticklabels()]
    positions = ax.get_xticks()
    for pos, cat in zip(positions, categories):
        if cat in medians:
            val = medians[cat]
            ax.text(pos, val, fmt.format(val),
                    ha='center', va='bottom', fontsize=9, color='black')

# Boxplots per distribution
for dist, group in df.groupby('distribution'):
    fig, ax = plt.subplots()
    sns.boxplot(x='strategy', y='competitive_ratio', data=group, hue='strategy',
                order=strategy_order, palette=strategy_palette, ax=ax, legend=False)
    annotate_medians(ax, group, 'strategy', 'competitive_ratio')
    ax.set_title(f'Competitive Ratio by Strategy ({dist})')
    ax.set_xlabel('Strategy')
    ax.set_ylabel('Competitive Ratio')
    plt.xticks(rotation=45)
    plt.tight_layout()
    fig.savefig(plots_dir / f'{dist}_competitive_ratio_boxplot.png', dpi=300)
    plt.close()

# Bar plot of mean competitive ratio
fig, ax = plt.subplots()
sns.barplot(x='distribution', y='competitive_ratio', hue='strategy', data=df,
            estimator='mean', errorbar=None, hue_order=strategy_order, palette=strategy_palette, ax=ax)
ax.set_title('Mean Competitive Ratio by Strategy and Distribution')
ax.set_xlabel('Distribution')
ax.set_ylabel('Mean Competitive Ratio')
plt.xticks(rotation=45)
plt.legend(title='Strategy', bbox_to_anchor=(1.05, 1), loc='upper left')
plt.tight_layout()
fig.savefig(plots_dir / 'mean_competitive_ratio_barplot.png', dpi=300)
plt.close()

# Overall boxplot
fig, ax = plt.subplots()
sns.boxplot(x='strategy', y='competitive_ratio', data=df, hue='strategy',
            order=strategy_order, palette=strategy_palette, ax=ax, legend=False)
annotate_medians(ax, df, 'strategy', 'competitive_ratio')
ax.set_title('Overall Competitive Ratio by Strategy')
ax.set_xlabel('Strategy')
ax.set_ylabel('Competitive Ratio')
plt.xticks(rotation=45)
plt.tight_layout()
fig.savefig(plots_dir / 'overall_competitive_ratio_boxplot.png', dpi=300)
plt.close()

# FacetGrid — Bin Count Boxplots (2x2 layout) with proper annotations
g = sns.FacetGrid(df, col='distribution', col_wrap=2, sharey=True, height=4, aspect=1.2)
g.map_dataframe(sns.boxplot, x='strategy', y='bin_count', hue='strategy',
                order=strategy_order, palette=strategy_palette, legend=False)
g.set_titles(col_template="{col_name}")
g.set_axis_labels("Strategy", "Bin Count")

# Rotate x-axis labels
for ax in g.axes.flat:
    for label in ax.get_xticklabels():
        label.set_rotation(45)

plt.tight_layout()
g.savefig(plots_dir / 'facet_bin_count_boxplots_2x2.png', dpi=300)
plt.close()