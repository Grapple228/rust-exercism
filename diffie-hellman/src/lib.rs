use rand::Rng;

pub fn private_key(p: u64) -> u64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(2..p)
}

pub fn public_key(p: u64, g: u64, a: u64) -> u64 {
    mod_exp(g as u128, a, p)
}

pub fn secret(p: u64, b_pub: u64, a: u64) -> u64 {
    mod_exp(b_pub as u128, a, p)
}

fn mod_exp(base: u128, exp: u64, modulus: u64) -> u64 {
    let mut result = 1;
    let mut base = base;
    let mut exp = exp;
    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * base) % modulus as u128;
        }
        exp /= 2;
        base = (base * base) % modulus as u128;
    }
    result as u64
}
