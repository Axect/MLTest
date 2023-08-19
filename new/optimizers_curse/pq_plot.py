import pandas as pd
import matplotlib.pyplot as plt
import scienceplots

# Import parquet file
df = pd.read_parquet('distribution.parquet')

# Prepare Data to Plot
x1 = df['1']
x2 = df['2']
x5 = df['5']
x10 = df['10']

# Plot params
pparam = dict(
    xlabel = r"$\max_{a'}Q^{\pi}(s',a')$",
    ylabel = r'Counts',
    xscale = 'linear',
    yscale = 'linear',
)

# Plot
with plt.style.context(["science", "nature"]):
    fig, ax = plt.subplots()
    ax.autoscale(tight=True)
    ax.set(**pparam)
    ax.hist(x1, bins=100, histtype='step', label=r'$k=1$')
    ax.hist(x2, bins=100, histtype='step', label=r'$k=2$')
    ax.hist(x5, bins=100, histtype='step', label=r'$k=5$')
    ax.hist(x10, bins=100, histtype='step', label=r'$k=10$')
    ax.legend()
    fig.savefig('dist_plot.png', dpi=600, bbox_inches='tight')
