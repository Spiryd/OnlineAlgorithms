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

# Create avg_cost_per_request column
df["avg_cost_per_request"] = df["avg_cost"] / 65_536

# Create output directory for plots
plots_dir = script_dir / "plots"
plots_dir.mkdir(exist_ok=True)

# Common Seaborn style
sns.set(style="whitegrid", font_scale=1.2)

# High resolution
HIGH_DPI = 300

# 2) Line Plot: Average Cost by p for each D
plt.figure(figsize=(10, 7), dpi=HIGH_DPI)
sns.lineplot(
    data=df,
    x="p",
    y="avg_cost",
    hue="D",
    marker="o",
    palette="tab10",
)
plt.title("Average Cost vs. Write Probability")
plt.xlabel("Write Probability (p)")
plt.ylabel("Average Cost")
plt.legend(title="Threshold D")
plt.tight_layout()
plt.savefig(plots_dir / "line_avg_cost_vs_p.png", dpi=HIGH_DPI)
plt.close()

# 3) Line Plot: Average Max Copies by p for each D
plt.figure(figsize=(10, 7), dpi=HIGH_DPI)
sns.lineplot(
    data=df,
    x="p",
    y="avg_max_copies",
    hue="D",
    marker="o",
    palette="tab10",
)
plt.title("Average Maximum Replicas vs. Write Probability")
plt.xlabel("Write Probability (p)")
plt.ylabel("Average Maximum Number of Copies")
plt.legend(title="Threshold D")
plt.tight_layout()
plt.savefig(plots_dir / "line_avg_max_copies_vs_p.png", dpi=HIGH_DPI)
plt.close()

# 4) Heatmaps
# 4a) Heatmap of avg_cost_per_request
pivot_cost = df.pivot(index="D", columns="p", values="avg_cost_per_request")
plt.figure(figsize=(8, 5), dpi=HIGH_DPI)
sns.heatmap(pivot_cost, annot=True, fmt=".6f", cmap="Blues")
plt.title("Heatmap of Avg. Cost per Request")
plt.xlabel("Write Probability (p)")
plt.ylabel("Threshold D")
plt.tight_layout()
plt.savefig(plots_dir / "heatmap_avg_cost_per_request.png", dpi=HIGH_DPI)
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
