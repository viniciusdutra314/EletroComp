use ndarray::{Array2, Dim, arr2};
use ndarray_npy::write_npy;

fn main() {
    let n: usize = 51;
    let lado_quadrado = (n * 10) / 100;
    let mut v = Array2::<f64>::zeros((n, n));

    for i in (n / 2 - lado_quadrado)..=(n / 2 + lado_quadrado) {
        v[(i, n / 2 + lado_quadrado)] = 1.0;
        v[(i, n / 2 - lado_quadrado)] = 1.0;

        v[(n / 2 - lado_quadrado, i)] = 1.0;
        v[(n / 2 + lado_quadrado, i)] = 1.0;
    }

    let dentro_do_quadrado = |x: usize| x != n / 2 - lado_quadrado && x != n / 2 + lado_quadrado;

    for _ in 1..10 {
        for i in 1..n - 1 {
            for j in 1..n - 1 {
                if dentro_do_quadrado(i) && dentro_do_quadrado(j) {
                    v[(i, j)] =
                        (v[(i + 1, j)] + v[(i - 1, j)] + v[(i, j + 1)] + v[(i, j - 1)]) / 4.0;
                }
            }
        }
    }
    let _ = ndarray_npy::write_npy("teste.npy", &v);
}
