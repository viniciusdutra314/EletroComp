use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let mut n = 100_000;
    let mut rng=rand::rng();
    let mut file_results=File::create("results.dat").unwrap();
    while n > 0 {
        writeln!(file_results,"{}",n);
        for _ in 1..=n {
            if rng.random_bool(0.1){
                n-=1;
            }
        }
    }
}
