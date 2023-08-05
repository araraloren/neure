use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    Null,

    NeedOne,

    NeedOneMore,

    NeedMore,

    Chars,

    SubStr,

    Match,

    NotStart,

    ReachEnd,

    NotEnd,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Null => write!(f, "Error::Null"),
            Error::Chars => write!(f, "Error::Chars"),
            Error::NeedOne => write!(f, "Error::NeedOne"),
            Error::NeedOneMore => write!(f, "Error::NeedOneMore"),
            Error::NeedMore => write!(f, "Error::NeedMore"),
            Error::SubStr => write!(f, "Error::SubStr"),
            Error::ReachEnd => write!(f, "Error::ReachEnd"),
            Error::NotStart => write!(f, "Error::NotStart"),
            Error::NotEnd => write!(f, "Error::NotEnd"),
            Error::Match => write!(f, "Error::Match"),
        }
    }
}
