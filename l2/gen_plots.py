import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt
from pathlib import Path

# Create directory for plots.
plots_dir = Path(__file__).parent / "plots"
plots_dir.mkdir(exist_ok=True, parents=True)

# Define the path to the results CSV file.
results_path = Path(__file__).parent / "cache_results.csv"

# Read the CSV file (delimiter is ";" because the file uses semicolons)
df = pd.read_csv(results_path, delimiter=";")

# Set publication-quality style.
sns.set(style="whitegrid", context="talk")
# Removed plt.rcParams["png.fonttype"] because it is not valid in your matplotlib version.

# Group data to average avg_cost over any duplicate (n, k, cache_strategy, distribution) combinations.
df_grouped = df.groupby(["distribution", "n", "cache_strategy", "k"], as_index=False).agg({"avg_cost": "mean"})

# ---------------------------------------------------------------------
# Plot 1: For each distribution, plot average cost vs. n for all cache strategies.
# ---------------------------------------------------------------------
distributions = df_grouped["distribution"].unique()
for dist in distributions:
    df_dist = df_grouped[df_grouped["distribution"] == dist]
    
    # Calculate avg_cost_dist_min (lowest k) and avg_cost_dist_max (highest k)
    df_dist_min = df_dist[df_dist.groupby(["n", "cache_strategy"])["k"].transform("min") == df_dist["k"]]
    df_dist_max = df_dist[df_dist.groupby(["n", "cache_strategy"])["k"].transform("max") == df_dist["k"]]
    
    plt.figure(figsize=(10, 6))
    
    # Plot avg_cost for all k
    sns.lineplot(
        data=df_dist,
        x="n",
        y="avg_cost",
        hue="cache_strategy",
        marker="o",
        palette="tab10"
    )
    plt.title(f"Average Cost vs. Endpoint (n) for Distribution: {dist} (All k)")
    plt.savefig(plots_dir / f"avg_cost_all_k_{dist}.png", dpi=300, bbox_inches="tight")
    plt.close()
    
    # Plot avg_cost_dist_min (lowest k)
    plt.figure(figsize=(10, 6))
    sns.lineplot(
        data=df_dist_min,
        x="n",
        y="avg_cost",
        hue="cache_strategy",
        marker="o",
        palette="tab10"
    )
    plt.title(f"Average Cost vs. Endpoint (n) for Distribution: {dist} (Lowest k)")
    plt.xlabel("Endpoint (n)")
    plt.ylabel("Average Cost")
    plt.legend(title="Cache Strategy")
    plt.savefig(plots_dir / f"avg_cost_lowest_k_{dist}.png", dpi=300, bbox_inches="tight")
    plt.close()
    
    # Plot avg_cost_dist_max (highest k)
    plt.figure(figsize=(10, 6))
    sns.lineplot(
        data=df_dist_max,
        x="n",
        y="avg_cost",
        hue="cache_strategy",
        marker="o",
        palette="tab10"
    )
    plt.title(f"Average Cost vs. Endpoint (n) for Distribution: {dist} (Highest k)")
    plt.xlabel("Endpoint (n)")
    plt.ylabel("Average Cost")
    plt.legend(title="Cache Strategy")
    plt.savefig(plots_dir / f"avg_cost_highest_k_{dist}.png", dpi=300, bbox_inches="tight")
    plt.close()

# ---------------------------------------------------------------------
# Additional Plot 1: Facet Grid by Cache Strategy.
# Each facet shows avg_cost vs. n, with hue by distribution.
# ---------------------------------------------------------------------
g1 = sns.FacetGrid(
    df_grouped,
    col="cache_strategy",
    col_wrap=3,
    height=4,
    aspect=1.2,
    hue="distribution",
    palette="Set2"
)
g1.map(sns.lineplot, "n", "avg_cost", marker="o")
g1.add_legend(title="Distribution")
g1.set_axis_labels("Endpoint (n)", "Average Cost")
g1.fig.suptitle("Average Cost vs. n by Cache Strategy (Hue = Distribution)", y=1.02)
plt.savefig(plots_dir / "avg_cost_by_strategy.png", dpi=300, bbox_inches="tight")
plt.close()

# ---------------------------------------------------------------------
# Additional Plot 2: For n = 100, plot average cost vs. cache size (k).
# ---------------------------------------------------------------------
df_n100 = df_grouped[df_grouped["n"] == 100]
plt.figure(figsize=(10, 6))
ax = sns.lineplot(
    data=df_n100,
    x="k",
    y="avg_cost",
    hue="cache_strategy",
    style="distribution",
    markers=True,
    dashes=False,
    palette="tab10"
)
ax.set_title("Average Cost vs. Cache Size (k) for n = 100")
ax.set_xlabel("Cache Size (k)")
ax.set_ylabel("Average Cost")
plt.legend(title="Cache Strategy / Distribution", bbox_to_anchor=(1.05, 1), loc="upper left")
plt.savefig(plots_dir / "avg_cost_vs_k_n100.png", dpi=300, bbox_inches="tight")
plt.close()

# ---------------------------------------------------------------------
# Additional Plot 3: For n = 2, plot average cost vs. cache size (k).
# ---------------------------------------------------------------------
df_n2 = df_grouped[df_grouped["n"] == 20]
plt.figure(figsize=(10, 6))
ax = sns.lineplot(
    data=df_n2,
    x="k",
    y="avg_cost",
    hue="cache_strategy",
    style="distribution",
    markers=True,
    dashes=False,
    palette="tab10"
)
ax.set_title("Average Cost vs. Cache Size (k) for n = 20")
ax.set_xlabel("Cache Size (k)")
ax.set_ylabel("Average Cost")
plt.legend(title="Cache Strategy / Distribution", bbox_to_anchor=(1.05, 1), loc="upper left")
plt.savefig(plots_dir / "avg_cost_vs_k_n2.png", dpi=300, bbox_inches="tight")
plt.close()

print("All plots generated and saved in the 'plots' directory.")
