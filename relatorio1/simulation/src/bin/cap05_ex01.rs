use eletrocomp::definitions::{*};


fn main() {
    let n=100;
    let quadrado_interno=0.2;
    let tolerance=1e-5;
    let initial_condition=create_initial_condition_fig5_4(n, quadrado_interno);
    let result = simulate_laplace(initial_condition, jacobi_method,no_boundary_condition, tolerance);
    save_array(&result, "ex01_potential.npy");
}
