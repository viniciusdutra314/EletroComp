using LinearAlgebra
using Plots
using Base.Threads
using StaticArrays  

const μ0 = 4π * 1e-7

function riemman_sum(f::Function,a::T,b::T,n::Integer) where T<:Real
    Δx = (b - a) / n
    x_values = range(a, stop=b - Δx, length=n)
    return sum(f, x_values) * Δx
end

function magnetic_field(r::SVector{3, T}, current::T, path_func::Function, a::T, b::T, n::Integer)::SVector{3, T} where T<:Real
    integrand(γ::T)::SVector{3, T} = begin
        h = (b - a) / n
        ℓ = path_func(γ)
        dℓ_dγ = (path_func(γ + h) - path_func(γ)) / h
        r_prime = r - ℓ
        return current * cross(dℓ_dγ, r_prime) / (norm(r_prime)^3)
    end
    return (μ0 / (4π)) * riemman_sum(integrand, a, b, n)
end

const I = 1.0
wire_path(γ) = SVector(0.0, γ, 0.0)

distances = range(0.1, stop=5.0, length=500)
a_limit = -1e3
b_limit = 1e3
n_steps = 100_000

B_values = Vector{SVector{3, Float64}}(undef, length(distances))

@threads for i in eachindex(distances)
    r_observe = SVector(distances[i], 0.0, 0.0)
    B_values[i] = magnetic_field(r_observe, I, wire_path, a_limit, b_limit, n_steps)
end


plot(distances, norm.(B_values), 
     label="Calculated |B|", 
     xlabel="Distance (m)", 
     ylabel="|B| (T)",
     title="Magnetic Field vs Distance from Infinite Wire",
     linewidth=2)

savefig("magnetic_field_vs_distance.png")