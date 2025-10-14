use eletrocomp::{
    initial_conditions::*, methods::jacobi_method, miscellaneous::*, update_functions::*,
};

fn main() {
    let n = 500;
    let quadrado_interno = 0.1;
    let tolerance = 1e-5;
    let (initial_condition, fixed_points) = create_initial_condition_fig5_4(n, quadrado_interno);
    let (result, _) = jacobi_method(
        initial_condition.view(),
        fixed_points.view(),
        None,
        simple_neighbor_average_2d,
        tolerance,
    );
    save_array(&result, "ex01_potential_small.npy");
    let quadrado_interno = 0.5; 
    let (initial_condition, fixed_points) = create_initial_condition_fig5_4(n, quadrado_interno);
    let (result, _) = jacobi_method(
        initial_condition.view(),
        fixed_points.view(),
        None,
        simple_neighbor_average_2d,
        tolerance,
    );
    save_array(&result, "ex01_potential_big.npy");
}
