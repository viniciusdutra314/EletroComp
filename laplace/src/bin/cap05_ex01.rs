use core::f64;
use std::fs::{self};

use contracts::*;
use ndarray::parallel::prelude::*;
use ndarray::{
    parallel::prelude::{IntoParallelIterator, ParallelIterator},
    Array2,
};
use ndarray_npy::write_npy;

#[requires(quadrado_interno>0.0 && quadrado_interno<1.0,"Quadrado precisa ser uma porcentagem")]
#[requires(tolerance>0.0,"Tolerância precisa ser um valor positivo não nulo")]
fn laplace_simulation(n: usize, quadrado_interno: f64, tolerance: f64) -> Array2<f64> {
    let lado_quadrado = ((n as f64) * quadrado_interno).round() as usize;
    let l_0 = n / 2 - lado_quadrado/2;
    let l_f = n / 2 + lado_quadrado/2;
    let mut v_old = Array2::<f64>::zeros((n, n));

    for i in l_0..=l_f {
        for j in l_0..=l_f{
            v_old[(i,j)]=1.0;
        };
    }

    let inside_square = |x: usize, y: usize| {
        (x >=l_0 && x <= l_f) && (y >= l_0 && y <= l_f)
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
                        if !inside_square(x, y) {
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
    let quadrado_interno=0.2;
    let tolerance=1e-5;
    let result = laplace_simulation(n, quadrado_interno, tolerance);
    if let Err(error) = fs::create_dir_all("results") {
        panic!("{error}");
    }
    if let Err(error) = write_npy("results/eletric_potential.npy", &result) {
        panic!("{error}");
    }
}
