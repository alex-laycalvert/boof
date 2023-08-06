const ERROR_USAGE: i32 = 64;

pub struct Error {
    pub code: i32,
    pub message: String,
}

impl Error {
    pub fn usage() -> Self {
        Error {
            code: ERROR_USAGE,
            message: "Usage: boof [script]".to_string(),
        }
    }

    pub fn lexing(message: String, line: usize) -> Self {
        Error {
            code: 1,
            message: format!("[line: {line}] Error: {message}"),
        }
    }

    pub fn parsing(message: String) -> Self {
        Error {
            code: 1,
            message: format!("Error: {message}"),
        }
    }
}
