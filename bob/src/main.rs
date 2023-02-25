fn main() {
    let answer: &str = reply("Okay if like my  spacebar  quite a bit?   ");
    println!("{}", answer);

}

pub fn reply(message: &str) -> &str {
    let msg_status = (is_question(message), all_caps(message), silence(message));
    println!("{:?}", &msg_status);
    match msg_status {
        (true, true, false) => "Calm down, I know what I'm doing!",
        (true, false, _) => "Sure.",
        (false, true, false) => "Whoa, chill out!",
        (_, _, true) => "Fine. Be that way!",
        _ => "Whatever."
    }
}

fn is_question(message: &str) -> bool {
    let last_char = message.trim();
    if last_char.len() > 0 && last_char.ends_with("?") { true } else { false } 
}

fn all_caps(message: &str) -> bool {
    let caps: Vec<char> = message.chars().filter(|x| x.is_uppercase()).collect();
    let alpha: Vec<char> = message.chars().filter(|x| x.is_alphabetic()).collect();
    if caps.len() == alpha.len() && caps.len() > 0 {true} else {false} 
}

fn silence(message: &str) -> bool {
    let alpha: Vec<char> = message.chars().filter(|x| x.is_alphanumeric()).collect();
    if alpha.len() == 0 || message.is_empty() { true } else { false }
}
