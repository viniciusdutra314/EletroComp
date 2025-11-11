include("magnetic_field.jl")
using Plots

const f=1
const n=1000
const r=1
const θf=20.0
const n_voltas=floor(Int64,θf*f)
curve(θ) = (r*cos(2π*θ*f),r*sin(2π*θ*f),θ)
fig=plot3d(curve.(range(0,θf,n)),label="Solenoide n_voltas=$(n_voltas)")
r=SVector(curve(θf/2)...)

const I=1.0
B_expected=(μ0*n_voltas*I)/θf
#B=magnetic_field(r,I,curve,0.0,θf,n)
savefig(fig,"teste.png")