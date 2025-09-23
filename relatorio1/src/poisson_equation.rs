use ndarray::{Array2};
use num_traits::{Float};

pub fn laplace_2d_equation<T:Float>(v:&Array2<T>,x:usize,y:usize) -> T{
    let four = T::from(4.0).unwrap();
    return (v[(x+1,y)]+v[(x-1,y)] + v[(x,y+1)]+ v[(x,y-1)])/four;

}