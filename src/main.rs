use rand::prelude::*;
use std::io;

type Prime = u128;

struct Keys{
    private: u128,
    public: u128,
    n: u128,
    exponent: u128,
}

fn random_prime() -> Prime {
    let primes: [Prime; 10] = [2, 3, 5, 7, 11, 13, 17, 19, 23, 29];
    let mut rng: ThreadRng = thread_rng();
    let index: usize = rng.gen_range(0..10);
    return primes[index];
}

fn gen_keys() -> Keys{
    let mut keys = Keys{
        private: 0,
        public: 0,
        n: 0,
        exponent: 0,
    };
    let mut rng: ThreadRng = thread_rng();
    let p: u128  = random_prime(); 
    let q: u128 = random_prime();
    let n: u128 = p*q;
    let theta_of_n: u128 = (p-1)*(q-1);
    let mut e: u128;
    let k: u128 = rng.gen_range(0..100000000000000000);

    loop{
        e = rng.gen_range(1..1000);

        if (theta_of_n % e != 0) && (e > 1) && (e < theta_of_n){
            if n.checked_pow( e as u32).is_some(){
                break;
            }
        }else{
            continue;
        }
    }
    keys.public = n.checked_pow(e as u32).unwrap();
    keys.private = (k * theta_of_n + 1) / e;
    keys.n = n;
    keys.exponent = e;
    keys
}

fn get_plaintext() -> String {
    let mut input: String = String::new();
    println!("Please enter your text: ");
    io::stdin().read_line(&mut input).expect("Failed to read text");
    input
}

fn encrypt(plaintext: String, exponent: u128, n:u128) -> String {
    let mut ciphertext: String = String::new();

    for char in plaintext.chars(){
        ciphertext.push(((((char as u128).checked_pow(exponent as u32)).unwrap() % n) as u8 as char));
    }
    ciphertext
}

fn decrypt(ciphertext: String, private_key: u128, n: u128) -> String {
    let mut plaintext: String = String::new();

    for char in ciphertext.chars(){
        plaintext.push(((char as u128).checked_pow(private_key as u32).unwrap() % n)as u32 as char);
    }

    plaintext
}

fn main() {
    let keys: Keys = gen_keys();
    let plaintext: String = get_plaintext();

    println!("Input: {}", plaintext);

    let ciphertext = encrypt(plaintext, keys.exponent, keys.n);

    println!("Ciphertext: {}", ciphertext);

    let output = decrypt(ciphertext, keys.private, keys.n);

    println!("Decrypted text: {}", output);
}