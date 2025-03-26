from matplotlib import pyplot as plt
import seaborn as sns
import pandas as pd
from pathlib import Path

plots_folder = Path(__file__).parent / 'plots'
plots_folder.mkdir(exist_ok=True)

# Load the data with columns: n list_type distribution  total_cost
data = pd.read_csv('l1.csv', delimiter=';')
data['avg_cost'] = data['total_cost'] / data['n']
print(data.head())

# Ensure consistent colors for each list_type across plots
hue_order = data['list_type'].unique()
palette = sns.color_palette(n_colors=len(hue_order))

for distribution in data['distribution'].unique():
    subset = data[data['distribution'] == distribution]
    sns.lineplot(data=subset, x='n', y='avg_cost', hue='list_type', hue_order=hue_order, palette=palette)
    plt.title(f'Distribution: {distribution}')
    plt.savefig(plots_folder / f'distribution_{distribution}.png', dpi=300)  # Set dpi for high resolution
    plt.close()

# Zoomed in Uniform distribution plot
subset = data[data['distribution'] == 'Uniform']
subset = subset[subset['n'] >= 5000]  # Only show n >= 1000
sns.lineplot(data=subset, x='n', y='avg_cost', hue='list_type', hue_order=hue_order, palette=palette)
plt.title('Distribution: Uniform')
plt.savefig(plots_folder / 'distribution_Uniform_zoomed.png', dpi=300)
