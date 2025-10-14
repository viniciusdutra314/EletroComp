import numpy as np
from cap05_common_plots import plot_potential_2d_colormap
import matplotlib.pyplot as plt
array=np.load("results/ex06_potential.npy")
potential_rotated = np.rot90(array, 3)
fig,ax=plot_potential_2d_colormap(potential_rotated,"bone")

Ey, Ex = np.gradient(-potential_rotated)
x = np.arange(0, Ex.shape[1], 1)
y = np.arange(0, Ey.shape[0], 1)
ax.streamplot(x, y, Ex, Ey, linewidth=0.8, density=1.0)

fig.savefig("results/ex06_potential.jpg", dpi=150)