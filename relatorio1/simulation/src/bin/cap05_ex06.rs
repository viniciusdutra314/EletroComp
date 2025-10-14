use core::f64;

use eletrocomp::{
    initial_conditions::*, methods::{jacobi_method, over_relaxation}, miscellaneous::*, update_functions::*
};
use ndarray::s;

fn main() {
    let n = 500;
    let rod_length = 0.5;
    let tolerance = 1e-5;
    let (initial_potential, fixed_points) = create_eletric_rod(n, rod_length);
    let (result, _) = over_relaxation(
        initial_potential.view(),
        fixed_points.view(),
        None,
        ex02_neighbor_average,
        tolerance,
        (2.0/(1.0+f64::consts::PI/(n as f64))));
    save_array(&result, "ex06_potential.npy");
}
