use rand::prelude::*;

type Prime = u128;

fn random_prime() -> u128 {
    let primes: [Prime; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let mut rng = thread_rng();
    let index = rng.gen_range(0..10);
    return primes[index];
}

fn main() {
    let P  = random_prime(); 
    let Q = random_prime();

    println!("{}", example);
}