use contracts::requires;
use rand::prelude::*;

/// Performs one stochastic decay step over `n` particles.
///
/// Each of the `n` particles independently decays with probability `p`. The
/// function returns how many particles remain after this step
///
/// Arguments:
/// - `n`: Initial number of particles 
/// - `p`: Per-particle decay probability 
/// - `rng`: Random number generator used to sample decays.
///
/// Returns the remaining number of particles after one step.
///
/// Panics
/// - If the preconditions are violated (checked via the `contracts` crate).
#[requires(n>0,"n must be non-negative")]
#[requires(0.0<=p || p<=1.0,"p must be a probability")]
#[ensures(ret>=0 && ret<=n,"Result needs to be not negative and smaller or equal to the initial n")]
pub fn decay_step(n: i64, p: f64, rng: &mut ThreadRng) -> i64 {
    let mut n_new = n;
    for _ in 1..=n {
        if rng.random_bool(p) {
            n_new -= 1;
        }
    }
    return n_new;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay_is_decreasing() {
        let mut rng = rand::rng();
        let mut n = 100;
        while n > 0 {
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
