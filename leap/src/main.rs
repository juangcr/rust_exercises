fn main() {
    println!("{}", is_leap_year(2000));
}


pub fn is_leap_year(year: u64) -> bool {
    let divisible: [bool; 3] = [4, 100, 400].iter()
        .map(|x| year % x == 0)
        .collect::<Vec<bool>>()
        .try_into()
        .unwrap();
    match divisible {
        [true, _ , true] => true,
        [true, false, false] => true,
        [true, true, false] => false,
        _ => false
    }
}
