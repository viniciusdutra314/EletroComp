import numpy as np
import os
import matplotlib.pyplot as plt
import matplotlib.cm as cm
plt.style.use("ggplot")
plates_arrays=[(f,np.load(os.path.join("results",f))) for f in os.listdir("results/") if f.startswith("ex04") and f.endswith(".npy")]
plates_arrays.sort(key=lambda tuple: tuple[0])
fig, ax = plt.subplots(2, 2, figsize=(15, 15))
axes = ax.flatten()
for i, (f, small_arr) in enumerate(reversed(plates_arrays)):
    h, w = small_arr.shape
    full_array = np.zeros((2 * h, 2 * w))
    full_array[h:, w:] = small_arr
    full_array[h:, :w] = small_arr[:, ::-1]
    full_array[:h, w:] = -small_arr[::-1, :]
    full_array[:h, :w] = -small_arr[::-1, ::-1]
    full_array = np.rot90(full_array)
    Ey, Ex = np.gradient(-full_array)
    h_full, w_full = full_array.shape
    x, y = np.meshgrid(np.arange(0, w_full, 1), np.arange(0, h_full, 1))
    skip = 100
    
    current_ax = axes[i]
    current_ax.quiver(x[::skip, ::skip], y[::skip, ::skip], Ex[::skip, ::skip], Ey[::skip, ::skip], color='black')
    current_ax.imshow(full_array, cmap="bwr")

    # Create an inset axis for magnification
    axins = current_ax.inset_axes([0.55, 0.55, 0.4, 0.4])

    # Define the zoom region (center of the array)
    zoom_size = h // 6
    x1, x2 = w - zoom_size, w + zoom_size
    y1, y2 = h - zoom_size, h + zoom_size
    
    # Apply the zoom
    axins.set_xlim(x1, x2)
    axins.set_ylim(y2, y1) # Inverted y-axis for imshow
    axins.set_xticklabels([])
    axins.set_yticklabels([])

    # Plot the same data on the inset axis
    inset_skip = 50
    axins.quiver(x[y1:y2:inset_skip, x1:x2:inset_skip], y[y1:y2:inset_skip, x1:x2:inset_skip],
                 Ex[y1:y2:inset_skip, x1:x2:inset_skip], Ey[y1:y2:inset_skip, x1:x2:inset_skip],
                 color='black')
    axins.imshow(full_array, cmap="bwr", extent=[0, w_full, h_full, 0])

    # Draw a box indicating the zoomed area
    current_ax.indicate_inset_zoom(axins, edgecolor="green")

# Hide any unused subplots if there are fewer than 9 arrays
for j in range(i + 1, len(axes)):
    axes[j].axis('off')

fig.tight_layout()
fig.savefig("results/ex04_placas_separadas.jpg",dpi=300)

fig_wire, ax_wire = plt.subplots(2, 2, figsize=(15, 15), subplot_kw={'projection': '3d'})
axes_wire = ax_wire.flatten()

for i, (f, small_arr) in enumerate(reversed(plates_arrays)):
    h, w = small_arr.shape
    full_array = np.zeros((2 * h, 2 * w))
    full_array[h:, w:] = small_arr
    full_array[h:, :w] = small_arr[:, ::-1]
    full_array[:h, w:] = -small_arr[::-1, :]
    full_array[:h, :w] = -small_arr[::-1, ::-1]
    full_array = np.rot90(full_array)
    
    h_full, w_full = full_array.shape
    x, y = np.meshgrid(np.arange(0, w_full, 1), np.arange(0, h_full, 1))
    norm = plt.Normalize(full_array.min(), full_array.max())
    colors = cm.bwr(norm(full_array))
    current_ax = axes_wire[i]
    current_ax.plot_surface(x, y, full_array, facecolors=colors,shade=False)

for j in range(i + 1, len(axes_wire)):
    axes_wire[j].axis('off')

fig_wire.tight_layout()
fig_wire.savefig("results/ex04_wireframes.jpg", dpi=300)
