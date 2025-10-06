use ndarray::Array2;
use ndarray_npy;
use num_traits::Float;
use std::fs;

#[derive(Clone,Debug)]
pub struct EletricPotential<T: Float> {
    pub potential_array: Array2<T>,
    pub fixed_points: Array2<bool>,
}

pub fn create_initial_condition_fig5_4<T: Float>(
    n: usize,
    quadrado_interno: f64,
) -> EletricPotential<T> {
    let lado_quadrado = ((n as f64) * quadrado_interno).round() as usize;
    let l_0 = n / 2 - lado_quadrado / 2;
    let l_f = n / 2 + lado_quadrado / 2;
    let mut potential = Array2::<T>::from_elem((n, n), T::zero());
    let mut fixed_points = Array2::<bool>::from_elem((n, n), false);
    for i in 0..n {
        fixed_points[(i, 0)] = true;
        fixed_points[(i, n - 1)] = true;
        fixed_points[(0, i)] = true;
        fixed_points[(n - 1, i)] = true;
    }
    for i in l_0..=l_f {
        for j in l_0..=l_f {
            potential[(i, j)] = T::one();
            fixed_points[(i, j)] = true;
        }
    }
    return EletricPotential {
        potential_array: potential,
        fixed_points,
    };
}

pub struct Neighbors<T: Float> {
    pub up: T,
    pub down: T,
    pub left: T,
    pub right: T,
}

pub fn no_boundary_condition<T:Float>(array:&Array2<T>, i:usize, j:usize)->Neighbors<T> {
    return Neighbors {
        up: array[(i, j+1)],
        down: array[(i, j-1)],
        left: array[(i-1, j)],
        right: array[(i+1, j)],
    };
}

pub fn ex02_boundary_condition<T: Float>(array: &Array2<T>, i: usize, j: usize) -> Neighbors<T> {
    match (i, j) {
        (0, 0) => {
            let right = array[(i + 1, j)];
            let up = array[(i, j + 1)];
            Neighbors {
                up,
                down: up,
                left: right, 
                right,
            }
        }
        (0, _) => {
            let right = array[(i + 1, j)];
            Neighbors {
                up: array[(i, j + 1)],
                down: array[(i, j - 1)],
                left: right, 
                right,
            }
        }
        (_, 0) => {
            let up = array[(i, j + 1)];
            Neighbors {
                up,
                down: up, 
                left: array[(i - 1, j)],
                right: array[(i + 1, j)],
            }
        }
        _ => no_boundary_condition(array, i, j),
    }
}

pub fn jacobi_method<T:Float>(neighbors:Neighbors<T>) -> T
{
    return (neighbors.up + neighbors.down + neighbors.left + neighbors.right) / T::from(4.0).unwrap();
}


pub fn simulate_laplace<T,M,B>(
    initial_condition: EletricPotential<T>,
    method: M,
    boundary_condition:B,
    error_tolerance: T,
) -> Array2<T>
where
    T: Float,
    M: Fn(Neighbors<T>) -> T,
    B: Fn(&Array2<T>,usize,usize) -> Neighbors<T>,
{
    let potential = initial_condition.potential_array;
    let fixed_points = initial_condition.fixed_points;
    let (n, m) = (potential.shape()[0], potential.shape()[1]);
    let mut v_old = potential;
    let mut v_new = v_old.clone();

    let mut old_delta_v = T::infinity();
    loop {
        let mut new_delta_v = T::zero();
        for i in 0..n {
            for j in 0..m {
                if fixed_points[(i, j)] {
                    continue;
                } else {
                    let neighbors=boundary_condition(&v_old,i,j);
                    v_new[(i, j)] = method(neighbors);
                    new_delta_v = new_delta_v + (v_new[(i, j)] - v_old[(i, j)]).abs();
                }
            }
        }

        if (new_delta_v - old_delta_v).abs() < error_tolerance {
            break;
        }
        old_delta_v = new_delta_v;
        std::mem::swap(&mut v_old, &mut v_new);
    }
    return v_new;
}

pub fn save_array<T>(array: &Array2<T>, filename: &str)
where
    T: Float + ndarray_npy::WritableElement,
{
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let result_dir = format!("{manifest_dir}/../results");
    if let Err(error) = fs::create_dir_all(&result_dir) {
        panic!("{error}");
    };
    let path = format!("{manifest_dir}/../results/{filename}");
    if let Err(error) = ndarray_npy::write_npy(path, array) {
        panic!("{error}");
    };
}
