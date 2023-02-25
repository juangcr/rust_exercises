fn main() {
    let result = nth(10_000);
    println!("{}", result);
}


pub fn nth(n: u32) -> u32 {
    match n {
        0 => 2,
        _ => validate(n)
    }
}

fn validate(n: u32) -> u32 {
    let mut count: u32 = 0;
    for i in 3.. {
        count += is_prime(i);
        if count == n {
            return i
        }
    }
    return 0;
}

fn is_prime(n: u32) -> u32 {
    let sq = (n as f64).sqrt().ceil();
    let divisors = (2..=sq as u32).filter(|x| n % x == 0).collect::<Vec<u32>>().len();
    match divisors {
        0 => 1,
        _ => 0
    }
}
