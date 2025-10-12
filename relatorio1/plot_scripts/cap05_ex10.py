import pandas as pd
import matplotlib.pyplot as plt
import numpy as np
plt.style.use("ggplot")
fig,axis=plt.subplots(figsize=(10,5))
df=pd.read_csv("results/ex10_alpha.csv")
axis.set_xlabel(r"Relaxation Factor ($\alpha$)")
axis.set_ylabel("Number of Iterations")
axis.scatter(df["Alpha"],df["Iterations2D"],label="2D",marker="o")
axis.scatter(df["Alpha"],df["Iterations3D"],label="3D",marker="o")
axis.legend()
plt.tight_layout()
fig.savefig("results/ex10_alpha.png",dpi=300)