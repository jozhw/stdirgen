use crate::errors::split_error::SplitError;

pub enum SplitResult<'a> {
    Parts(&'a str, &'a str),
    Error(SplitError),
}
pub fn split_string(input_str: &str, delimiter: char) -> SplitResult<'_> {
    if let Some((first_part, second_part)) = input_str.split_once(delimiter) {
        SplitResult::Parts(first_part, second_part)
    } else {
        SplitResult::Error(SplitError::DelimiterNotFound(delimiter))
    }
}
