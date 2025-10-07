use crate::definitions::{*};
use ndarray::Array2;
use num_traits::Float;



pub fn jacobi_method<T,B>(
    initial_condition: EletricPotential<T>,
    boundary_condition:B,
    error_tolerance: T,
) -> (Array2<T>, usize)
where
    T: Float,
    B: Fn(&Array2<T>,usize,usize) -> Neighbors<T>,
{
    let potential = initial_condition.potential_array;
    let fixed_points = initial_condition.fixed_points;
    let (n, m) = (potential.shape()[0], potential.shape()[1]);
    let mut v_old = potential;
    let mut v_new = v_old.clone();

    let mut iterations = 0;
    let mut old_delta_v = T::infinity();
    loop {
        let mut new_delta_v = T::zero();
        for i in 0..n {
            for j in 0..m {
                if fixed_points[(i, j)] {
                    continue;
                } else {
                    let neighbors=boundary_condition(&v_old,i,j);
                    v_new[(i, j)] = (neighbors.up + neighbors.down + neighbors.left + neighbors.right) / T::from(4.0).unwrap();
                    new_delta_v = new_delta_v + (v_new[(i, j)] - v_old[(i, j)]).abs();
                }
            }
        }

        if (new_delta_v - old_delta_v).abs() < error_tolerance {
            break;
        }
        old_delta_v = new_delta_v;
        std::mem::swap(&mut v_old, &mut v_new);
        iterations += 1;
    }
    return (v_new, iterations);
}


pub fn over_relaxation<T,B>(
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
}
