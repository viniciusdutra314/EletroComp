use rand::prelude::*;
use std::fs::File;
use std::io::prelude::*;
use std::panic;

fn main() {
    let mut n = 100_000;
    let mut rng = rand::rng();
    let mut file_results = File::create("results.dat").unwrap();
    while n > 0 {
        writeln!(file_results, "{}", n);
        n = decay_step(n, 0.01, &mut rng);
    }
}

fn decay_step(n: i64, p: f64, rng: &mut ThreadRng) -> i64 {
    debug_assert!(n >= 0, "n must be non-negative");
    debug_assert!(0.0<=p || p<=1.0,"p must be a probability");
    let mut n_new = n;
    for _ in 1..=n {
        if rng.random_bool(p) {
            n_new -= 1;
        }
    }
    debug_assert!(n_new >= 0, "Decay step resulted in negative n");
    debug_assert!(n_new <= n, "Decay step increased n: {} -> {}", n, n_new);
    return n_new;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay_is_decreasing() {
        let mut rng = rand::rng();
        let mut n = 100;
        while (n > 0) {
            let old_n = n;
            n = decay_step(n, 0.01, &mut rng);
            assert!(n <= old_n, "Decay step did not decrease value: {} -> {}", old_n, n);
        }
    }
    #[should_panic]
    #[test]
    fn test_negative_num_particles(){
        let mut rng=rand::rng();
        decay_step(-100, 1.0, &mut rng);
    }

    #[should_panic]
    #[test]
    fn test_negative_probability(){
        let mut rng=rand::rng();
        decay_step(100,-1.0, &mut rng);
    }

    #[should_panic]
    #[test]
    fn test_probability_larger_than_1(){
        let mut rng=rand::rng();
        decay_step(100,1.05, &mut rng);
    }
}
