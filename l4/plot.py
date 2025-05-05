#!/usr/bin/env python3
from pathlib import Path

import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

def plot_distribution_comparison(df: pd.DataFrame, out_dir: Path):
    g = sns.catplot(
        data=df, x="Distribution", y="Cost", hue="Graph",
        col="Algorithm", kind="bar", estimator="mean", ci="sd",
        height=4, aspect=1.2
    )
    g.set_axis_labels("Distribution", "Average Cost")
    g.fig.suptitle("Cost by Distribution and Graph for Each Algorithm", y=1.02)
    g.savefig(out_dir / "compare_by_distribution.png", dpi=300, bbox_inches="tight")
    plt.close(g.fig)

def plot_algorithm_comparison(df: pd.DataFrame, out_dir: Path):
    g = sns.catplot(
        data=df, x="Algorithm", y="Cost", hue="Graph",
        col="Distribution", kind="bar", estimator="mean", ci="sd",
        height=4, aspect=1.2
    )
    g.set_axis_labels("Algorithm", "Average Cost")
    g.fig.suptitle("Cost by Algorithm and Graph for Each Distribution", y=1.02)
    g.savefig(out_dir / "compare_by_algorithm.png", dpi=300, bbox_inches="tight")
    plt.close(g.fig)

def plot_graph_comparison(df: pd.DataFrame, out_dir: Path):
    g = sns.catplot(
        data=df, x="Graph", y="Cost", hue="Algorithm",
        col="Distribution", kind="bar", estimator="mean", ci="sd",
        height=4, aspect=1.2
    )
    g.set_axis_labels("Graph", "Average Cost")
    g.fig.suptitle("Cost by Graph and Algorithm for Each Distribution", y=1.02)
    g.savefig(out_dir / "compare_by_graph.png", dpi=300, bbox_inches="tight")
    plt.close(g.fig)

def plot_cost_vs_D(df: pd.DataFrame, out_dir: Path):
    summary = (
        df
        .groupby(['Algorithm','Distribution','Graph','D'], as_index=False)
        .agg(mean_cost=('Cost','mean'), sd_cost=('Cost','std'))
    )
    g = sns.FacetGrid(
        summary,
        row="Algorithm", col="Distribution",
        hue="Graph",
        margin_titles=True,
        sharey=False,
        height=3.5, aspect=1.2
    )
    def plot_fn(data, **kwargs):
        ax = plt.gca()
        for graph, grp in data.groupby("Graph"):
            ax.plot(grp["D"], grp["mean_cost"], marker="o", label=graph)
            ax.fill_between(
                grp["D"],
                grp["mean_cost"] - grp["sd_cost"],
                grp["mean_cost"] + grp["sd_cost"],
                alpha=0.2
            )
    g.map_dataframe(plot_fn)
    g.set_axis_labels("D (migration cost factor)", "Average Cost")
    g.add_legend(title="Graph")
    g.fig.subplots_adjust(top=0.9)
    g.fig.suptitle("Average Cost vs. D by Algorithm & Distribution")
    g.savefig(out_dir / "cost_vs_D.png", dpi=300, bbox_inches="tight")
    plt.close(g.fig)

def plot_cost_vs_D_split(df: pd.DataFrame, out_dir: Path):
    """
    For each (Algorithm, Distribution), plot mean Cost vs. D in its own figure.
    """
    # precompute the grouped means
    summary = (
        df
        .groupby(['Algorithm','Distribution','Graph','D'], as_index=False)
        .agg(mean_cost=('Cost','mean'), sd_cost=('Cost','std'))
    )
    for alg in summary['Algorithm'].unique():
        for dist in summary['Distribution'].unique():
            sub = summary[(summary['Algorithm']==alg) & (summary['Distribution']==dist)]
            if sub.empty: continue

            plt.figure(figsize=(6,4))
            for graph, grp in sub.groupby('Graph'):
                plt.plot(grp['D'], grp['mean_cost'], marker='o', label=graph)
                plt.fill_between(
                    grp['D'],
                    grp['mean_cost'] - grp['sd_cost'],
                    grp['mean_cost'] + grp['sd_cost'],
                    alpha=0.2
                )
            plt.title(f'Cost vs D  |  {alg}  |  {dist}')
            plt.xlabel('D (migration cost factor)')
            plt.ylabel('Average Cost')
            plt.legend(title='Graph')
            plt.grid(True, linestyle='--', alpha=0.5)
            plt.tight_layout()
            fname = f'cost_vs_D_{alg}_{dist}.png'.replace(' ','_')
            path = out_dir / fname
            plt.savefig(path, dpi=300)
            plt.close()
            print(f"Saved {path}")

def main():
    base = Path(__file__).resolve().parent
    df = pd.read_csv(base / "results.csv")
    out_dir = base / "plots"
    out_dir.mkdir(exist_ok=True)

    sns.set(style="whitegrid")

    plot_distribution_comparison(df, out_dir)
    plot_algorithm_comparison(df, out_dir)
    plot_graph_comparison(df, out_dir)
    plot_cost_vs_D(df, out_dir)
    plot_cost_vs_D_split(df, out_dir)

if __name__ == '__main__':
    main()
