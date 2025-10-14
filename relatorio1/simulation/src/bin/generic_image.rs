use core::error;
use std::f64::consts::PI;
use eletrocomp::{update_functions::*,miscellaneous::*,methods::*,initial_conditions::*};


fn main(){
    let (initial_condition,fixed_points)=generic_image("results/eifel.jpg");
    let error_tolerance=1e-6;
    let alpha_factor=2.0/(1.0+PI/initial_condition.dim().0 as f64);
    let (result,_)=over_relaxation(initial_condition.view(),fixed_points.view(),None, simple_neighbor_average_2d, error_tolerance, alpha_factor);
    save_array(&result, "generic_image.npy");
}