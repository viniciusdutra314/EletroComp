include("magnetic_field.jl")
using CairoMakie
using StaticArrays
using LinearAlgebra

const R = 5
const r = 2
const N_turns = 200
const I_current = 1.0
const μ₀ = 4π * 1e-7

torus_coil(γ) = SVector(
    (R + r * cos(N_turns * γ)) * cos(γ),
    (R + r * cos(N_turns * γ)) * sin(γ),
    r * sin(N_turns * γ)
)

const γ_start = 0.0
const γ_end = 2π
const n_integration = 10_000

x_range = range(0, R + 2 * r, length=200)

B_x_sim = Float64[]
B_x_ampere = Float64[]
ϵ=0.065
for x in x_range
    if x != R - r && x != R + r
        r_obs = SVector(x, ϵ, ϵ)
        B = magnetic_field(r_obs, I_current, torus_coil, γ_start, γ_end, n_integration, simpsons_integration)
        push!(B_x_sim, norm(B))
        B_amp = ifelse((x > (R - r)) && (x < (R + r)), (μ₀ * N_turns * I_current) / (2π * x), 0.0)
        push!(B_x_ampere, B_amp)
    end
end

fig = Figure(size=(1000, 500))

ax1 = Axis(fig[1, 1], xlabel="Distância até centro (m)",
    ylabel="B (μT)",
    title="Campo Magnético Toroide (R=5m, r=2m)")
lines!(ax1, x_range, B_x_sim * 1e6, label="Simulação", color=:blue, linewidth=2)
lines!(ax1, x_range, B_x_ampere * 1e6, label="Lei de Ampère (Ideal)", color=:red, linestyle=:dash, linewidth=2)
axislegend(ax1)

mkpath(joinpath(@__DIR__, "..", "plots"))
save(joinpath(@__DIR__, "..", "plots", "ex16_torus.png"), fig)
