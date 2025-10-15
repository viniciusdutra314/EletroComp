use eletrocomp::{initial_conditions::*, methods::*, miscellaneous::*, update_functions::*};
use ndarray::{s};
use std::f64::consts::PI;
use std::fs::File;
use std::io::{BufWriter, Write};
use num_traits::Float;
use std::fmt::{Display, Debug};
use std::iter::Sum;
use ndarray::LinalgScalar;
use f128::f128;

fn run_simulation_for_precision<
    T: Float + Display + Debug + LinalgScalar + Sum,
>() -> std::io::Result<()> {
    let filename = format!("results/ex05_comparison_{}.csv", std::any::type_name::<T>().split("::").last().unwrap());
    let file = File::create(filename)?;
    let mut writer = BufWriter::new(file);
    writeln!(writer, "Tolerance,Jacobi-It,Gauss-It,Sor-It,Jacobi-Err,Gauss-Err,Sor-Err")?;

    let n=300;
    
    let plate_separation_f128 = f128::from(0.2);
    let plate_potential_f128 = f128::from(1.0);
    let plate_length_f128 = f128::from(0.3);
    let alpha_f128 = f128::from(2.0) / (f128::from(1.0) + (f128::from(PI) / f128::from(n)));

    let (initial_potential_f128, fixed_points_f128) =
        create_two_capacitors(n, plate_separation_f128, plate_length_f128, plate_potential_f128);
    let initial_potential_view_f128 = initial_potential_f128.slice(s![n / 2..n, n / 2..n]);
    let fixed_points_view_f128 = fixed_points_f128.slice(s![n / 2..n, n/2..n]);
    let (perfect_result_f128, _) = poisson_solver(
        Method::OverRelaxation { alpha_factor: alpha_f128 },
        initial_potential_view_f128,
        fixed_points_view_f128,
        ex03_neighbor_average,
        f128::from(1e-30),
    );


    for &tol_f64 in &[1e-2, 1e-3, 1e-4, 1e-5, 1e-6, 1e-7, 1e-8,1e-9,1e-10,1e-11] {
        let tolerance = T::from(tol_f64).unwrap() ;
        let plate_separation = T::from(0.2).unwrap();
        let plate_potential = T::from(1.0).unwrap();
        let plate_length = T::from(0.3).unwrap();
        
        let pi = T::from(PI).unwrap();
        let n_t = T::from(n).unwrap();
        let alpha_factor = T::from(2.0).unwrap() / (T::one() + (pi / n_t));
        let (initial_potential, fixed_points) =
            create_two_capacitors(n, plate_separation, plate_length, plate_potential);

        let initial_potential_view = initial_potential.slice(s![n / 2..n, n / 2..n]);
        let fixed_points_view = fixed_points.slice(s![n / 2..n, n / 2..n]);

        let (_jacobi_result, jacobi_iterations) = poisson_solver(
            Method::Jacobi,
            initial_potential_view,
            fixed_points_view,
            ex03_neighbor_average,
            tolerance,
        );
        let jacobi_error = (&perfect_result_f128 - &_jacobi_result.mapv(|x| f128::from(T::to_f64(&x).unwrap()))).mapv(f128::abs).sum();

        let (_gauss_result, gauss_iterations) = poisson_solver(
            Method::Gauss,
            initial_potential_view,
            fixed_points_view,
            ex03_neighbor_average,
            tolerance,
        );
        let gauss_error= (&perfect_result_f128 - &_gauss_result.mapv(|x| f128::from(T::to_f64(&x).unwrap()))).mapv(f128::abs).sum();


        let (_relaxation_result, relaxation_iterations) = poisson_solver(
            Method::OverRelaxation { alpha_factor },
            initial_potential_view,
            fixed_points_view,
            ex03_neighbor_average,
            tolerance,
        );
        let relaxation_error = (&perfect_result_f128 - &_relaxation_result.mapv(|x| f128::from(T::to_f64(&x).unwrap()))).mapv(f128::abs).sum();

        writeln!(
            writer,
            "{},{},{},{},{},{},{}",
            tolerance,
            jacobi_iterations,
            gauss_iterations,
            relaxation_iterations,
            jacobi_error,
            gauss_error,
            relaxation_error,
        )?;
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    run_simulation_for_precision::<f32>();
    run_simulation_for_precision::<f64>();
    
    Ok(())
}
