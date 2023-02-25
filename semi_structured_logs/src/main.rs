fn main() {
    let x: String = log(LogLevel::Info, "info.");
    println!("{}", x);
}

pub enum LogLevel {
    Info,
    Warning,
    Error,
}
/// primary function for emitting logs
pub fn log(level: LogLevel, message: &str) -> String {
    match level {
        LogLevel::Info => info(message),
        LogLevel::Warning => warn(message),
        LogLevel::Error => error(message),
    }
}
pub fn info(message: &str) -> String {
    String::from("[INFO]: ") + message    
}
pub fn warn(message: &str) -> String {
    String::from("[WARNING]: ") + message    
}
pub fn error(message: &str) -> String {
    String::from("[ERROR]: ") + message
}
