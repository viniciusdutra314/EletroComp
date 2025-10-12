use eletrocomp::{neighbor_averages::*, initial_conditions::*, methods::*, miscellaneous::*};
use ndarray::s;

fn main() {
    let n = 500;
    let plate_separation = 0.2;
    let plate_potential = 1.0;
    let plate_length = 0.3;
    let tolerance = 1e-5;
    let (initial_potential, fixed_points) =
        create_two_capacitors(n, plate_separation, plate_length, plate_potential);
    let (result, _) = jacobi_method(
        initial_potential.slice(s![n / 2..n, n / 2..n]),
        fixed_points.slice(s![n / 2..n, n / 2..n]),
        None,
        ex03_neighbor_average,
        tolerance,
    );
    save_array(&result, "ex03_potential.npy");
}
