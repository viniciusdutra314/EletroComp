use eletrocomp::{boundary_conditions::*,definitions::*,methods::*,initial_conditions::*};
use ndarray::s;

fn main() {
    let n=200;
    let plate_separation=0.2;
    let plate_potential=1.0;
    let plate_length=0.3;
    let tolerance=1e-5;
    let initial_condition=create_two_capacitors(n, plate_separation,plate_length,plate_potential);
    let smaller_initial_condition=EletricPotential { potential_array: 
        initial_condition.potential_array.slice(s![n/2..n, n/2..n]).to_owned(),
        fixed_points: initial_condition.fixed_points.slice(s![n/2..n, n/2..n]).to_owned()};
    let (result,_) = jacobi_method(smaller_initial_condition,ex03_boundary_condition, tolerance);
    save_array(&result, "ex03_potential.npy");

}
