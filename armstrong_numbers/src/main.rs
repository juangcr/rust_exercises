fn main() {
    println!("{}", is_armstrong_number(1234567890));
}

pub fn is_armstrong_number(num: u32) -> bool {
    let char_vec: Vec<char> = num.to_string().chars().collect();
    let power: u32 = char_vec.len().try_into().unwrap();
    let result: u128 = char_vec.iter()
        .map(|x| *x)
        .map(|x| x.to_digit(10))
        .map(|x| u128::pow(x.unwrap().into(), power))
        .sum();
    u128::from(num)== result
}
