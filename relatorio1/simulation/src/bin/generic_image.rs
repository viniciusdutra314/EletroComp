use core::error;
use std::f64::consts::PI;
use eletrocomp::{neighbor_averages::*,miscellaneous::*,methods::*,initial_conditions::*};


fn main(){
    let initial_condition=generic_image("/workspaces/EletroComp/relatorio1/simulation/eifel.jpg");
    let error_tolerance=1e-6;
    let alpha_factor=2.0/(1.0+PI/initial_condition.potential_array.dim().0 as f64);
    let (result,_)=over_relaxation(initial_condition, simple_neighbor_average, error_tolerance, alpha_factor);
    save_array(&result, "generic_image.npy");
}