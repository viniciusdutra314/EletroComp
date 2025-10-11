use ndarray::{Array,Dimension};
use ndarray_npy::{write_npy};
use num_traits::Float;
use std::fs;

#[derive(Clone,Debug)]
pub struct EletricPotential<T: Float,D:Dimension> {
    pub potential_array: Array<T,D>,
    pub fixed_points: Array<bool,D>,
}

pub fn save_array<T,D>(array: &Array<T,D>, filename: &str)
where
    T: Float + ndarray_npy::WritableElement,
    D: Dimension,
{
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    let result_dir = format!("{manifest_dir}/../results");
    if let Err(error) = fs::create_dir_all(&result_dir) {
        panic!("{error}");
    };
    let path = format!("{manifest_dir}/../results/{filename}");
    if let Err(error) = write_npy(path, array) {
        panic!("{error}");
    };
}
