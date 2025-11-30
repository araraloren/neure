use std::fmt::Display;

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Error {
    Null,

    Not,

    Consume,

    Slice,

    String,

    End,

    Start,

    LockMutex,

    Option,

    FromStr,

    TryInto,

    SelectEq,

    SepCollect,

    Collect,

    Separate,

    RegexRepeat,

    NeuRepeatRange,

    NeuRepeat,

    NeuOneMore,

    NeuOne,

    NeuThen,

    OriginOutOfBound,

    Vec,

    PairVec,

    PairSlice,

    Utf8Error,

    FromLeBytes,

    FromBeBytes,

    FromNeBytes,

    Other,

    Uid(usize),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Null => write!(f, "Error::Null"),
            Error::Not => write!(f, "In (`not`): got error when invoke regex"),
            Error::Consume => write!(f, "In (`consume`): need more data"),
            Error::Slice => write!(f, "In (`slice`): bytes not equal"),
            Error::String => write!(f, "In (`string`): string not equal"),
            Error::End => write!(f, "In (`end`): offset is not at the ending"),
            Error::Start => write!(f, "In (`start`): offset is not at the begining"),
            Error::LockMutex => write!(f, "Can not lock mutex for regex"),
            Error::Option => write!(f, "In (`Option`): unexcepted `None` value"),
            Error::FromStr => write!(f, "In (`FromStr`): got error in `from_str_radix`"),
            Error::TryInto => write!(f, "In (`MapTryInto`): got error in `TryInto::try_into`"),
            Error::SelectEq => write!(f, "In (`SelectEq`): tuple.0 and tuple.1 not equal"),
            Error::SepCollect => write!(f, "In (`SepCollect`): need more data"),
            Error::Collect => write!(f, "In (`Collect`): need more data"),
            Error::Separate => write!(f, "In (`Separate`): need more data"),
            Error::RegexRepeat => write!(f, "In (`RegexRepeat`): need more data"),
            Error::NeuRepeatRange => write!(f, "In (`NeuRepeatRange`): need more data"),
            Error::NeuRepeat => write!(f, "In (`NeuRepeat`): need more data"),
            Error::NeuOneMore => write!(f, "In (`NeuOneMore`): need more data"),
            Error::NeuOne => write!(f, "In (`NeuOne`): need more data"),
            Error::NeuThen => write!(f, "In (`NeuThen`): need more data"),
            Error::Vec => write!(f, "In (`Vec`): all match failed"),
            Error::PairVec => write!(f, "In (`Hash`): all match failed"),
            Error::PairSlice => todo!(),
            Error::OriginOutOfBound => write!(f, "Offset out of bound"),
            Error::Utf8Error => write!(f, "In (`FromUtf8`): catch `Utf8Error` or `FromUtf8Error`"),
            Error::FromLeBytes => write!(f, "In (`FromLeBytes`): need more bytes for given type"),
            Error::FromBeBytes => write!(f, "In (`FromBeBytes`): need more bytes for given type"),
            Error::FromNeBytes => write!(f, "In (`FromNeBytes`): need more bytes for given type"),
            Error::Other => write!(f, "Error::Other"),
            Error::Uid(id) => write!(f, "Got error(id = {id})"),
        }
    }
}
