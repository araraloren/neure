use neure::prelude::*;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let comma = ','.repeat_one();
    let digit = neu::range('0'..='9').repeat_one_more().sep(comma);
    let mut ctx = CharsCtx::new("123,456,789");

    assert_eq!(
        ctx.ctor_span(&digit)?,
        vec![Span::new(0, 3), Span::new(4, 3), Span::new(8, 3)]
    );
    Ok(())
}
