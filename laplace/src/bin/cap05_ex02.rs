use core::f64;
use std::fs::{self};
use laplace::my_matrix_type::SymmetricMatrix;
use contracts::*;
use ndarray_npy::write_npy;

#[requires(quadrado_interno>0.0 && quadrado_interno<1.0,"Quadrado precisa ser uma porcentagem")]
#[requires(tolerance>0.0,"Tolerância precisa ser um valor positivo não nulo")]
fn laplace_simulation(n: usize, quadrado_interno: f64, tolerance: f64) -> SymmetricMatrix<f64> {
    let lado_quadrado = ((n as f64) * quadrado_interno).round() as usize;
    let l_0 = n / 2 - lado_quadrado/2;
    let mut v_old = SymmetricMatrix::<f64>::new(n/2,0.0);

    for i in l_0..n/2 {
        v_old.set(i, l_0, 1.0);
        v_old.set(l_0, i, 1.0);
    }

    let on_square_border = |x: usize, y: usize| {
        ((x == l_0) && (y >= l_0)) || ((y == l_0) && (x >= l_0))
    };

    let mut v_new = v_old.clone();

    let mut old_delta_v = f64::MAX;

    loop {
        let mut new_delta_v=0.0;
        for i in 1..n/2-1{
            for j in 1..=i{
                if !on_square_border(i,j){
                    let update_v=(v_old.get(i+1,j)+v_old.get(i-1,j)+
                v_old.get(i, j+1)+v_old.get(i, j-1))/4.0;
                    v_new.set(i, j, update_v);
                    new_delta_v+=(v_old.get(i,j)-update_v).abs();
                }
            }
        }
        
        if (new_delta_v - old_delta_v).abs() < tolerance {
            break;
        }
        old_delta_v = new_delta_v;
        std::mem::swap(&mut v_old, &mut v_new);
    }

    return v_new;
}

fn main() {
    let n=100;
    let quadrado_interno=0.2;
    let tolerance=1e-5;
    let partial_result = laplace_simulation(n, quadrado_interno, tolerance);
    if let Err(error) = fs::create_dir_all("results") {
        panic!("{error}");
    }

    let mut total_result=ndarray::Array2::<f64>::zeros((n,n));
    for x in 0..partial_result.get_dimension(){
        for y in 0..partial_result.get_dimension(){
            total_result[(x,y)]=partial_result.get(x, y);
            total_result[(x,(n-1)-y)]=partial_result.get(x, y);
            total_result[((n-1)-x,y)]=partial_result.get(x, y);
            total_result[((n-1)-x,(n-1)-y)]=partial_result.get(x, y);
        }
    }



    if let Err(error) = write_npy("results/eletric_potential.npy", &total_result) {
        panic!("{error}");
    }
}
