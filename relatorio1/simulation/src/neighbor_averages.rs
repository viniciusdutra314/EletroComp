use crate::miscellaneous::*;
use ndarray::{Array, Array2, Array3, Dimension, IntoDimension, Ix2, Ix3};
use num_traits::Float;

pub fn simple_neighbor_average_2d<T: Float>(array: &Array2<T>, index: Ix2) -> T {
    let (i, j) = (index[0], index[1]);
    let up = array[(i, j + 1)];
    let down = array[(i, j - 1)];
    let left = array[(i - 1, j)];
    let right = array[(i + 1, j)];
    return (up + down + left + right) / T::from(4.0).unwrap();
}

pub fn simple_neighbor_average_3d<T: Float>(array: &Array3<T>, index: Ix3) -> T {
    let (i, j, k) = (index[0], index[1], index[2]);
    return (array[(i, j, k + 1)]
        + array[(i, j, k - 1)]
        + array[(i, j + 1, k)]
        + array[(i, j - 1, k)]
        + array[(i - 1, j, k)]
        + array[(i + 1, j, k)])
        / T::from(6.0).unwrap();
}

pub fn ex02_neighbor_average<T: Float>(array: &Array2<T>, index: Ix2) -> T {
    let (i, j) = (index[0], index[1]);
    match (i, j) {
        (0, 0) => {
            let right = array[(i + 1, j)];
            let up = array[(i, j + 1)];
            return (up + up + right + right) / T::from(4.0).unwrap();
        }
        (0, _) => {
            let right = array[(i + 1, j)];
            let up = array[(i, j + 1)];
            let down = array[(i, j - 1)];
            return (right + right + up + down) / T::from(4.0).unwrap();
        }
        (_, 0) => {
            let up = array[(i, j + 1)];
            let left = array[(i - 1, j)];
            let right = array[(i + 1, j)];
            return (up + up + left + right) / T::from(4.0).unwrap();
        }
        _ => {
            let up = array[(i, j + 1)];
            let down = array[(i, j - 1)];
            let left = array[(i - 1, j)];
            let right = array[(i + 1, j)];
            return (up + down + left + right) / T::from(4.0).unwrap();
        }
    }
}

pub fn ex03_neighbor_average<T: Float>(array: &Array2<T>, index: Ix2) -> T {
    let (i, j) = (index[0], index[1]);
    match (i, j) {
        (0, 0) => {
            let up = array[(i, j + 1)];
            return up / T::from(2.0).unwrap();
        }
        (0, _) => {
            let up = array[(i, j + 1)];
            let down = array[(i, j - 1)];
            return (up + down) / T::from(2.0).unwrap();
        }
        (_, 0) => {
            let up = array[(i, j + 1)];
            let left = array[(i - 1, j)];
            let right = array[(i + 1, j)];
            return (up + up + left + right) / T::from(4.0).unwrap();
        }
        _ => simple_neighbor_average_2d(array, index),
    }
}
