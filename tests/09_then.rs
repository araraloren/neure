use neure::prelude::*;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let val = neu::ascii_alphabetic().repeat_one_more();
    let num = neu::ascii_alphanumeric().repeat_one_more();
    let tuple = val.then(num);

    assert_eq!(
        CharsCtx::new("abc42").ctor_span(&tuple)?,
        (Span::new(0, 3), Span::new(3, 2))
    );
    Ok(())
}
