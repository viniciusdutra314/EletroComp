import numpy as np
import matplotlib.pyplot as plt

data = np.load("results/ex08_potential.npy")
Ey, Ex = np.gradient(-data)

ny, nx = data.shape
x,y = np.mgrid[0:nx, 0:ny]

charge_pos_y, charge_pos_x = np.unravel_index(np.argmax(data), data.shape)
fig, ax = plt.subplots(figsize=(10, 8))

contour = ax.contourf(x, y, data, levels=20, cmap='bone')
fig.colorbar(contour, label='Electric Potential (V)')

ax.contour(x, y, data, levels=10, colors='white', linewidths=0.75)
ax.scatter(charge_pos_x, charge_pos_y,  color='white', label='Point Charge')


ax.set_xlabel('x position')
ax.set_ylabel('y position')
ax.set_aspect('equal')
ax.legend()

ax.set_xlim(nx*0.85, nx-1)
ax.set_ylim(ny*0.85, ny-1)

plt.savefig("results/ex08_charge_near_plate.png", dpi=300, bbox_inches='tight')