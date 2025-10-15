use core::f64;

use eletrocomp::{
    initial_conditions::*, methods::*, miscellaneous::*, update_functions::*
};
use ndarray::s;

fn main() {
    let n = 500;
    let rod_length = 0.5;
    let tolerance = 1e-5;
    let (initial_potential, fixed_points) = create_eletric_rod(n, rod_length);
    let (result, _) = poisson_solver(
        Method::OverRelaxation { alpha_factor: (2.0/(1.0+f64::consts::PI/(n as f64))) },
        initial_potential.view(),
        fixed_points.view(),
        ex02_neighbor_average,
        tolerance);
    save_array(&result, "ex06_potential.npy");
}
