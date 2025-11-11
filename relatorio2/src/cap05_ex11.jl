include("magnetic_field.jl")
using Colors

const I = 1.0
wire_path(γ) = SVector(0.0, γ, 0.0)

distances = range(0.1, stop=5.0, length=500)
B_values = Vector{SVector{3, Float64}}(undef, length(distances))
fig=plot(
     xlabel="Distance (m)", 
     ylabel="|B| (T)",
     title="Magnetic Field vs Distance from Infinite Wire",
     dpi=300
)
n_steps=10_000
L_values=range(start=0.1,stop=100.0,length=1000)
colors=range(start=colorant"blue",stop=colorant"red",length=length(L_values))
for (color,L) in zip(colors,L_values)
    @threads for i in eachindex(distances)
        r_observe = SVector(distances[i], 0.0, 0.0)
        B_values[i] = magnetic_field(r_observe, I, wire_path, -L, L, n_steps)
    end
    plot!(fig,distances,norm.(B_values),color=color,alpha=0.3,label=nothing)
end

savefig(fig,"finite_wire.png")