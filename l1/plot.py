from matplotlib import pyplot as plt
import seaborn as sns
import pandas as pd

# Load the data with columns: n list_type distribution  total_cost
data = pd.read_csv('l1.csv', delimiter=';')
data['avg_cost'] = data['total_cost'] / data['n']
print(data.head())

# plot
for distribution in data['distribution'].unique():
    subset = data[data['distribution'] == distribution]
    sns.lineplot(data=subset, x='n', y='avg_cost', hue='list_type')
    plt.title(f'Distribution: {distribution}')
    plt.savefig(f'distribution_{distribution}.png', dpi=300)  # Set dpi for high resolution
    plt.close()