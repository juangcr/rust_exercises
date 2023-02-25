use time::PrimitiveDateTime as DateTime;
use time::macros::datetime;
use time::Duration;

fn main() {
    let t1 = datetime!(2000-01-01 0:00); 
    let t2 = after(t1);
    println!("{:?}", t2);
}

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start + Duration::seconds(1_000_000_000)
}
