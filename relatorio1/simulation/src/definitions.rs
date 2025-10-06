use ndarray::Array2;
use ndarray_npy;
use num_traits::Float;
use std::fs;

#[derive(Clone,Debug)]
pub struct EletricPotential<T: Float> {
    pub potential_array: Array2<T>,
    pub fixed_points: Array2<bool>,
}

pub struct Neighbors<T: Float> {
    pub up: T,
    pub down: T,
    pub left: T,
    pub right: T,
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
