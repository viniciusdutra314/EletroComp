import matplotlib.pyplot as plt
import numpy as np
from scipy.optimize import curve_fit

data=np.load("results/ex09_potential.npy")
data=-np.gradient(data)
charge_size=20

x_data = np.arange(charge_size, 10*charge_size)

fit_func=lambda x,A,B,C: A/(x+C)+B 
popt, pcov = curve_fit(fit_func, x_data, data[charge_size:10*charge_size])

plt.style.use("ggplot")
fig,ax=plt.subplots()
ax
ax.plot(data, 'o', label='Dados')
ax.plot(x_data, fit_func(x_data, *popt), '-', label=r'$E(r>r_{\min}) \propto \frac{1}{r^2}$')

fit_func=lambda x,D: D*x
popt, pcov = curve_fit(fit_func, np.arange(0, charge_size), data[0:charge_size])
ax.plot(np.arange(0, charge_size), fit_func(np.arange(0, charge_size), *popt), '-', 
        label=r'$E(r<r_{\min}) \propto r$')

ax.set_xlabel("Distância Radial (r)")
ax.set_ylabel(r"Campo Elétrico ($\vec{E})$")
ax.set_xlim(0,200)
ax.legend()

fig.savefig("results/ex09.png",dpi=300)
