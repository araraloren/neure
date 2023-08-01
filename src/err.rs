use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Null,

    NeedMore,

    Chars,

    SubStr,

    ReachEnd,

    Match,

    NotReachEnd,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Null => write!(f, "Error::Null"),
            Error::Chars => write!(f, "Error::Chars"),
            Error::SubStr => write!(f, "Error::SubStr"),
            Error::NeedMore => write!(f, "Error::NeedMore"),
            Error::ReachEnd => write!(f, "Error::ReachEnd"),
            Error::Match => write!(f, "Error::Match"),
            Error::NotReachEnd => write!(f, "Error::NotReachEnd"),
        }
    }
}
