use neure::prelude::*;

#[test]
fn quote() {
    assert!(quote_impl().is_ok());
}

fn quote_impl() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let comma = ','.repeat_one();
    let digit = neu::range('0'..='9').repeat_one_more().sep(comma);
    let array = digit.quote("[", "]");

    assert_eq!(
        CharsCtx::new("[123,456,789]").span(&array)?,
        vec![Span::new(1, 3), Span::new(5, 3), Span::new(9, 3)]
    );
    Ok(())
}
