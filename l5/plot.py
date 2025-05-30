# analyze.py
from pathlib import Path
import pandas as pd
import seaborn as sns
import matplotlib.pyplot as plt

# 1) Load data
script_dir = Path(__file__).parent
data_path = script_dir / "results.csv"
df = pd.read_csv(data_path)

# Ensure correct dtypes
df["D"] = df["D"].astype(int)
df["p"] = df["p"].astype(float)
df["avg_cost"] = df["avg_cost"].astype(float)
df["avg_max_copies"] = df["avg_max_copies"].astype(float)

# Create output directory for plots
plots_dir = script_dir / "plots"
plots_dir.mkdir(exist_ok=True)

# Common Seaborn style
sns.set(style="whitegrid", font_scale=1.2)

# High resolution
HIGH_DPI = 300

# 2) Bar Plot: Average Cost by p for each D
plt.figure(figsize=(10, 7), dpi=HIGH_DPI)
sns.barplot(
    data=df,
    x="p",
    y="avg_cost",
    hue="D",
    palette="tab10",
)
plt.title("Average Request Cost vs. Write Probability")
plt.xlabel("Write Probability (p)")
plt.ylabel("Average Cost per Request")
plt.legend(title="Threshold D")
plt.tight_layout()
plt.savefig(plots_dir / "bar_avg_cost_vs_p.png", dpi=HIGH_DPI)
plt.close()

# 3) Bar Plot: Average Max Copies by p for each D
plt.figure(figsize=(10, 7), dpi=HIGH_DPI)
sns.barplot(
    data=df,
    x="p",
    y="avg_max_copies",
    hue="D",
    palette="tab10",
)
plt.title("Average Maximum Replicas vs. Write Probability")
plt.xlabel("Write Probability (p)")
plt.ylabel("Average Maximum Number of Copies")
plt.legend(title="Threshold D")
plt.tight_layout()
plt.savefig(plots_dir / "bar_avg_max_copies_vs_p.png", dpi=HIGH_DPI)
plt.close()

# 4) Heatmaps
# 4a) Heatmap of avg_cost
pivot_cost = df.pivot(index="D", columns="p", values="avg_cost")
plt.figure(figsize=(8, 5), dpi=HIGH_DPI)
sns.heatmap(pivot_cost, annot=True, fmt=".2f", cmap="Blues")
plt.title("Heatmap of Avg. Cost")
plt.xlabel("Write Probability (p)")
plt.ylabel("Threshold D")
plt.tight_layout()
plt.savefig(plots_dir / "heatmap_avg_cost.png", dpi=HIGH_DPI)
plt.close()

# 4b) Heatmap of avg_max_copies
pivot_max = df.pivot(index="D", columns="p", values="avg_max_copies")
plt.figure(figsize=(8, 5), dpi=HIGH_DPI)
sns.heatmap(pivot_max, annot=True, fmt=".1f", cmap="Greens")
plt.title("Heatmap of Avg. Max Copies")
plt.xlabel("Write Probability (p)")
plt.ylabel("Threshold D")
plt.tight_layout()
plt.savefig(plots_dir / "heatmap_avg_max_copies.png", dpi=HIGH_DPI)
plt.close()
