use std::{mem::MaybeUninit, vec};
use contracts::*;


struct SymmetricMatrixUninit<T:num_traits::Num +Copy>{
    data: Box<[MaybeUninit<T>]>,
    dimension:usize
} 

impl<T:num_traits::Num +Copy> SymmetricMatrixUninit<T>{
    pub fn new(n:usize) ->Self {
        return Self{data:Box::new_uninit_slice(n*(n+1)/2),dimension:n};
    }
    pub unsafe fn set_uncheked(&mut self,i:usize,j:usize,value:T){
        if j>i {
            unsafe {
                let _= *self.data.get_unchecked_mut(j*(j+1)/2 +i).write(value);
            }
        }
        else {
            unsafe {
                let _= *self.data.get_unchecked_mut(i*(i+1)/2 +j).write(value);
            }
        }
    }
    #[requires(i<self.dimension && j<self.dimension,"The element should be inside the matrix")]
    pub fn set(&mut self,i:usize,j:usize,value:T){
        unsafe {
            self.set_uncheked(i, j, value);
        }
    }

    pub unsafe fn has_initialized(self) -> SymmetricMatrix<T>{
        unsafe {
            return SymmetricMatrix { data: (self.data.assume_init()), dimension: (self.dimension) };
        }
    }

}


struct SymmetricMatrix<T>{
    data: Box<[T]>,
    dimension:usize,
}

impl<T:num_traits::Num + Copy> SymmetricMatrix<T> {
    pub fn new(n:usize,fill_value:T) -> Self {
        return Self{data:vec![fill_value;n*(n+1)/2].into_boxed_slice(),dimension:n};
    }


    pub fn get_dimension(&self)->usize{
        return self.dimension;
    }

    pub unsafe fn get_unchecked(&self,i:usize,j:usize)->T{
        if j>i {
            unsafe {
                return *self.data.get_unchecked(j*(j+1)/2 +i);
            }
        }
        else {
            unsafe {
                return *self.data.get_unchecked(i*(i+1)/2 +j);
            }
        }
    }

    #[requires(i<self.dimension && j<self.dimension,"The element should be inside the matrix")]
    pub fn get(&self, i:usize,j:usize) -> T{
        unsafe{
            return self.get_unchecked(i, j);
        };
    }

    pub unsafe fn set_uncheked(&mut self,i:usize,j:usize,value:T){
        if j>i {
            unsafe {
                *self.data.get_unchecked_mut(j*(j+1)/2 +i)=value;
            }
        }
        else {
            unsafe {
                *self.data.get_unchecked_mut(i*(i+1)/2 +j)=value;
            }
        }

    }
    #[requires(i<self.dimension && j<self.dimension,"The element should be inside the matrix")]
    pub fn set(&mut self,i:usize,j:usize,value:T){
        unsafe {
            self.set_uncheked(i, j, value);
        }
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let matrix=SymmetricMatrix::<i32>::new(2,0);
        assert_eq!(matrix.get(0, 0),0);
        assert_eq!(matrix.get(0, 1),0);
        assert_eq!(matrix.get(1, 0),0);
        assert_eq!(matrix.get(1, 1),0);
    }
    #[test]
    fn test_set(){
        let mut uninit_matrix=SymmetricMatrixUninit::<f64>::new(3);
        for i in 0..3{
            for j in 0..3{
                uninit_matrix.set(i, j, (i*j) as f64);
            }
        }
        let init_matrix:SymmetricMatrix::<f64>=unsafe {uninit_matrix.has_initialized()};

        for i in 0..3{
            for j in 0..3{
                assert_eq!(init_matrix.get(i, j),(i*j) as f64);
            }
        }
    }

    #[test]
    fn test_multiple_invalid_indices() {
        let matrix = SymmetricMatrix::<i8>::new(10,0);
        for i in [10,11,12] {
            for j in [4,5,6]{
                let result_a = std::panic::catch_unwind(|| matrix.get(i, j));
                let result_b = std::panic::catch_unwind(|| matrix.get(j, i));
                assert!(result_a.is_err());
                assert!(result_b.is_err());

            }

        }
    }
    
    #[test]
    fn test_symmmetry(){
        let mut matrix=SymmetricMatrix::<i128>::new(10,0);
        for i in 0..matrix.get_dimension() {
            for j in 0..matrix.get_dimension(){
                unsafe {
                    let value=(i*j+j+i) as i128;
                    matrix.set_uncheked(i, j, value);
                    assert_eq!(matrix.get_unchecked(i, j),matrix.get_unchecked(j,i));
                    assert_eq!(matrix.get_unchecked(i, j),value);
                    assert_eq!(matrix.get_unchecked(j, i),value);
                }
            }
        }
    }

}
