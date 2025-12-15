use neure::prelude::*;

#[test]
fn longest() {
    assert!(ltm_impl().is_ok());
}

fn ltm_impl() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let name = regex::string("localhost");
    let ip = regex::string("8080");
    let port = name.sep_once(":", ip);
    let or = name.or(port);
    let longest = name.longest(port);

    assert_eq!(CharsCtx::new("localhost").try_mat(&or)?, Span::new(0, 9));
    assert_eq!(
        CharsCtx::new("localhost:8080").try_mat(&or)?,
        Span::new(0, 9)
    );
    assert_eq!(
        CharsCtx::new("localhost:8080").try_mat(&longest)?,
        Span::new(0, 14)
    );
    Ok(())
}
