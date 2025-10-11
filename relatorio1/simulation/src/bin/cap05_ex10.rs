use eletrocomp::{
    initial_conditions::*, methods::{jacobi_method, over_relaxation}, miscellaneous::*, neighbor_averages::*,
};
use ndarray::{Ix2,Array2};

fn main() {
    let n = 500;
    let tolerance = 1e-5;
    let (initial_condition, fixed_points) = create_hypercube::<f64,Ix2>(n);
    let mut charge_density=Array2::<f64>::zeros((n,n));
    charge_density[(n/2,n/2)]=100.0;
    let (result, _) = over_relaxation(
        initial_condition.view(),
        fixed_points.view(),
        Some(charge_density.view()),
        simple_neighbor_average,
        tolerance,
        2.0/(1.0+(std::f64::consts::PI/(n as f64))),
    );
    save_array(&result, "ex10_potential.npy");
}
