use neure::prelude::*;

#[test]
fn or() {
    assert!(or_impl().is_ok());
}

fn or_impl() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let name = regex::literal("localhost");
    let ip = regex::literal("127.0.0.1");
    let local = name.or(ip);

    assert_eq!(CharsCtx::new("127.0.0.1").try_mat(&local)?, Span::new(0, 9));
    assert_eq!(CharsCtx::new("localhost").try_mat(&local)?, Span::new(0, 9));
    Ok(())
}
