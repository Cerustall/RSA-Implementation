use rand::prelude::*;

type Prime = u128;

fn random_prime() -> Prime {
    let primes: [Prime; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let mut rng: ThreadRng = thread_rng();
    let index: usize = rng.gen_range(0..10);
    return primes[index];
}

fn main() {
    let mut rng: ThreadRng = thread_rng();
    let p: u128  = random_prime(); 
    let q: u128 = random_prime();
    let n: u128 = p*q;
    let theta_of_n: u128 = (p-1)*(q-1);
    let mut e: u128;
    let k: u128 = rng.gen_range(0..100000000000000000);
    let public_key: u128;
    let private_key: u128;

    loop{
        e = rng.gen_range(0..1000);

        if (theta_of_n % e != 0) && (e > 1) && (e < theta_of_n){
            if n.checked_pow( e as u32).is_some(){
                break;
            }
        }else{
            continue;
        }
    }

    public_key = n.checked_pow(e as u32).unwrap();
    private_key = (k * theta_of_n + 1) / e;

    println!("Public key: {}", public_key);
    println!("Private key: {}", private_key);
}