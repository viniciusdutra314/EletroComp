import numpy as np
from cap05_common_plots import plot_potential_2d_colormap
small_array=np.load("results/ex02_potential.npy")

h, w = small_array.shape
full_array = np.empty((2 * h, 2 * w))

full_array[:h, :w] = small_array[::-1, ::-1]       
full_array[:h, w:] = small_array[::-1, ::]      
full_array[h:, :w] = small_array[::, ::-1]
full_array[h:, w:] = small_array

fig,ax=plot_potential_2d_colormap(full_array,"bone")
ax.axhline(y=h,linestyle="--",color="black",alpha=0.3)
ax.axvline(x=w,linestyle="--",color="black",alpha=0.3)
ax.legend(["Eixo de simetria"],loc="upper left",fontsize=8)
fig.savefig("results/ex02_eletric_potential_entire.jpg", dpi=200)

fig,ax=plot_potential_2d_colormap(small_array,"bone")
fig.savefig("results/ex02_eletric_potential_partial.jpg", dpi=200)