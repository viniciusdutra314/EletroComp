use crate::definitions::{*};
use num_traits::{Float};
use ndarray::{Array, Array2, Dimension, IntoDimension, Ix2};

pub fn no_boundary_condition<T:Float>(array:&Array2<T>, index:Ix2)->T 
{
    let (i,j)=(index[0],index[1]);
    let up=array[(i,j+1)];
    let down=array[(i,j-1)];
    let left=array[(i-1,j)];
    let right=array[(i+1,j)];
    return (up+down+left+right)/T::from(4.0).unwrap();
}

 pub fn ex02_boundary_condition<T: Float>(array: &Array2<T>,index:Ix2) -> T {
    let (i, j) = (index[0], index[1]);
    match (i, j) {
        (0, 0) => {
            let right = array[(i + 1, j)];
            let up = array[(i, j + 1)];
            return (up+up+right+right)/T::from(4.0).unwrap();
        }
        (0, _) => {
            let right = array[(i + 1, j)];
            let up= array[(i, j + 1)];
            let down = array[(i, j - 1)];
            return (right+right + up + down)/T::from(4.0).unwrap();
            }
        (_, 0) => {
            let up = array[(i, j + 1)];
            let left = array[(i - 1, j)];
            let right = array[(i + 1, j)];
            return (up+up + left + right)/T::from(4.0).unwrap();
        }
        _ => {
            let up = array[(i, j + 1)];
            let down = array[(i, j - 1)];
            let left = array[(i - 1, j)];
            let right = array[(i + 1, j)];
            return (up + down + left + right)/T::from(4.0).unwrap();
        }
    }
}

/* pub fn ex03_boundary_condition<T:Float>(array:&Array2<T>, i:usize, j:usize)->Neighbors<T> {
    match (i,j) {
        (0,0)=>{
            let right=array[(i+1,j)];
            let up=array[(i,j+1)];
            Neighbors{
                up,
                down:up,
                left:-right, 
                right,
            }
        }
        (0,_)=>{
            let right=array[(i+1,j)];
            Neighbors{
                up:array[(i,j+1)],
                down:array[(i,j-1)],
                left:-right, 
                right,
            }
        }
        (_,0)=>{
            let up=array[(i,j+1)];
            Neighbors{
                up,
                down:up, 
                left:array[(i-1,j)],
                right:array[(i+1,j)],
            }
        }
        _=>no_boundary_condition(array,i,j),
    }
}  */