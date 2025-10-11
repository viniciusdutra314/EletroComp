use ndarray::{indices_of, ArrayView,Array, Dimension, IntoDimension, NdIndex};
use num_traits::Float;
use contracts::{ensures, requires};


#[requires(error_tolerance > T::zero(),"error_tolerance must be positive")]
#[requires(initial_potential.shape() == fixed_points.shape(),"initial_potential and fixed_points must have the same shape")]
#[ensures(ret.0.shape() == initial_potential.shape(),"result array must have the same shape as initial_potential")]
pub fn jacobi_method<T,D,NeighborAvg>(
    initial_potential: ArrayView<T,D>,
    fixed_points: ArrayView<bool,D>,
    neighbor_average: NeighborAvg,
    error_tolerance: T,
) -> (Array<T, D>, usize)
where
    T: Float,
    D: Dimension +Copy,
    <D as Dimension>::Pattern: NdIndex<D>,
    D::Pattern: Copy,
    NeighborAvg: Fn(&Array<T, D>, D) -> T,
{
    let mut v_old = initial_potential.to_owned();
    let mut v_new = v_old.clone();

    let mut old_delta_v = T::infinity();
    for iterations in 1.. {
        let mut new_delta_v = T::zero();
        for index in indices_of(&v_old) {
            if !fixed_points[index] {
                let old_val = v_old[index];
                let new_val = neighbor_average(&v_old, index.into_dimension());
                v_new[index] =new_val;
                new_delta_v = new_delta_v + (new_val- old_val).abs();
            };
        }
        if (new_delta_v - old_delta_v).abs() < error_tolerance {
            return (v_new, iterations);
        }
        old_delta_v = new_delta_v;
        std::mem::swap(&mut v_old, &mut v_new);
    };
    unreachable!();
}


#[requires(error_tolerance > T::zero(),"error_tolerance must be positive")]
#[requires(initial_potential.shape() == fixed_points.shape(),"initial_potential and fixed_points must have the same shape")]
#[ensures(ret.0.shape() == initial_potential.shape(),"result array must have the same shape as initial_potential")]
#[requires(alpha_factor > T::one() && alpha_factor < T::from(2.0).unwrap(), "alpha_factor must be in (1,2)")]
pub fn over_relaxation<T,D,NeighborAvg>(
    initial_potential: ArrayView<T,D>,
    fixed_points: ArrayView<bool,D>,
    neighbor_avg:NeighborAvg,
    error_tolerance: T,
    alpha_factor:T,
) -> (Array<T,D>, usize)
where
    T: Float,
    D: Dimension +Copy,
    <D as Dimension>::Pattern: NdIndex<D>,
    D::Pattern: Copy,
    NeighborAvg: Fn(&Array<T,D>,D) -> T,
{
    let mut v = initial_potential.to_owned();
    let mut old_delta_v = T::infinity();
    for iterations in 1.. {
        let mut new_delta_v = T::zero();
        for index in indices_of(&v){
            if !fixed_points[index]{
                let delta_v=neighbor_avg(&v,index.into_dimension())-v[index];
                let v_new=alpha_factor*delta_v+v[index];
                new_delta_v = new_delta_v + (v_new - v[index]).abs();
                v[index]=v_new;
            }
        } 
         if (new_delta_v - old_delta_v).abs() < error_tolerance {
            return (v, iterations);
        }
        old_delta_v = new_delta_v;
    }
    unreachable!();
}
                