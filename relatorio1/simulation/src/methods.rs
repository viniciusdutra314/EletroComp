use crate::definitions::{*};
use ndarray::Array2;
use num_traits::Float;

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
