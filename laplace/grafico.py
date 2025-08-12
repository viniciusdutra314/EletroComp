import numpy as np
import matplotlib.pyplot as plt

potencial=np.load("teste.npy")
plt.imshow(potencial)
plt.savefig("teste.jpg")