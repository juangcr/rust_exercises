fn main() {
    let car = working_items_per_minute(6);
    println!("{}", car);
}

pub fn production_rate_per_hour(speed: u8) -> f64 {
    match speed {
        1|2|3|4 => f64::from(speed) * 221.0,
        5|6|7|8 => f64::from(speed) * 221.0 * 0.9,
        9|10 => f64::from(speed) * 221.0 *0.77,
        _ => 0.0,
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    let cars: f64 = production_rate_per_hour(speed);
    (cars / 60.0).trunc() as u32
}

