#[derive(Debug)]
pub enum LogLevel {
    Info,
    Important,
    Warning,
    Error,
    Fatal,
}

impl core::fmt::Display for LogLevel {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }

}

pub fn log(message: String, level: LogLevel) {
    let time = chrono::offset::Local::now();
    println!("[{}] [{}] - {}", time.format("%Y-%m-%d %H:%M:%S"), level.to_string().to_uppercase(), message);
}
