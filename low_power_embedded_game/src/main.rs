fn main() {
    println!("Hello, world!");
}

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    if divisor == 0 {
        panic!("Division by zero.")
    }

    ( dividend / divisor, dividend % divisor )
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.step_by(2)
}


pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        let x: i16 = if self.0.is_positive() { self.0 } else { -self.0 };
        let y: i16 = if self.1.is_positive() { self.1 } else { -self.1 };
        x + y
    }
}
