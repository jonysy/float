use core::fmt;

/// A list specifying general error categories.
#[derive(Debug)]
pub enum Error {
    /// A non-finite value was provided.
    NotFinite,
}

impl Error {
    fn as_str(&self) -> &'static str {
        use self::Error::*;

        match *self {
            NotFinite => "a non-finite value was provided",
        }
    }
}

impl fmt::Display for Error {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}