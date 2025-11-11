function integrate(f::Function,a::Real,b::Real,n::Integer)::Real
    Δx=(b-a)/n
    x_values = range(a, stop = b-Δx, length = n)
    return sum(f, x_values) * Δx
end 

f(x)=sqrt(1-(x*x))
for exponent in 1:9
    local n=10^exponent
    local π_approx = 4*integrate(f,0.0,1.0,n)
    local error=abs(π - π_approx)
    println("n=$n: π ≈ $(π_approx),error=$(error)")
end  