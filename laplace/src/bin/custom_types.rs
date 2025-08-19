
struct Symmetrical2DMatrix<T>{
    data: Vec<T>,
    dimension:usize
}

impl<T:num_traits::Num + Copy> Symmetrical2DMatrix<T> {
    pub fn new(size:usize) -> Self {
        let data = vec![T::zero(); (size * (size+1))/2];
        return Self{data:data,dimension:size};
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

    pub fn get(&self, i:usize,j:usize) -> T{
        if i>=self.dimension || j>=self.dimension{
            panic!("Out of bounds error, ({},{}) {}x{}",i,j,self.dimension,self.dimension);
        }
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
    pub fn set(&mut self,i:usize,j:usize,value:T){
        if i>=self.dimension || j>=self.dimension{
            panic!("Out of bounds error, ({},{}) {}x{}",i,j,self.dimension,self.dimension);
        }
        
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
        let matrix=Symmetrical2DMatrix::<i32>::new(2);
        assert_eq!(matrix.get(0, 0),0);
        assert_eq!(matrix.get(0, 1),0);
        assert_eq!(matrix.get(1, 0),0);
        assert_eq!(matrix.get(1, 1),0);
    }
    #[test]
    fn test_set(){
        let mut matrix=Symmetrical2DMatrix::<f64>::new(3);
        for i in 0..3{
            for j in 0..3{
                matrix.set(i, j, (i*j) as f64);
            }
        }
        for i in 0..3{
            for j in 0..3{
                assert_eq!(matrix.get(i, j),(i*j) as f64);
            }
        }
    }

    #[test]
    fn test_multiple_invalid_indices() {
        let matrix = Symmetrical2DMatrix::<i8>::new(10);
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
        let mut matrix=Symmetrical2DMatrix::<i128>::new(10);
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
