use eletrocomp::{
    update_functions::*, initial_conditions::*, methods::*, miscellaneous::*,
};
use ndarray::{s, Array2, Array3, Ix2, Ix3};

fn main() {
    let n = 200;
    let tolerance = 1e-5;
    let (mut potential, fixed_points) = create_hypercube::<f64,Ix2>(n);
    let mut rho = Array2::<f64>::zeros((n, n));
    rho[[n-10, n-10]] = 1.0;
    let update_func=|array: &Array2<f64>, idx: Ix2| {
        simple_neighbor_average_2d(array, idx) + rho[idx]
    };

    let (result, _) = poisson_solver(
        Method::OverRelaxation { alpha_factor: 2.0 / (1.0 + (std::f64::consts::PI / (n as f64))) },
        potential.view(),
        fixed_points.view(),
        update_func,
        tolerance,
    );

    save_array(&result, "ex08_potential.npy");
}
