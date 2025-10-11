use crate::definitions::*;
use ndarray::{indices_of, Array, Dimension, IntoDimension, NdIndex};
use num_traits::Float;

pub fn jacobi_method<T, B, D>(
    initial_condition: EletricPotential<T, D>,
    boundary_condition: B,
    error_tolerance: T,
) -> (Array<T, D>, usize)
where
    T: Float,
    B: Fn(&Array<T, D>, D) -> T,
    D: Dimension +Copy,
    <D as Dimension>::Pattern: NdIndex<D>,
    D::Pattern: Copy,
{
    let potential = initial_condition.potential_array;
    let fixed_points = initial_condition.fixed_points;
    let mut v_old = potential;
    let mut v_new = v_old.clone();

    let mut old_delta_v = T::infinity();
    for iterations in 0.. {
        let mut new_delta_v = T::zero();
        for index in indices_of(&v_old) {
            if !fixed_points[index] {
                let old_val = v_old[index];
                let new_val = boundary_condition(&v_old, index.into_dimension());
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

/* pub fn over_relaxation<T,B>(
    initial_condition: EletricPotential<T>,
    boundary_condition:B,
    error_tolerance: T,
    alpha_factor:T,
) -> (Array2<T>, usize)
where
    T: Float,
    B: Fn(&Array2<T>,usize,usize) -> Neighbors<T>,
{
    let potential = initial_condition.potential_array;
    let fixed_points = initial_condition.fixed_points;
    let (n, m) = (potential.shape()[0], potential.shape()[1]);
    let mut v = potential;

    let mut iterations = 0;
    let mut old_delta_v = T::infinity();
    loop {
        let mut new_delta_v = T::zero();
        for i in 0..n {
            for j in 0..m {
                if fixed_points[(i, j)] {
                    continue;
                } else {
                    let neighbors=boundary_condition(&v,i,j);
                    let delta_v=(neighbors.up + neighbors.down + neighbors.left + neighbors.right) / T::from(4.0).unwrap()-v[(i,j)];
                    let v_new=alpha_factor*delta_v+v[(i,j)];
                    new_delta_v = new_delta_v + (v_new - v[(i,j)]).abs();
                    v[(i,j)]=v_new;
                }
            }
        }

        if (new_delta_v - old_delta_v).abs() < error_tolerance {
            break;
        }
        old_delta_v = new_delta_v;
        iterations += 1;
    }
    return (v, iterations);
} */
