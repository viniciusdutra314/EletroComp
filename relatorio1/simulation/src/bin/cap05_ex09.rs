use eletrocomp::{
    initial_conditions::{self, *},
    methods::*,
    miscellaneous::*,
    update_functions::*,
};
use ndarray::{Array1, Ix1, s};

fn main() {
    let n = 10000;
    let charge_size = 20;
    let tolerance = 1e-10;
    let mut initial_potential = Array1::from_elem(n, 1.0);
    let mut fixed_points = Array1::from_elem(n, false);
    fixed_points[n - 1] = true;
    initial_potential[n - 1] = 0.0;
    let mut charge_density = Array1::from_elem(n, 0.0);
    charge_density.slice_mut(s![0..charge_size]).fill(1.0);
    let update_function = |v: &Array1<f64>, idx: Ix1| {
        let r = idx[0];
        if r == 0 {
            return v[1] + charge_density[0] / 3.0;
        }
        else {
            return charge_density[idx] * 0.5
                + 0.5
                    * (v[r + 1] * (1.0 + 1.0 / (r as f64)) + v[r - 1] * (1.0 - 1.0 / (r as f64)));
        }
    };

    let (result, _) = poisson_solver(
        Method::OverRelaxation {
            alpha_factor: (2.0 / (1.0 + (std::f64::consts::PI / (n as f64)))),
        },
        initial_potential.view(),
        fixed_points.view(),
        update_function,
        tolerance,
    );
    save_array(&result, "ex09_potential.npy");
}
