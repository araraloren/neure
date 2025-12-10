use neure::prelude::*;

#[test]
fn sep_once() {
    assert!(sep_once_impl().is_ok());
}

fn sep_once_impl() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let words = ':'.not().repeat_one_more();
    let title = words.sep_once(":", words);

    assert_eq!(
        CharsCtx::new("Explained: What is programming language?").ctor_span(&title)?,
        (Span::new(0, 9), Span::new(10, 30)),
    );
    Ok(())
}
