use crate::miscellaneous::{*};
use num_traits::{Float};
use ndarray::{Array, Array2, Dimension, IntoDimension, Ix2};

pub fn simple_neighbor_average<T:Float>(array:&Array2<T>, index:Ix2)->T 
{
    let (i,j)=(index[0],index[1]);
    let up=array[(i,j+1)];
    let down=array[(i,j-1)];
    let left=array[(i-1,j)];
    let right=array[(i+1,j)];
    return (up+down+left+right)/T::from(4.0).unwrap();
}

 pub fn ex02_neighbor_average<T: Float>(array: &Array2<T>,index:Ix2) -> T {
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

pub fn ex03_neighbor_average<T:Float>(array:&Array2<T>, index:Ix2)->T {
    let (i,j)=(index[0],index[1]);
    match (i,j) {
        (0,0)=>{
            let up=array[(i,j+1)];
            return up/T::from(2.0).unwrap();
        }
        (0,_)=>{
            let up=array[(i,j+1)];
            let down=array[(i,j-1)];
            return (up+down)/T::from(2.0).unwrap();
        }
        (_,0)=>{
            let up=array[(i,j+1)];
            let left=array[(i-1,j)];
            let right=array[(i+1,j)];
            return (up+up+left+right)/T::from(4.0).unwrap();
        }
        _=>simple_neighbor_average(array,index),
    }
} 