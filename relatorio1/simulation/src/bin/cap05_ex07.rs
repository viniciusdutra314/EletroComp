use eletrocomp::{initial_conditions::*, methods::*, miscellaneous::*, neighbor_averages::*};
use ndarray::s;
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::time::Instant;

fn main() -> std::io::Result<()> {
    let plate_separation = 0.2;
    let plate_potential = 1.0;
    let plate_length = 0.3;
    let tolerance = 1e-9;

    let file = File::create("results/ex07_comparison.csv")?;
    let mut writer = BufWriter::new(file);
    writeln!(
        writer,
        "N,Jacobi Iterations,SOR Iterations,Jacobi Time (s),SOR Time (s),Speedup"
    )?;

    println!(
        "{:<5} | {:<17} | {:<16} | {:<15} | {:<15} | {:<10}",
        "N", "Jacobi Iterations", "SOR Iterations", "Jacobi Time (s)", "SOR Time (s)", "Speedup"
    );
    println!(
        "{:-<5} | {:-<17} | {:-<16} | {:-<15} | {:-<15} | {:-<10}",
        "", "", "", "", "", ""
    );

    for n in (50..500).step_by(50) {
        let alpha_factor = 2.0 / (1.0 + (PI / (n as f64)));
        let (initial_potential, fixed_points) =
            create_two_capacitors(n, plate_separation, plate_length, plate_potential);
        let initial_potential_view = initial_potential.slice(s![n / 2..n, n / 2..n]);
        let fixed_points_view = fixed_points.slice(s![n / 2..n, n / 2..n]);

        let start_jacobi = Instant::now();
        let (_jacobi_result, jacobi_iterations) = jacobi_method(
            initial_potential_view,
            fixed_points_view,
            None,
            ex03_neighbor_average,
            tolerance,
        );
        let jacobi_duration = start_jacobi.elapsed();

        let start_sor = Instant::now();
        let (_relaxation_result, relaxation_iterations) = over_relaxation(
            initial_potential_view,
            fixed_points_view,
            None,
            ex03_neighbor_average,
            tolerance,
            alpha_factor,
        );
        let sor_duration = start_sor.elapsed();

        let speedup = jacobi_duration.as_secs_f64() / sor_duration.as_secs_f64();

        println!(
            "{:<5} | {:<17} | {:<16} | {:<15.3} | {:<15.3} | {:<10.2}",
            n,
            jacobi_iterations,
            relaxation_iterations,
            jacobi_duration.as_secs_f64(),
            sor_duration.as_secs_f64(),
            speedup
        );
        writeln!(
            writer,
            "{},{},{},{:.6},{:.6},{:.2}",
            n,
            jacobi_iterations,
            relaxation_iterations,
            jacobi_duration.as_secs_f64(),
            sor_duration.as_secs_f64(),
            speedup
        )?;
    }
    Ok(())
}
