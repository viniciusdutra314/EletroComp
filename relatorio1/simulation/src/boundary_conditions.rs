use crate::definitions::{*};
use num_traits::{float, Float};
use ndarray::Array2;

pub fn no_boundary_condition<T:Float>(array:&Array2<T>, i:usize, j:usize)->Neighbors<T> {
    return Neighbors {
        up: array[(i, j+1)],
        down: array[(i, j-1)],
        left: array[(i-1, j)],
        right: array[(i+1, j)],
    };
}

pub fn ex02_boundary_condition<T: Float>(array: &Array2<T>, i: usize, j: usize) -> Neighbors<T> {
    match (i, j) {
        (0, 0) => {
            let right = array[(i + 1, j)];
            let up = array[(i, j + 1)];
            Neighbors {
                up,
                down: up,
                left: right, 
                right,
            }
        }
        (0, _) => {
            let right = array[(i + 1, j)];
            Neighbors {
                up: array[(i, j + 1)],
                down: array[(i, j - 1)],
                left: right, 
                right,
            }
        }
        (_, 0) => {
            let up = array[(i, j + 1)];
            Neighbors {
                up,
                down: up, 
                left: array[(i - 1, j)],
                right: array[(i + 1, j)],
            }
        }
        _ => no_boundary_condition(array, i, j),
    }
}

pub fn ex03_boundary_condition<T:Float>(array:&Array2<T>, i:usize, j:usize)->Neighbors<T> {
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
}