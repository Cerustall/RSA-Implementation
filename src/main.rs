use rand::prelude::*;
use std::io;

#[derive(Clone)]
struct Keys{
    public: u64,
    private: u64,
    n: u64,
    p: u64,
    q: u64,
    e: u64,
    k: u64,
    d: u64,
    theta_of_n: u64
}

fn random_prime() -> u64{
    let primes: [u64; 10] = [2, 3, 5, 7, 11 ,13 ,17, 19, 23, 29];
    let mut rng = rand::thread_rng();
    let selector = rng.gen_range(0..=9);
    primes[selector]
}

fn lcm(a: u64, b: u64) -> u64{
    let max: u64;
    let mut lcm: u64;
    if a>b{
        max = a;
    }else{
        max = b;
    }
    lcm = max;
    while (lcm % a != 0) || (lcm % b != 0){
        lcm += max;
    }
    lcm
}

fn gcd(a: u64, b: u64) -> u64{
    let mut min: u64;
    if a<b{
        min = a;
    }else{
        min = b;
    }
    while min>0{
        if (a%min == 0) && (b%min == 0){
            break;
        }
        min -= 1;
    }
    min
}

fn mod_inverse(a: u64, m: u64, keys: Keys) -> u64{
    let mut mod_inverse: u64 = 0;
    assert_eq!((keys.e*keys.d)%keys.theta_of_n, 0);
    mod_inverse = keys.d;
    mod_inverse
}

fn gen_keys() -> Keys{
    let mut rng = rand::thread_rng();
    let mut e_count = 2;
    let primes: [u64; 10] = [2, 3, 5, 7, 11 ,13 ,17, 19, 23, 29];
    let mut keys =  Keys{
        public: 0,
        private: 0,
        n: 0,
        p: 0,
        q: 0,
        e: 0,
        k: 0,
        d: 0,
        theta_of_n: 0
    };

    keys.p = random_prime();
    keys.q = random_prime();
    keys.n = keys.p*keys.q;
    keys.theta_of_n = lcm(keys.p-1, keys.q-1);
    keys.k = rng.gen_range(0..=100);

    keys.e = 3;
    loop{
        if gcd(keys.theta_of_n, keys.e) != 1{
            keys.e = primes[e_count];
            e_count += 1;
        }else{
            break;
        }
    }
    let _throwaway = keys.q.checked_pow(keys.e.try_into().unwrap());
    match _throwaway{
        Some(_throwaway) => keys.public = keys.p * _throwaway,
        None => println!("Exponent not accetable, triggers overflow on raising q to e. Exponent is {}", keys.e)
    }

    keys.d = mod_inverse(keys.e, keys.theta_of_n, keys.clone());

    keys.private = (keys.k * keys.theta_of_n + 1)/keys.e;
    keys
}

fn get_input() -> String{
    let mut input: String = String::new();
    println!("Please enter your text: ");
    input.clear();
    io::stdin().read_line(&mut input).unwrap();
    print!("Input: {}", input);
    input
}

fn main(){
    let keys: Keys = gen_keys();
    let input: String = get_input();
    let input_bytes: &[u8] = input.as_bytes();

    //Pass input.as_bytes() and keys to fn encrypt(), should pass back &[u8] of encrypted 'characters'
    
    //Pass encrypted &[u8] and keys to fn decrypt(), should pass back &[u8] identical to input.as_bytes()

    println!("P: {}\nQ: {}\nE: {}\nTheta of n: {}\nD: {}", keys.p, keys.q, keys.e, keys.theta_of_n, keys.d);
    if gcd(keys.e, keys.theta_of_n) == 1{
        println!("E and theta of N are coprime.");
    }

    print!("Input as bytes: ");
    for c in input_bytes{
        print!("{} ", c)
    }


}