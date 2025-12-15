use neure::prelude::*;

#[test]
fn sep() {
    assert!(sep_impl().is_ok());
}

fn sep_impl() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let comma = ','.once();
    let digit = neu::range('0'..='9').many1().sep(comma);
    let mut ctx = CharsCtx::new("123,456,789");

    assert_eq!(
        ctx.ctor_span(&digit)?,
        vec![Span::new(0, 3), Span::new(4, 3), Span::new(8, 3)]
    );
    Ok(())
}
