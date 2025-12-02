using LinearAlgebra
using Plots
using Base.Threads
using StaticArrays  

const μ0 = 4π * 1e-7

function simpsons_integration(f, a::Real, b::Real, n::Integer)
    if isodd(n)
        throw(ArgumentError("n must be even"))
    end
    Δx = (b - a) / n
    s_odd = sum(f(a + i*Δx) for i in 1:2:n-1)
    s_even = sum(f(a + i*Δx) for i in 2:2:n-2)
    return (Δx/ 3)* (f(a) + f(b) + 4s_odd + 2s_even)
end

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
    n::Integer,integration_method)::SVector{3, T} where T<:Real

    integrand(γ::T)::SVector{3, T} = begin
        h = (b - a) / n
        ℓ = path_func(γ)
        dℓ_dγ = (path_func(γ + h) - path_func(γ)) / h
        r_prime = r - ℓ
        return I * (dℓ_dγ × r_prime) / (norm(r_prime)^3)
    end
    
    return (μ0 / (4π)) * integration_method(integrand, a, b, n)
end

