use eletrocomp::{initial_conditions::{self, *}, methods::*, miscellaneous::*, update_functions::*};
use ndarray::{s, Array1, Ix3};

fn main() {
    let n = 5000;
    let charge_size=5;
    let tolerance = 1e-10;
    let mut initial_potential= Array1::from_elem(n, 0.0);
    let mut fixed_points = Array1::from_elem(n, false);
    fixed_points[n-1]=true;
    let mut charge_density = Array1::from_elem(n, 0.0);
    charge_density.slice_mut(s![0..charge_size]).fill(1.0);

    let (result, _) = over_relaxation_spherical_coordinates(
        initial_potential.view(),
        fixed_points.view(),
        charge_density.view(),
        tolerance,
        1.9,
    );
    save_array(&result, "ex09_potential.npy");
}
