import pandas as pd
import matplotlib.pyplot as plt
import scienceplots

# Import parquet file
df = pd.read_parquet('result.parquet')
keys = df.keys()
values = [df[k][0] for k in keys]

# Plot params
pparam = dict(
    title = r'$(|w_j| + \varepsilon)^\eta$',
    xscale = 'linear',
    yscale = 'linear',
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    # Pie chart
    ax.pie(values, labels=keys, autopct='%1.1f\%%', startangle=90)
    fig.savefig('plot.png', dpi=600, bbox_inches='tight')
