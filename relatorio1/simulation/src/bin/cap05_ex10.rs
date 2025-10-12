use eletrocomp::{
    initial_conditions::*,
    methods::{over_relaxation},
    neighbor_averages::*,
};
use ndarray::{Array2, Array3, Ix2, Ix3};
use std::fs::File;
use std::io::{BufWriter, Write};
fn main() -> std::io::Result<()> {
    let n = 100;
    let tolerance = 1e-6;
    let file = File::create("results/ex10_alpha.csv")?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "Alpha,Iterations2D,Iterations3D")?;
    for alpha in ndarray::linspace(1.8, 1.99, 100) {
        let (initial_condition, fixed_points) = create_hypercube::<f64, Ix2>(n);
        let mut charge_density = Array2::<f64>::zeros((n, n));
        charge_density[(n / 2, n / 2)] = 100.0;
        let (_, iterations_2d) = over_relaxation(
            initial_condition.view(),
            fixed_points.view(),
            Some(charge_density.view()),
            simple_neighbor_average_2d,
            tolerance,
            alpha,
        );
        let (initial_condition, fixed_points) = create_hypercube::<f64, Ix3>(n);
        let mut charge_density = Array3::<f64>::zeros((n, n,n));
        charge_density[(n / 2, n / 2,n/2)] = 100.0;
        let (_, iterations_3d) = over_relaxation(
            initial_condition.view(),
            fixed_points.view(),
            Some(charge_density.view()),
            simple_neighbor_average_3d,
            tolerance,
            alpha,
        );
        println!("Alpha: {:.4}, Iterations2D: {},Iterations3D: {}", alpha, iterations_2d,iterations_3d);
        writeln!(writer, "{},{},{}", alpha, iterations_2d,iterations_3d)?;
    }
    return Ok(());
}
