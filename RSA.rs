use rand::prelude::*;

type Prime = u128;

fn random_prime() -> Prime{
    let primes [Prime, 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let mut rnd = thread_rng();
    let index = rnd.gen_range(0.0..10.0);
    primes[index as u64];
}

fn main() {
    let mut example  = random_prime(); 

    println!("{}" example);
}