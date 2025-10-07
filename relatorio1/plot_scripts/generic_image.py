import numpy as np
import matplotlib.pyplot as plt
plt.imshow(np.load("results/generic_image.npy"),cmap="viridis")
plt.savefig("results/generic_image.png")