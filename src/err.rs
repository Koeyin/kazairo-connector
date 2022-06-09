use std::process::exit;

pub enum Recoverable {
    Recoverable,
    Unrecoverable(i32),
}

pub struct Error {
    recoverable: Recoverable,
    message: String,
}

impl Error {
    pub fn new(recoverable: Recoverable, message: &str) -> Error {
        Error {
            recoverable,
            message: message.to_string(),
        }
    }

    pub fn solve(&self) {
        match self.recoverable {
            Recoverable::Recoverable => println!("Error: {}", self.message),
            Recoverable::Unrecoverable(code) => {
                println!("Panic: {}", self.message);
                exit(code);
            }
        }
    }
}
