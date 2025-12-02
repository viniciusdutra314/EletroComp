include("magnetic_field.jl")
using CairoMakie
using StaticArrays
using LinearAlgebra

const R = 1.0
const I_current = 1.0
const μ₀ = 4π * 1e-7
const n_integration = 10000

coil1(γ) = SVector(R * cos(γ), R * sin(γ), -R/2)
coil2(γ) = SVector(R * cos(γ), R * sin(γ), R/2)

const γ_start = 0.0
const γ_end = 2π

function analytical_B_z(z)
    d = R/2
    term1 = 1 / (R^2 + (z - d)^2)^(3/2)
    term2 = 1 / (R^2 + (z + d)^2)^(3/2)
    return (μ₀ * I_current * R^2 / 2) * (term1 + term2)
end

z_range = range(-2R, 2R, length=200)
x_range = range(-2R, 2R, length=200)

B_z_sim = Float64[]
B_z_exact = Float64[]

for z in z_range
    r = SVector(0.0, 0.0, z)
    B1 = magnetic_field(r, I_current, coil1, γ_start, γ_end, n_integration, simpsons_integration)
    B2 = magnetic_field(r, I_current, coil2, γ_start, γ_end, n_integration, simpsons_integration)
    B_total = B1 + B2
    push!(B_z_sim, norm(B_total))
    push!(B_z_exact, analytical_B_z(z))
end

B_x_sim = Float64[]
for x in x_range
    r = SVector(x, 0.0, 0.0)
    B1 = magnetic_field(r, I_current, coil1, γ_start, γ_end, n_integration, simpsons_integration)
    B2 = magnetic_field(r, I_current, coil2, γ_start, γ_end, n_integration, simpsons_integration)
    B_total = B1 + B2
    push!(B_x_sim, norm(B_total))
end

fig = Figure(size=(1000, 500))

ax1 = Axis(fig[1, 1], xlabel="z/R", ylabel="B (μT)", title="Campo Magnético no Eixo Z")
lines!(ax1, z_range, 1e6*B_z_sim, label="Simulação", color=:blue, linewidth=2)
lines!(ax1, z_range, 1e6*B_z_exact, label="Analítico", color=:red, linestyle=:dash, linewidth=2)
axislegend(ax1)

ax2 = Axis(fig[1, 2], xlabel="x/R", ylabel="B (μT)", title="Campo Magnético no Eixo X")
lines!(ax2, x_range, 1e6*B_x_sim, label="Simulação", color=:green, linewidth=2)
axislegend(ax2)

mkpath(joinpath(@__DIR__, "..", "plots"))
save(joinpath(@__DIR__, "..", "plots", "ex15_helmholtz.png"), fig)
