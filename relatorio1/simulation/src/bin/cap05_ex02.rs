use eletrocomp::{
    neighbor_averages::*, initial_conditions::*, methods::jacobi_method, miscellaneous::*,
};
use ndarray::s;

fn main() {
    let n = 100;
    let quadrado_interno = 0.2;
    let tolerance = 1e-5;
    let (initial_potential, fixed_points) = create_initial_condition_fig5_4(n, quadrado_interno);
    let (result, _) = jacobi_method(
        initial_potential.slice(s![n / 2..n, n / 2..n]),
        fixed_points.slice(s![n / 2..n, n / 2..n]),
        None,
        ex02_neighbor_average,
        tolerance,
    );
    save_array(&result, "ex02_potential.npy");
}
