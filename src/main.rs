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

fn mod_inverse(a: u64, m: u64) -> u64{
let mut multiplicative_inverse: u64 = 1;
if gcd(a, m) == 1{
    loop{
        if (multiplicative_inverse*a)%m != 1{
            multiplicative_inverse += 1;
        }else{
            break;
        }
    }
}else{
    println!("Multiplicative inverse does not exist for E and theta of N");
}
multiplicative_inverse
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

    keys.d = mod_inverse(keys.e, keys.theta_of_n);

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

fn encrypt(input_bytes: Vec<u8>, keys: Keys) -> Vec<u64>{
    let mut ciphertext: Vec<u64> = vec![0,0,0,0];
    let mut input_bytes_64: Vec<u64> = vec![0,0,0,0];
    let mut count = 0;

    for i in input_bytes.clone(){
        input_bytes_64[count] = input_bytes[count] as u64;
        count += 1;
    }
    count = 0;

    println!("*BEFORE ENCRYPTION* Input bytes (u64): ");
    for c in input_bytes_64.clone(){
        print!("{} ", c);
    }


    for i in input_bytes_64{
        let _throwaway = i.checked_pow(keys.e as u32);
        match _throwaway{
            Some(_throwaway) => {
                println!("\n{} ^ {} % {} = {}", i, keys.e, keys.n, _throwaway as u64 % keys.n);
                ciphertext[count] = _throwaway as u64 % keys.n;
            }, //ciphertext[count] = _throwaway as u64 % keys.n,
            None => println!("Encryption error. Overflow likely.")
        }
        count += 1;
    }
    ciphertext
}

fn decrypt(ciphertext: Vec<u64>, keys: Keys) -> Vec<u64> {
    let mut plaintext: Vec<u64> = vec![0,0,0,0];
    let mut count = 0;

    println!("*BEFORE DECRYPTION* Ciphertext as bytes: ");
    for c in ciphertext.clone(){
        print!("{} ", c);
    }




    for i in ciphertext.clone(){
        let _throwaway = i.checked_pow(keys.d as u32);
        match _throwaway{
            Some(_throwaway) => {
                println!("\n{} ^ {} % {} = {}", ciphertext[count], keys.d, keys.n, _throwaway % keys.n);
                plaintext[count] = _throwaway % keys.n
            },
            None => println!("\nError in decryption. Overflow likely.")
        }
        count += 1;
    }
    plaintext
}

fn main(){
    let keys: Keys = gen_keys();
    let input: String = get_input();
    let input_bytes: Vec<u8> = input.into_bytes();

    print!("Input as bytes: ");
    for c in input_bytes.clone(){
        print!("{} ", c)
    }

    println!("\nP: {}\nQ: {}\nE: {}\nTheta of n: {}\nD: {}", keys.p, keys.q, keys.e, keys.theta_of_n, keys.d);
    if gcd(keys.e, keys.theta_of_n) == 1{
        println!("E and theta of N are coprime.");
    }

    let ciphertext_bytes: Vec<u64> = encrypt(input_bytes.clone(), keys.clone());
    let decrypted_bytes: Vec<u64> = decrypt(ciphertext_bytes.clone(), keys.clone());

    //Pass input.as_bytes() and keys to fn encrypt(), should pass back &[u8] of encrypted 'characters'
    
    //Pass encrypted &[u8] and keys to fn decrypt(), should pass back &[u8] identical to input.as_bytes()

   

    

    print!("\nCiphertext as bytes: ");
    for c in ciphertext_bytes{
        print!("{} ", c);
    }

    print!("\nDecrypted text as bytes: ");
    for c in decrypted_bytes{
        print!("{} ", c);
    }

    let reconverted_string: Result<String, std::string::FromUtf8Error> = String::from_utf8(input_bytes.clone());
    match reconverted_string{
        Ok(reconverted_string) => print!("\nReconverted string: {}", reconverted_string),
        Err(reconverted_string) => print!("How did we get here?")
    }


    
}