use clock::Clock;

pub fn main() {
    let clock1 = Clock::new(23,59);
    let clock2 = clock1.add_minutes(2);

    println!("clock1 - {}", clock1.to_string());
    println!("clock2 - {}", clock2.to_string());
}