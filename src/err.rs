use std::fmt::Display;

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
pub enum Error {
    Fail,

    AssertFalse,

    AssertTrue,

    Consume,

    Slice,

    LitSlice,

    LitString,

    AnchorEnd,

    AnchorStart,

    Mutex,

    Option,

    FromStr,

    TryInto,

    SelectEq,

    SepCollect,

    Collect,

    Separate,

    Repeat,

    Times,

    Between,

    Many1,

    Once,

    NeureThen,

    OutOfBound,

    Array,

    Vector,

    PairArray,

    PairVector,

    PairSlice,

    Utf8Error,

    FromLeBytes,

    FromBeBytes,

    FromNeBytes,

    Request,

    Uid(usize),
}

impl std::error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Fail => write!(f, "Fail: always failed"),
            Error::AssertFalse => write!(f, "AssertFalse: internal pattern match succeeded"),
            Error::AssertTrue => write!(f, "AssertTrue: internal pattern match failed"),
            Error::Consume => write!(f, "Consume: remaining data length is insufficient"),
            Error::Slice => write!(f, "Slice: all slices failed to match"),
            Error::LitSlice => write!(f, "LitSlice: slice failed to match"),
            Error::LitString => write!(f, "LitString: string failed to match"),
            Error::AnchorEnd => write!(f, "AnchorEnd: offset is not at the end"),
            Error::AnchorStart => write!(f, "AnchorStart: offset is not at the beginning"),
            Error::Mutex => write!(f, "Mutex: mutex lock operation failed"),
            Error::Option => write!(f, "Option: pattern is `None`, failed to match"),
            Error::FromStr => write!(f, "FromStr: call `std::str::parse` failed"),
            Error::TryInto => write!(f, "TryInto: call `TryInto::try_into` failed"),
            Error::SelectEq => write!(f, "SelectEq: elements in the tuple are not equal"),
            Error::SepCollect => write!(
                f,
                "SepCollect: number of matched patterns does not meet the requirement"
            ),
            Error::Collect => write!(
                f,
                "Collect: number of matched patterns does not meet the requirement"
            ),
            Error::Separate => write!(
                f,
                "Separate: number of matched patterns does not meet the requirement"
            ),
            Error::Repeat => write!(
                f,
                "Repeat: number of matched patterns does not meet the requirement"
            ),
            Error::Times => write!(
                f,
                "Times: number of matched units does not meet the requirement"
            ),
            Error::Between => write!(
                f,
                "Between: number of matched units does not meet the requirement"
            ),
            Error::Many1 => write!(
                f,
                "Many1: number of matched units does not meet the requirement"
            ),
            Error::Once => write!(
                f,
                "Once: number of matched units does not meet the requirement"
            ),
            Error::NeureThen => write!(f, "NeureThen: unit failed to match"),
            Error::Array => write!(f, "Array: all patterns failed to match"),
            Error::Vector => write!(f, "Vector: all patterns failed to match"),
            Error::PairArray => write!(f, "PairArray: all patterns failed to match"),
            Error::PairVector => write!(f, "PairVector: all patterns failed to match"),
            Error::PairSlice => write!(f, "PairSlice: all patterns failed to match"),
            Error::OutOfBound => write!(f, "offset out of bound"),
            Error::Utf8Error => write!(f, "FromUtf8: call `std::str::from_utf8` failed"),
            Error::FromLeBytes => write!(f, "FromLeBytes: need more bytes for given type"),
            Error::FromBeBytes => write!(f, "FromBeBytes: need more bytes for given type"),
            Error::FromNeBytes => write!(f, "FromNeBytes: need more bytes for given type"),
            Error::Request => write!(f, "request data failed"),
            Error::Uid(id) => write!(f, "Got error(id = {id})"),
        }
    }
}
