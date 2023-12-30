use neure::prelude::*;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let val = neu::ascii_alphabetic().repeat_one();
    let vec = val.collect::<_, Vec<_>>();

    assert_eq!(
        CharsCtx::new("abcdf").ctor_span(&vec)?,
        vec![
            Span::new(0, 1),
            Span::new(1, 1),
            Span::new(2, 1),
            Span::new(3, 1),
            Span::new(4, 1),
        ]
    );
    Ok(())
}