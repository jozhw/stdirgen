use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub struct CreateError(pub String);

impl fmt::Display for CreateError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Create Error: {}", self.0)
    }
}

impl Error for CreateError {}
