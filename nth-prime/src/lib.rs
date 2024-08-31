use bit_vec::BitVec;
use lazy_static::lazy_static;

const MAX_SIZE: u32 = 150_000;

lazy_static! {
    pub static ref PRIMES: Vec<u32> = SieveOfEratosthenes::new(MAX_SIZE).primes();
}

pub struct SieveOfEratosthenes {
    size: u32,
    pub data: BitVec,
}

impl SieveOfEratosthenes {
    pub fn new(size: u32) -> SieveOfEratosthenes {
        let mut data = BitVec::from_elem((size / 2 + 1) as usize, true);
        let max_factor = (size as f64).sqrt() as u32;

        for p in (3..=max_factor).step_by(2) {
            if data[p as usize / 2] {
                for i in (p * p..size).step_by(2 * p as usize) {
                    data.set(i as usize / 2, false);
                }
            }
        }

        Self { size, data }
    }

    pub fn primes(&self) -> Vec<u32> {
        let mut result = Vec::new();

        result.push(2);

        for i in 1..self.size / 2 {
            if self.data[i as usize] {
                let p = 2 * i + 1;
                if p >= self.size {
                    break;
                }
                result.push(p);
            }
        }

        result
    }
}

pub fn nth(n: u32) -> u32 {
    *PRIMES.get(n as usize).unwrap()
}
