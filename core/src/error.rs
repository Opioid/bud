#[derive(Debug)]
pub struct Error {
    message: String,
}

impl Error {
    pub fn new(message: &str) -> Error {
        Error { message: message.to_string() }
    }

    pub fn from_string(message: String) -> Error {
        Error { message }
    }

    pub fn message(&self) -> &str {
        &self.message[..]
    }
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Error {
        Error { message: "std::io::Error".to_string() }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        Error {
            message: format!("{:?} (line {}, column {})", err.classify(), err.line(), err.column()),
        }
    }
}
