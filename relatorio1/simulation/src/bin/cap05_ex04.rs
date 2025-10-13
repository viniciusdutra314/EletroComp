use eletrocomp::{neighbor_averages::*, initial_conditions::*, methods::*, miscellaneous::*};
use ndarray::{s,linspace};

fn main() {
    for plate_separation in linspace(0.5, 0.05, 4) {
        let n = 2000;
        let plate_potential = 1.0;
        let plate_length = 0.3;
        let tolerance = 1e-10;
        let (initial_potential, fixed_points) =
            create_two_capacitors(n, plate_separation, plate_length, plate_potential);
        let (result, _) = over_relaxation(
            initial_potential.slice(s![n / 2..n, n / 2..n]),
            fixed_points.slice(s![n / 2..n, n / 2..n]),
            None,
            ex03_neighbor_average,
            tolerance,
            2.0/(1.0 + std::f64::consts::PI/n as f64)
        );
        save_array(
            &result,
            &format!("ex04_potential_{}.npy", plate_separation),
        );
    }
}
