import numpy as np
from cap05_common_plots import plot_potential_2d_colormap,plot_wireframe
import matplotlib.pyplot as plt
plt.style.use("ggplot")
small_array=np.load("results/ex03_potential.npy")
h, w = small_array.shape
full_array = np.zeros((2 * h, 2 * w))

full_array[h:, w:] = small_array
full_array[h:, :w] = small_array[::, ::-1]
full_array[:h, w:] = -small_array[::-1,::]
full_array[:h, :w] = -small_array[::-1, ::-1]

full_array=np.rot90(full_array)
fig,ax=plot_potential_2d_colormap(full_array,"bwr")
fig.savefig("results/ex03_eletric_potential_colormap.jpg", dpi=200)
fig,ax=plot_wireframe(full_array)
fig.savefig("results/ex03_eletric_potential_wire.jpg", dpi=200)