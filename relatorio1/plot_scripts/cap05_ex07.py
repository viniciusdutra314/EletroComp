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
jacobi_fitting=np.polyfit(df['N'],df['Jacobi-It'],2)
axis[0].plot(N_VALUES,np.polyval(jacobi_fitting,N_VALUES),color="red",linestyle='--')
gauss_fitting=np.polyfit(df['N'],df['Gauss-It'],2)
axis[0].plot(N_VALUES,np.polyval(gauss_fitting,N_VALUES),color="green",linestyle='--')
sor_fitting=np.polyfit(df['N'],df['SOR-It'],1)
axis[0].plot(N_VALUES,np.polyval(sor_fitting,N_VALUES),color="blue",linestyle='--')
axis[0].scatter(df["N"],df["Jacobi-It"],label="Jacobi",marker="o",color='red')
axis[0].scatter(df["N"],df["Gauss-It"],label="Gauss",marker="o",color='green')
axis[0].scatter(df["N"],df["SOR-It"],label="SOR",marker="o",color='blue')


axis[1].set_ylabel("Execution Time (s)")
axis[1].set_xlabel("N")
axis[1].plot(df["N"],df["Jacobi-Time"],marker="o",color='red')
axis[1].plot(df["N"],df["Gauss-Time"],marker="o",color='green')
axis[1].plot(df["N"],df["SOR-Time"],marker="o",color='blue')
axis[1].set_yscale("log")

for ax in axis:
    ax.legend()


fig.savefig("results/ex07_comparison.png",dpi=300)