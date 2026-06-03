use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

fn is_prime(n: u64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n.is_multiple_of(2) {
        return false;
    }
    let mut i = 3;
    while i * i <= n {
        if n.is_multiple_of(i) {
            return false;
        }
        i += 2;
    }

    true
}

fn main() {
    let numbers = (2..1_000_000).collect::<Vec<_>>();

    let prime_seq = numbers
        .iter()
        .filter(|&&n| is_prime(n))
        .copied()
        .collect::<Vec<_>>();

    let mut primes_par = numbers
        .par_iter()
        .filter(|&&n| is_prime(n))
        .copied()
        .collect::<Vec<_>>();

    primes_par.sort();

    assert_eq!(prime_seq, primes_par);
}
