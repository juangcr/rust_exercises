fn main() {
    let result = factors(8);
    println!("{:?}", result);
}

pub fn factors(n: u64) -> Vec<u64> {
    let binding = div_by(n);
    let roots: Vec<u64> = binding.iter()
        .filter(|x| is_prime(**x))
        .map(|x| *x)
        .collect();
    let mut prime_factors: Vec<u64> = Vec::new();
    let mut num = n;
    
    for r in roots.iter() {
        let mut residue = 0;
        while residue == 0 {
            num /= r;
            residue = num % r;
            prime_factors.push(*r);
        }
    }
    prime_factors
}

fn div_by(n: u64) -> Vec<u64> {
    let sq = (n as f64).sqrt().ceil() as u64;
    (2..=sq as u64).filter(|x| n % x == 0).collect()
}

fn is_prime(n: u64) -> bool {
    match n {
        2 => true,
        _ => div_by(n).len() == 0
    }
}
