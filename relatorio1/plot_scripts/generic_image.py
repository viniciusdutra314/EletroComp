import numpy as np
from cap05_common_plots import *
import matplotlib.pyplot as plt
plt.imshow(np.load("results/generic_image.npy"),cmap="bone")
plt.savefig("results/generic_image.png")