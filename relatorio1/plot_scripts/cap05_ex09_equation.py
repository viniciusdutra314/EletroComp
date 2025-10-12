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
pprint(sympy_result)


tolentinos_eq = sp.Eq(
    V(r),
    sp.Rational(1, 2)
    * ((rho / epsilon_0) + V(r + 1) * (1 + 1 / r) + V(r - 1) * (1 - 1 / r)),
)
print("Tolentino's Result for V(r):")
sp.pprint(tolentinos_eq)

sp.pprint(f"{tolentinos_eq.equals(sympy_result)=}")

codegen(
    name_expr=[("update_V", sympy_result.rhs)],
    language="f95",
    project="PoissonSolver",
    to_files=True,
    header=False,
)
