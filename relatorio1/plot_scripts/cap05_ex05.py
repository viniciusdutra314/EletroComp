from cap05_common_plots import *
import matplotlib.pyplot as plt
import numpy as np
import pandas as pd

plt.style.use('ggplot')

df_f32=pd.read_csv('results/ex05_comparison_f32.csv')
df_f64=pd.read_csv('results/ex05_comparison_f64.csv')

tolerance_f32=-np.log10(df_f32['Tolerance'])
jacobi_it_f32=df_f32['Jacobi-It']
gauss_it_f32=df_f32['Gauss-It']
sor_it_f32=df_f32['Sor-It']
jacobi_err_f32=df_f32['Jacobi-Err']
gauss_err_f32=df_f32['Gauss-Err']
sor_err_f32=df_f32['Sor-Err']

tolerance_f64=-np.log10(df_f64['Tolerance'])
jacobi_it_f64=df_f64['Jacobi-It']
gauss_it_f64=df_f64['Gauss-It']
sor_it_f64=df_f64['Sor-It']
jacobi_err_f64=df_f64['Jacobi-Err']
gauss_err_f64=df_f64['Gauss-Err']
sor_err_f64=df_f64['Sor-Err']


fig,axis=plt.subplots(ncols=2,figsize=(12,6))

axis[0].set_xlabel(r'$p$ dígitos significativos')
axis[0].set_ylabel('Número de iterações')
axis[0].set_yscale('log')
linestyle=(0, (3, 1, 1, 1, 1, 1)) 

line_jacobi_f32, = axis[0].plot(tolerance_f32,jacobi_it_f32,label='Jacobi (f32)',marker='o')
line_gauss_f32, = axis[0].plot(tolerance_f32,gauss_it_f32,label='Gauss (f32)',marker='o')
line_sor_f32, = axis[0].plot(tolerance_f32,sor_it_f32,label='SOR (f32)',marker='o')

color_jacobi = line_jacobi_f32.get_color()
color_gauss  = line_gauss_f32.get_color()
color_sor    = line_sor_f32.get_color()

line_jacobi_f64, = axis[0].plot(tolerance_f64,jacobi_it_f64,label='Jacobi (f64)',linestyle=linestyle, color=color_jacobi,marker='^')
line_gauss_f64, = axis[0].plot(tolerance_f64,gauss_it_f64,label='Gauss (f64)', linestyle=linestyle, color=color_gauss,marker='^')
line_sor_f64, = axis[0].plot(tolerance_f64,sor_it_f64,label='SOR (f64)',linestyle=linestyle, color=color_sor,marker='^')


axis[1].set_xlabel(r'$p$ dígitos significativos')
axis[1].set_ylabel(r'$\sum |V_{ideal} - V|$')
axis[1].set_yscale('log')

axis[1].plot(tolerance_f32,jacobi_err_f32,label='Jacobi (f32)', color=color_jacobi,marker='o')
axis[1].plot(tolerance_f32,gauss_err_f32,label='Gauss (f32)', color=color_gauss,marker='o')
axis[1].plot(tolerance_f32,sor_err_f32,label='SOR (f32)', color=color_sor,marker='o')

axis[1].plot(tolerance_f64,jacobi_err_f64,label='Jacobi (f64)',linestyle=linestyle,marker='^', color=color_jacobi)
axis[1].plot(tolerance_f64,gauss_err_f64,label='Gauss (f64)',linestyle=linestyle, marker='^',color=color_gauss)
axis[1].plot(tolerance_f64,sor_err_f64,label='SOR (f64)',linestyle=linestyle, marker='^',color=color_sor)


handles, labels = axis[0].get_legend_handles_labels()
ordered_handles = [handles[idx] for idx in [0, 3, 1, 4, 2, 5]]
ordered_labels = [labels[idx] for idx in [0, 3, 1, 4, 2, 5]]

fig.legend(ordered_handles, ordered_labels, loc='upper center', ncol=3) 

fig.savefig('results/ex05_comparison.png',dpi=300)