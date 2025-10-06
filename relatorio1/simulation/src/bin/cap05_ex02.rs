
use eletrocomp::laplace::{create_initial_condition_fig5_4, ex02_boundary_condition, jacobi_method, save_array, simulate_laplace, EletricPotential};
use ndarray::{Array2,s};


fn main() {
    let n=100;
    let quadrado_interno=0.2;
    let tolerance=1e-5;
    let initial_condition=create_initial_condition_fig5_4(n, quadrado_interno);
    let smaller_initial_condition=EletricPotential { potential_array: 
        initial_condition.potential_array.slice(s![n/2..n, n/2..n]).to_owned(),
        fixed_points: initial_condition.fixed_points.slice(s![n/2..n, n/2..n]).to_owned()};
    let result = simulate_laplace(smaller_initial_condition, jacobi_method,ex02_boundary_condition, tolerance);
    save_array(&result, "ex02_potential.npy");

}
