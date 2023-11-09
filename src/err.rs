use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum Error {
    Null,

    Other,

    TryInto,

    RegexOption,

    FromStr,

    SelectEq,

    UnitRepeat,

    TryRepeat,

    Repeat,

    OutOfBound,

    One,

    OneMore,

    CountIf,

    Start,

    End,

    String,

    Bytes,

    Consume,

    SpanIndex,
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Null => write!(f, "Error::Null"),
            Error::Other => write!(f, "Error::Other"),

            Error::TryInto => write!(f, "In (`Map`): got error in `try_into`"),
            Error::RegexOption => write!(f, "In (`Option<I>`): need option value in ctor or regex"),
            Error::FromStr => write!(f, "In (`Map`): can not map str value to type"),
            Error::SelectEq => write!(f, "In (`Map`): not equal"),
            Error::UnitRepeat => write!(f, "In (`UnitRepeat`): need more data"),
            Error::TryRepeat => write!(f, "In (`TryRepeat`): need more data"),
            Error::Repeat => write!(f, "In (`Repeat`): need more data"),
            Error::OutOfBound => write!(f, "Got error: offset out of bound"),
            Error::One => write!(f, "In (`one`): need more data"),
            Error::OneMore => write!(f, "In (`one_more`): need more data"),
            Error::CountIf => write!(f, "In (`count_if`): need more data"),
            Error::Start => write!(f, "In (`start`): offset is not at the begining"),
            Error::End => write!(f, "In (`end`): offset is not at the ending"),
            Error::String => write!(f, "In (`string`): match failed"),
            Error::Bytes => write!(f, "In (`bytes`): match failed"),
            Error::Consume => write!(f, "In (`consume`): need more data"),
            Error::SpanIndex => write!(f, "Got error: invalid span index"),
        }
    }
}
