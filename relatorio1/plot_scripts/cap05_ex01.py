import numpy as np
from cap05_common_plots import plot_potential_2d_colormap

fig,ax=plot_potential_2d_colormap(np.load("results/ex01_potential_small.npy"),"bone")
fig.savefig("results/ex01_potential_small.jpg", dpi=200)
fig,ax=plot_potential_2d_colormap(np.load("results/ex01_potential_big.npy"),"bone")
fig.savefig("results/ex01_potential_big.jpg", dpi=200)
