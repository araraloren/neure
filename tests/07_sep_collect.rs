use neure::prelude::*;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let host = '.'.or(neu::ascii_alphabetic()).repeat_one_more();
    let http = "http".sep_once("://", host);
    let https = "https".sep_once("://", host);
    let urls = http.or(https).sep_collect::<_, _, Vec<_>>(",");
    let ret = CharsCtx::new("https://developer.mozilla.org,http://developer.mozilla.org")
        .ctor_span(&urls)?;

    assert_eq!(
        ret,
        [
            (Span { beg: 0, len: 5 }, Span { beg: 8, len: 21 },),
            (Span { beg: 30, len: 4 }, Span { beg: 37, len: 21 },),
        ]
    );
    Ok(())
}
