import numpy as np
from cap05_common_plots import plot_potential_2d_colormap

fig,ax=plot_potential_2d_colormap(np.load("results/ex10_potential.npy"),"Eletric Potential","bwr")
fig.savefig("results/ex01_potential_cmap.jpg", dpi=200)