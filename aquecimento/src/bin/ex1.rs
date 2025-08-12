use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::panic;

use aquecimento::decay_functions::*;

fn main() {
    let mut n = 100_000;
    let mut rng = rand::rng();
    let mut file_result = match File::create("results.dat"){
        Err(why) => panic!("File couldn't be created: {}",why),
        Ok(file) => file,
    };
    while n > 0 {
        writeln!(file_result, "{}", n);
        n = decay_step(n, 0.01, &mut rng);
    }
}

