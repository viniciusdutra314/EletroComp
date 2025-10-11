use crate::definitions::*;
use image::{ImageReader};
use ndarray::{s, Array2, Ix2};
use num_traits::Float;

pub fn create_initial_condition_fig5_4<T: Float>(
    n: usize,
    quadrado_interno: f64,
) -> EletricPotential<T,Ix2> {
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

pub fn create_two_capacitors<T: Float>(
    n: usize,
    plate_separation: T,
    plate_length: T,
    plate_potential: T,
) -> EletricPotential<T,Ix2> {
    let mut potential = Array2::<T>::from_elem((n, n), T::zero());
    let mut fixed_points = Array2::<bool>::from_elem((n, n), false);
    for i in 0..n {
        fixed_points[(i, 0)] = true;
        fixed_points[(i, n - 1)] = true;
        fixed_points[(0, i)] = true;
        fixed_points[(n - 1, i)] = true;
    }

    let plate_separation = (plate_separation * T::from(n).unwrap())
        .round()
        .to_usize()
        .unwrap();
    let plate_length = (plate_length * T::from(n).unwrap())
        .round()
        .to_usize()
        .unwrap();
    let plate_start = n / 2 - plate_length / 2;
    let plate_end = n / 2 + plate_length / 2;
    let mid = n / 2;
    for i in plate_start..plate_end {
        potential[(mid - plate_separation / 2, i)] = plate_potential;
        fixed_points[(mid - plate_separation / 2, i)] = true;

        potential[(mid + plate_separation / 2, i)] = -plate_potential;
        fixed_points[(mid + plate_separation / 2, i)] = true;
    }
    return EletricPotential {
        potential_array: potential,
        fixed_points,
    };
}
pub fn generic_image<T: Float>(path: &str) -> EletricPotential<T,Ix2> {
    let img = ImageReader::open(path)
        .unwrap()
        .decode()
        .unwrap()
        .to_luma8();
    let (width, height) = img.dimensions();
    let height = height as usize;
    let width = width as usize;
    let mut potential = Array2::<T>::from_elem((height, width), T::zero());
    let mut fixed_points = Array2::<bool>::from_elem((height, width), false);
    fixed_points.slice_mut(s![0, ..]).fill(true);
    fixed_points.slice_mut(s![height - 1, ..]).fill(true);
    fixed_points.slice_mut(s![.., 0]).fill(true);
    fixed_points.slice_mut(s![.., width - 1]).fill(true);
    for i in 0..height {
        for j in 0..width {
            let pixel = img.get_pixel(j as u32, i as u32)[0];
            if pixel<200 {  
                potential[(i, j)] = T::one();
                fixed_points[(i, j)] = true;
            }
        }
    }
    return EletricPotential {
        potential_array: potential,
        fixed_points,
    };
}
