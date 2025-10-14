from sympy import *
r, rho, epsilon_0 = symbols("r rho epsilon_0")
V = Function("V")

poisson_eq = Eq(diff(r * V(r), r, 2) / r, -rho / epsilon_0)
dv_dr2 = Derivative(V(r), r, 2).as_finite_difference([r - 1, r, r + 1])
dv_dr = Derivative(V(r), r).as_finite_difference([r + 1, r - 1])
lhs_discretized = poisson_eq.lhs.subs(
    [(Derivative(V(r), r, 2), dv_dr2), 
     (Derivative(V(r), r), dv_dr)]
)
discretized_eq = Eq(lhs_discretized, poisson_eq.rhs)
sympy_result = Eq(V(r), solve(discretized_eq, V(r))[0].simplify())

print("Sympy Result for V(r):")
pprint(sympy_result.expand())

