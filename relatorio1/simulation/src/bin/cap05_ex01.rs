use eletrocomp::{
    initial_conditions::*, methods::jacobi_method, miscellaneous::*, neighbor_averages::*,
};
use ndarray::ArrayView2;

fn main() {
    let n = 100;
    let quadrado_interno = 0.2;
    let tolerance = 1e-5;
    let (initial_condition, fixed_points) = create_initial_condition_fig5_4(n, quadrado_interno);
    let (result, _) = jacobi_method(
        initial_condition.view(),
        fixed_points.view(),
        simple_neighbor_average,
        tolerance,
    );
    save_array(&result, "ex01_potential.npy");
}
