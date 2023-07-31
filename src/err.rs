use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Null,

    Chars,

    SubStr,

    ReachEnd,

    Match(String),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Null => write!(f, "{}", "Error::Null"),
            Error::Chars => write!(f, "{}", "Error::Chars"),
            Error::SubStr => write!(f, "{}", "Error::SubStr"),
            Error::ReachEnd => write!(f, "{}", "Error::ReachEnd"),
            Error::Match(msg) => write!(f, "Error::Match({})", msg),
        }
    }
}
