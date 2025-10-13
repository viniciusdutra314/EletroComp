import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
from scipy.optimize import curve_fit
plt.style.use("ggplot")
fig,axis=plt.subplots(ncols=2,figsize=(10,5))

df=pd.read_csv("results/ex07_comparison.csv")

N_VALUES=np.linspace(min(df['N']),max(df['N']),100)

axis[0].set_xlabel("N")
axis[0].set_xscale("log")
axis[0].set_yscale("log")
axis[0].set_ylabel("Number of Iterations")
jacobi_fitting=np.polyfit(df['N'],df['Jacobi Iterations'],2)
axis[0].plot(N_VALUES,np.polyval(jacobi_fitting,N_VALUES),label=r"$aN^2+bN+c$",color="red",linestyle='--')
sor_fitting=np.polyfit(df['N'],df['SOR Iterations'],1)
axis[0].plot(N_VALUES,np.polyval(sor_fitting,N_VALUES),label=r"$aN+b$",color="blue",linestyle='--')
axis[0].scatter(df["N"],df["Jacobi Iterations"],label="Jacobi",marker="o",color='red')
axis[0].scatter(df["N"],df["SOR Iterations"],label="SOR",marker="o",color='blue')


axis[1].set_ylabel("Speedup (execution time ratio)")
axis[1].set_xlabel("N")
axis[1].plot(df["N"],df["Jacobi Time (s)"]/df["SOR Time (s)"],marker="o",color='green')
for ax in axis:
    ax.legend()


fig.savefig("results/ex07_comparison.png",dpi=300)