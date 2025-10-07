import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
plt.style.use("ggplot")
fig,axis=plt.subplots(ncols=2,figsize=(10,5),sharex=True)

df=pd.read_csv("results/ex07_comparison.csv")

N_VALUES=np.linspace(min(df['N']),max(df['N']),100)

axis[0].set_ylabel("Number of Iterations")
jacobi_fitting=np.polyfit(df['N'],df['Jacobi Iterations'],2)
axis[0].plot(N_VALUES,np.polyval(jacobi_fitting,N_VALUES),label=r"$aN^2+bN+c$",color="red")
sor_fitting=np.polyfit(df['N'],df['SOR Iterations'],1)
axis[0].plot(N_VALUES,np.polyval(sor_fitting,N_VALUES),label=r"$aN+b$",color="blue")
axis[0].scatter(df["N"],df["Jacobi Iterations"],label="Jacobi",marker="o",color='red')
axis[0].scatter(df["N"],df["SOR Iterations"],label="SOR",marker="o",color='blue')

axis[1].set_ylabel("Execution Time (s)")
jacobi_fitting=np.polyfit(df['N'],df['Jacobi Time (s)'],2)
#axis[1].plot(N_VALUES,jacobi_fitting[0],N_VALUES),label=r"$aN^2+bN+c$",color="red")
sor_fitting=np.polyfit(df['N'],df['SOR Time (s)'],1)
axis[1].plot(N_VALUES,np.polyval(sor_fitting,N_VALUES),label=r"$aN+b$",color="blue")

axis[1].scatter(df["N"],df["Jacobi Time (s)"],label="Jacobi",marker="o")
axis[1].scatter(df["N"],df["SOR Time (s)"],label="SOR",marker="o")
plt.tight_layout()

for ax in axis:
    ax.legend()
    ax.set_xlabel("N")
    ax.set_yscale("log")
    ax.set_xscale("log")


fig.savefig("results/ex07_comparison.png",dpi=300)