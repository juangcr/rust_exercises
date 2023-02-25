fn main() {
    println!("{}", sing(10, 0));
}

pub fn verse(n: u32) -> String {
    match n {
        0 => String::from("No more bottles of beer on the wall, \
            no more bottles of beer.\n\
            Go to the store and buy some more, \
            99 bottles of beer on the wall.\n"),
        1 => format!("{} bottle of beer on the wall, {} bottle of beer.\n\
            Take it down and pass it around, \
            no more bottles of beer on the wall.\n", n,  n),
        2 => format!("{} bottles of beer on the wall, {} bottles of beer.\n\
            Take one down and pass it around, \
            1 bottle of beer on the wall.\n", n,  n),
        _ => format!("{} bottles of beer on the wall, {} bottles of beer.\n\
            Take one down and pass it around, \
            {} bottles of beer on the wall.\n", n, n, n-1)
    }
}

pub fn sing(start: u32, end: u32) -> String {
    let reps: Vec<u32> = if start < end {
        vec![0]
    } else if start == end {
        vec![start]
    } else { 
        (end..=start).collect()
    };
    reps.iter().rev().map(|x| verse(*x)).reduce(|x, y| x + "\n" + &y).unwrap()
}
