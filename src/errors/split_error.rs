use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SplitError {
    DelimiterNotFound(char),
}

impl fmt::Display for SplitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SplitError::DelimiterNotFound(delimiter) => {
                write!(f, "Delimiter '{} not found", delimiter)
            }
        }
    }
}

impl Error for SplitError {}
