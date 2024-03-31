use neure::prelude::*;

#[test]
fn repeat() {
    assert!(repeat_impl().is_ok());
}

fn repeat_impl() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let name = re::string("foo");
    let names = name.repeat(2..5);

    assert!(CharsCtx::new("foo").ctor_span(&names).is_err(),);
    assert_eq!(
        CharsCtx::new("foofoofoofoo").ctor_span(&names)?,
        vec![
            Span::new(0, 3),
            Span::new(3, 3),
            Span::new(6, 3),
            Span::new(9, 3)
        ]
    );
    Ok(())
}
