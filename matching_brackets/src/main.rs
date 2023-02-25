fn main() {
    let result: bool = brackets_are_balanced("");
    println!("{}", result);
}

// "\", [({]}), }{
pub fn brackets_are_balanced(string: &str) -> bool {
    if string.is_empty() { return false; }
    let brackets: [(char, char); 3] = [('[', ']'), ('(', ')'), ('{', '}')]; 
    let mut balanced: u32 = 0;
    for brk in brackets.iter() {
        let opened: String = string.chars().filter(|x| x == &brk.0).collect();
        let closed: String = string.chars().filter(|x| x == &brk.1).collect();
        if opened.len() == closed.len() { balanced += 1;}
    }
    match balanced {
        3 => true,
        _ => false
    }
}

