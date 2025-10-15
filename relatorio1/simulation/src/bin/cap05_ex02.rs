use eletrocomp::{
    update_functions::*, initial_conditions::*, methods::*, miscellaneous::*,
};
use ndarray::s;

fn main() {
    let n = 500;
    let quadrado_interno = 0.2;
    let tolerance = 1e-5;
    let (initial_potential, fixed_points) = create_initial_condition_fig5_4(n, quadrado_interno);
    let (result, _) = poisson_solver(
        Method::Jacobi,
        initial_potential.slice(s![n / 2..n, n / 2..n]),
        fixed_points.slice(s![n / 2..n, n / 2..n]),
        ex02_neighbor_average,
        tolerance,
    );
    save_array(&result, "ex02_potential.npy");
}
