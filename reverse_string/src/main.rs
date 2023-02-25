fn main() {
    let result = reverse("Hello.");
    println!("{}", result);
}


pub fn reverse(input: &str) -> String {
    input.chars().rev().collect()
}
