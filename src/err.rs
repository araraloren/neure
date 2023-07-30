#[derive(Debug)]
pub enum Error {
    Null,

    Chars,

    SubStr,

    ReachEnd,

    Match(String),
}
