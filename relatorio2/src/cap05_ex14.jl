include("magnetic_field.jl")
using CairoMakie
using StaticArrays
using LinearAlgebra
using GeometryBasics

const R = 1
const L = 3
const N_turns = 10
const I_current = 1.0
const μ₀ = 4π * 1e-7

solenoid(γ) = SVector(R * cos(γ), R * sin(γ), (L / (2π * N_turns)) * γ)

const γ_start = 0.0
const γ_end = 2π * N_turns
const n_integration = 10000

n_x = n_y = 5
n_z = 5
x_range = range(-1.5R, 1.5R, length=n_x)
y_range = range(-1.5R, 1.5R, length=n_y)
z_range = range(0, L, length=n_z)

points = Point3f[]
vectors = Vec3f[]
M = Float64[]

for z in z_range, y in y_range, x in x_range
    r_obs = SVector(x, y, z)
    B = magnetic_field(r_obs, I_current, solenoid, γ_start, γ_end, n_integration, simpsons_integration)
    B_norm = norm(B)
    push!(M, B_norm * 1e6)
    push!(points, Point3f(x, y, z))
    push!(vectors, Vec3f(B[1], B[2], B[3]) / B_norm)
end

fig = Figure(size=(800, 800),dpi=300)
ax = Axis3(fig[1, 1], xlabel="x (m)", ylabel="y (m)", zlabel="z (m)",
    aspect=:data, azimuth=0.7, elevation=0.3)

γ_plot = range(γ_start, γ_end, length=1000)
sol_points = solenoid.(γ_plot)
lines!(ax,
    getindex.(sol_points, 1),
    getindex.(sol_points, 2),
    getindex.(sol_points, 3),
    label="Solenoid", linewidth=2, color=:black, alpha=0.5)


arrows3d!(ax, points, vectors,
    lengthscale=0.5,             
    tiplength=0.1,               
    tipradius=0.04,              
    shaftradius=0.01,            
    color=M,
    colormap=:greens,
    align=:center,
    label="Magnetic Field",alpha=0.5
)

Colorbar(fig[1, 2], limits=extrema(M), colormap=:greens, label="B magnitude (μT)")

mkpath(joinpath(@__DIR__, "..", "plots"))
save(joinpath(@__DIR__, "..", "plots", "ex14_solenoid_3d.png"), fig)