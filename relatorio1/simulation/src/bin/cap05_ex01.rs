use eletrocomp::{definitions::*, methods::jacobi_method, boundary_conditions::*, initial_conditions::*,};


fn main() {
    let n=100;
    let quadrado_interno=0.2;
    let tolerance=1e-5;
    let initial_condition=create_initial_condition_fig5_4(n, quadrado_interno);
    let (result,_) = jacobi_method(initial_condition,no_boundary_condition, tolerance);
    save_array(&result, "ex01_potential.npy");
}
