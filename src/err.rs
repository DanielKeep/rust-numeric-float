use std::error::Error;
use std::fmt;
use std::num::ParseFloatError;

/**
An error which can be returned when parsing a numeric float (*e.g.* `n32f`, `n64p`).
*/
#[derive(Clone, Debug, PartialEq)]
pub enum ParseNumericError {
    /// Parsing resulted in "Not-a-Number".
    NotANumber,

    /// Parsing failed for some other reason.
    FloatError(ParseFloatError),
}

impl fmt::Display for ParseNumericError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ParseNumericError::NotANumber => "not a number".fmt(fmt),
            ParseNumericError::FloatError(ref err) => err.fmt(fmt)
        }
    }
}

impl Error for ParseNumericError {
    fn description(&self) -> &str {
        "invalid real literal"
    }

    fn cause(&self) -> Option<&Error> {
        match *self {
            ParseNumericError::NotANumber => None,
            ParseNumericError::FloatError(ref err) => Some(err)
        }
    }
}
