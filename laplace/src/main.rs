use core::f64;
use std::fs::{self};

use contracts::*;
use ndarray::{Array2};
use ndarray_npy::write_npy;


#[requires(quadrado_interno>0.0 && quadrado_interno<1.0,"Quadrado precisa ser uma porcentagem")]
#[requires(tolerance>0.0,"Tolerância precisa ser um valor positivo não nulo")]
fn laplace_simulation(n: usize, quadrado_interno: f64,tolerance:f64) -> Array2<f64> {
    let lado_quadrado = ((n as f64) * quadrado_interno).round() as usize;
    let l_0=n/2 -lado_quadrado;
    let l_f=n/2 + lado_quadrado;
    let mut v_old = Array2::<f64>::zeros((n, n));
    
    for i in l_0..=l_f {
        v_old[(i, l_0)] = 1.0;
        v_old[(i, l_f)] = 1.0;
        v_old[(l_0, i)] = 1.0;
        v_old[(l_f, i)] = 1.0;
    }

    let mut v_new=v_old.clone();

    let mut old_delta_v = f64::MAX;

    let on_square_border=|x:usize,y:usize| {((x==l_0 || x==l_f) && (y>l_0 && y<l_f)) || ((y==l_0 || y==l_f) && (x>l_0 && x<l_f))};

    loop {
        let mut new_delta_v = 0.0;
        for x in 1..n-1{
            for y in 1..n-1{
                if !on_square_border(x,y){
                    v_new[(x, y)] = (v_old[(x + 1, y)] + v_old[(x - 1, y)] + v_old[(x, y + 1)] + v_old[(x, y - 1)]) / 4.0;
                    new_delta_v += (v_new[(x, y)] - v_old[(x,y)]).abs();
                }
            }
        }
        if (new_delta_v - old_delta_v).abs() < tolerance {
            break;
        }
        old_delta_v=new_delta_v;
        std::mem::swap(&mut v_old, &mut v_new);
        }
        
    return v_new;
}

fn main() {
    let result = laplace_simulation(100, 0.1,1e-5);
    if let Err(error) = fs::create_dir_all("results"){
        panic!("{error}");
    }
    if let Err(error) = write_npy("results/eletric_potential.npy", &result) {
        panic!("{error}");
    }
}
