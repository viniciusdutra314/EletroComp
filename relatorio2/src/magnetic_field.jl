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

function magnetic_field(
    r::SVector{3, T}, 
    I::T, 
    path_func::Function, 
    a::T, 
    b::T, 
    n::Integer)::SVector{3, T} where T<:Real

    integrand(γ::T)::SVector{3, T} = begin
        h = (b - a) / n
        ℓ = path_func(γ)
        dℓ_dγ = (path_func(γ + h) - path_func(γ)) / h
        r_prime = r - ℓ
        return I * (dℓ_dγ × r_prime) / (norm(r_prime)^3)
    end
    return (μ0 / (4π)) * riemman_sum(integrand, a, b, n)
end

