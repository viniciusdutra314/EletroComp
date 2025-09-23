use core::f64;
use std::fs::{self};

use contracts::*;
use ndarray::parallel::prelude::*;
use ndarray::{
    Array2,
};
use ndarray_npy::write_npy;

#[requires(plate_height<n/2,"O tamanho das placas precisa ser menor que n/2")]
#[requires(plate_separation<n/2,"A separação das placas precisa ser menor que n/2")]
#[requires(tolerance>0.0,"Tolerância precisa ser um valor positivo não nulo")]
fn laplace_simulation(n: usize, plate_separation: usize, plate_height:usize,tolerance: f64) -> Array2<f64> {
    let mut v_old = Array2::<f64>::zeros((n, n));
    for height in (n/2 - plate_height)..=(n/2 + plate_height){
        v_old[(n/2+plate_separation,height)]=-1.0;
        v_old[(n/2-plate_separation,height)]=1.0;
    }
    
    let inside_plate = |x: usize, y: usize| {
        (x ==n/2+plate_separation || x==n/2-plate_separation) && (y >= n/2-plate_height && y<=n/2 +plate_height)
    };

    let mut v_new = v_old.clone();

    let mut old_delta_v = f64::MAX;

    loop {
        let new_delta_v: f64 = v_new
            .outer_iter_mut()
            .into_par_iter()
            .enumerate()
            .map(|(x, mut row)| {
                let mut partial_delta_v = 0.0;
                if x != 0 && x != n - 1 {
                    for y in 1..n - 1 {
                        if !inside_plate(x, y) {
                            row[y] = (v_old[(x + 1, y)]
                                + v_old[(x - 1, y)]
                                + v_old[(x, y + 1)]
                                + v_old[(x, y - 1)])
                                / 4.0;
                            partial_delta_v += (row[y] - v_old[(x, y)]).abs();
                        }
                    }
                }
                partial_delta_v
            })
            .sum();

        if (new_delta_v - old_delta_v).abs() < tolerance {
            break;
        }
        old_delta_v = new_delta_v;
        std::mem::swap(&mut v_old, &mut v_new);
    }

    return v_new;
}

fn main() {
    let n=500;
    let plate_separation=100;
    let plate_height=100;
    let tolerance=1e-5;
    let result = laplace_simulation(n,plate_separation,plate_height,tolerance);
    if let Err(error) = fs::create_dir_all("results") {
        panic!("{error}");
    }
    if let Err(error) = write_npy("results/ex04.npy", &result) {
        panic!("{error}");
    }
}
