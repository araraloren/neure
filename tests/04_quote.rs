use neure::prelude::*;

#[test]
fn quote() {
    assert!(quote_impl().is_ok());
}

#[cfg(not(feature = "alloc"))]
fn quote_impl() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let comma = ','.once();
    let digit = neu::range('0'..='9').many1().sep2::<_, 1, 5>(comma);
    let array = digit.enclose("[", "]");

    assert_eq!(
        CharsCtx::new("[123,456,789]").ctor_span(&array)?,
        [
            Some(Span::new(1, 3)),
            Some(Span::new(5, 3)),
            Some(Span::new(9, 3)),
            None,
            None
        ]
    );
    Ok(())
}

#[cfg(feature = "alloc")]
fn quote_impl() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let comma = ','.once();
    let digit = neu::range('0'..='9').many1().sep(comma);
    let array = digit.enclose("[", "]");

    assert_eq!(
        CharsCtx::new("[123,456,789]").ctor_span(&array)?,
        vec![Span::new(1, 3), Span::new(5, 3), Span::new(9, 3)]
    );
    Ok(())
}
