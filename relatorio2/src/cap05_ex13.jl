include("magnetic_field.jl")

f(x)=sqrt(1-(x*x))
results = Vector{Tuple{Int,Float64,Float64}}()

for exponent in 1:9
    n = 10^exponent
    π_approx = 4 * simpsons_integration(f, 0.0, 1.0, n)
    abs_error = abs(pi - π_approx)
    log10_error = log10(abs_error)
    println("n=$n: π ≈ $(π_approx), log10(abs error)=$(log10_error)")
    push!(results, (log10(n), Float64(π_approx), Float64(log10_error)))
end

results_dir = joinpath(@__DIR__, "..", "results")
mkpath(results_dir)
csv_path = joinpath(results_dir, "ex13_simpson.csv")
open(csv_path, "w") do io
    for (n, pi_approx, log10_error) in results
        println(io, "$n,$pi_approx,$(trunc(log10_error, digits=2))")
    end
end
