include("magnetic_field.jl")
using Colors
using Plots
using StaticArrays

const I = 1.0
const μ₀ = 4π * 1e-7
wire_path(γ) = SVector(0.0, γ, 0.0)
L = 1.0
B_wire(d) = (μ₀ * I*L) / (2π*d*sqrt(L^2+d^2))

fig = plot(
    xlabel="Grid size (log10(dz))",
    ylabel="log10(Error)",
    dpi=300,
)

r_observe = SVector(1, 0.0, 0.0)
ns = trunc.(Int64,logrange(10,1e9,20))
ns = [ifelse(iseven(n),n,n+1) for n in ns]

B_riemman = [norm(magnetic_field(r_observe, I, wire_path, -L, L, n, riemman_sum)) for n in ns]
B_simpsons = [norm(magnetic_field(r_observe, I, wire_path, -L, L, n, simpsons_integration)) for n in ns]
B_analytic = B_wire(norm(r_observe))
B_riemman_error = log10.(abs.(B_riemman .- B_analytic) ./ B_analytic) 
B_simpsons_error = log10.(abs.(B_simpsons .- B_analytic) ./ B_analytic)
dz = log10.(2L ./ ns) 

scatter!(fig, dz, B_riemman_error , color=:red, label="Soma de Riemman",alpha=0.5)
scatter!(fig, dz, B_simpsons_error, color=:blue, label="Método de Simpsons",alpha=0.5)


savefig(fig, "../plots/ex11_12.png")