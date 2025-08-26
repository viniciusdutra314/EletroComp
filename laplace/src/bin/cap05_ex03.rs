use contracts::*;
use core::f64;
use laplace::my_matrix_type::SymmetricMatrix;
use ndarray_npy::write_npy;
use std::path::Path;
use std::{
    fs::{self},
    usize,
};

fn save_image(quadrant_matrix: &SymmetricMatrix<f64>, file_path: &Path) {
    let n = quadrant_matrix.get_dim();
    let mut total_result = ndarray::Array2::<f64>::zeros((n, n));
    for x in 0..quadrant_matrix.get_dim() {
        for y in 0..quadrant_matrix.get_dim() {
            total_result[(x, y)] = quadrant_matrix.get(x, y);
            total_result[(x, (n - 1) - y)] = quadrant_matrix.get(x, y);
            total_result[((n - 1) - x, y)] = quadrant_matrix.get(x, y);
            total_result[((n - 1) - x, (n - 1) - y)] = quadrant_matrix.get(x, y);
        }
    }
    if let Err(error) = write_npy(file_path, &total_result) {
        panic!("{error}");
    }
}

#[requires(inner_square_length>0.0 && inner_square_length<1.0,
    "Quadrado precisa ser uma porcentagem")]
fn figure5_4(n: usize, inner_square_length: f64) -> SymmetricMatrix<f64> {
    let square_length = ((n as f64) * inner_square_length).round() as usize;
    let l_0 = n / 2 - square_length / 2;
    let l_f = n / 2 + square_length / 2;
    let mut results = SymmetricMatrix::<f64>::new(n, 0.0);
    for i in l_0..=l_f {
        for j in l_0..=l_f {
            results.set(i, j, 1.0);
        }
    }
    return results;
}

#[requires(tolerance>0.0,"Tolerância precisa ser um valor positivo não nulo")]
fn laplace_simulation(
    initial_condition: &SymmetricMatrix<f64>,
    tolerance: f64,
) -> SymmetricMatrix<f64> {
    let n = initial_condition.get_dim();
    let mut v_old = SymmetricMatrix::<f64>::new(n, fill_value);
    let mut v_new = v_old.clone();
    let mut old_delta_v = f64::MAX;

    loop {
        let mut new_delta_v = 0.0;
        for i in 1..n / 2 {
            for j in 1..n / 2 {
                if !(initial_condition.get(i, j) == 1.0) {
                    let right = if i == n / 2 - 1 {
                        v_old.get(i - 1, j)
                    } else {
                        v_old.get(i + 1, j)
                    };
                    let left = v_old.get(i - 1, j);
                    let down = v_old.get(i, j - 1);
                    let up = if j == n / 2 - 1 {
                        v_old.get(i, j - 1)
                    } else {
                        v_old.get(i, j + 1)
                    };
                    let update_v = (right + left + down + up) / 4.0;
                    v_new.set(i, j, update_v);
                    new_delta_v += (v_old.get(i, j) - update_v).abs();
                }
            }
        }

        if (new_delta_v - old_delta_v).abs() < tolerance {
            break;
        }
        old_delta_v = new_delta_v;
        std::mem::swap(&mut v_old, &mut v_new);
    }

    return v_new;
}

fn main() {
    let n = 500;
    let tolerance = 1e-5;
    let inner_square_length = 0.2;
    let initial_condition = figure5_4(n, inner_square_length);
    let partial_result = laplace_simulation(&initial_condition, tolerance);
    if let Err(error) = fs::create_dir_all("results") {
        panic!("{error}");
    };
    save_image(&partial_result, Path::new("results/eletric_potential.npy"));
}
