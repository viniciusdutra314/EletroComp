import numpy as np
from cap05_common_plots import plot_potential_2d_colormap
small_array=np.load("results/ex02_potential.npy")

small_array = np.load("results/ex02_potential.npy")
h, w = small_array.shape
full_array = np.empty((2 * h, 2 * w))

full_array[:h, :w] = small_array[::-1, ::-1]       
full_array[:h, w:] = small_array[::-1, ::]      
full_array[h:, :w] = small_array[::, ::-1]
full_array[h:, w:] = small_array

fig,ax=plot_potential_2d_colormap(full_array,"Eletric Potential","bwr")
fig.savefig("results/ex02_eletric_potential_colormap.jpg", dpi=200)