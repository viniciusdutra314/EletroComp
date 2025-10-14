import matplotlib.pyplot as plt
import numpy as np
from scipy.optimize import curve_fit

data=np.load("results/ex09_potential.npy")
selected_region=len(data)//80
data=data[:selected_region]


x_data = np.arange(1, selected_region + 1)

fit_func=lambda x,A: A/x 
popt, pcov = curve_fit(fit_func, x_data[5:], data[5:])
A_optimal = popt[0]

plt.style.use("ggplot")
fig,ax=plt.subplots()
ax
ax.plot(x_data, data, 'o', label='Dados')
ax.plot(x_data[5:], fit_func(x_data[5:], A_optimal), '-', label=f'Ajuste: V(r>r_min) = {A_optimal:.2f}/r')

ax.set_xlabel("Distância Radial")
ax.set_ylabel("Potencial (V)")
ax.set_yscale("log")
ax.set_xscale("log")
ax.legend()

fig.savefig("results/ex09.png",dpi=300)
