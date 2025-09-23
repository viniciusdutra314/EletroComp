import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D  

potencial = np.load("results/ex04.npy")

plt.figure()
plt.imshow(potencial, cmap="bwr")
plt.colorbar()
plt.title("Electric Potential (2D)")
plt.tight_layout()
plt.savefig("results/eletric_potential.jpg", dpi=200)

ny, nx = potencial.shape
x = np.arange(nx)
y = np.arange(ny)
X, Y = np.meshgrid(x, y)

fig = plt.figure()
ax = fig.add_subplot(111, projection="3d")
rstride = max(1, nx // 100)
cstride = max(1, ny // 100)
ax.plot_wireframe(X, Y, potencial, rstride=rstride, cstride=cstride, color="black", linewidth=0.6)
ax.set_xlabel("x")
ax.set_ylabel("y")
ax.set_zlabel("V")
ax.set_title("Electric Potential (wireframe)")
ax.view_init(elev=30, azim=-60)
fig.tight_layout()
fig.savefig("results/eletric_potential_wire.jpg", dpi=200)

plt.close("all")