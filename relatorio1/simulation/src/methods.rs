use contracts::{ensures, requires};
use ndarray::{
    ArcArray, Array, ArrayView, ArrayView1, Dimension, IntoDimension, Ix1, NdIndex, indices_of,
};
use num_traits::Float;

pub enum Method<T: Float> {
    Jacobi,
    Gauss,
    OverRelaxation { alpha_factor: T },
}
#[requires(error_tolerance > T::zero(),"error_tolerance must be positive")]
#[requires(initial_potential.shape() == fixed_points.shape(),"initial_potential and fixed_points must have the same shape")]
#[ensures(ret.0.shape() == initial_potential.shape(),"result array must have the same shape as initial_potential")]
pub fn poisson_solver<T, D, UpdateFunc>(
    method: Method<T>,
    initial_potential: ArrayView<T, D>,
    fixed_points: ArrayView<bool, D>,
    update_function: UpdateFunc,
    error_tolerance: T,
) -> (Array<T, D>, usize)
where
    T: Float,
    D: Dimension + Copy,
    <D as Dimension>::Pattern: NdIndex<D>,
    D::Pattern: Copy,
    UpdateFunc: Fn(&Array<T, D>, D) -> T,
{
    let mut old_delta_v = T::infinity();
    let mut v = initial_potential.to_owned();
    match method {
        Method::Jacobi => {
            let mut v_new = v.clone();
            for iterations in 1.. {
                let mut new_delta_v = T::zero();
                for index in indices_of(&v) {
                    if !fixed_points[index] {
                        let old_val = v[index];
                        let new_val = update_function(&v, index.into_dimension());
                        v_new[index] = new_val;
                        new_delta_v = new_delta_v + (new_val - old_val).abs();
                    };
                }
                if (new_delta_v - old_delta_v).abs() < error_tolerance {
                    return (v_new, iterations);
                }
                old_delta_v = new_delta_v;
                std::mem::swap(&mut v, &mut v_new);
            }
        }
        Method::Gauss | Method::OverRelaxation { .. } => {
            let alpha_factor = match method {
                Method::OverRelaxation { alpha_factor } => alpha_factor,
                Method::Gauss => T::one(),
                _ => unreachable!(),
            };
            for iterations in 1.. {
                let mut new_delta_v = T::zero();
                for index in indices_of(&v) {
                    if !fixed_points[index] {
                        let delta_v = update_function(&v, index.into_dimension()) - v[index];
                        let v_new = alpha_factor * delta_v + v[index];
                        new_delta_v = new_delta_v + (v_new - v[index]).abs();
                        v[index] = v_new;
                    }
                }
                if (new_delta_v - old_delta_v).abs() < error_tolerance {
                    return (v, iterations);
                }
                old_delta_v = new_delta_v;
            }
        }
    }
    unreachable!();
}
