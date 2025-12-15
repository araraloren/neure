use neure::prelude::*;
use std::fs::read_to_string;

#[test]
fn policy() {
    assert!(policy_impl().is_ok());
}

fn policy_impl() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let dat = read_to_string(file!())?;
    let ident = char::is_ascii_alphabetic
        .or('_')
        .once()
        .then(char::is_ascii_alphanumeric.or('_').many0())
        .pat();
    let path = ident.sep("::").with_skip(true);
    let use_parser = path.then("*".opt()).prefix("use").suffix(";");

    // ignore white space using re_policy
    let mut ctx = CharsCtx::new(&dat).skip_before(neu::whitespace().many0());

    let uses = ctx.ctor(&use_parser.collect::<_, Vec<_>>())?;

    assert_eq!(uses.len(), 2);
    assert_eq!(uses[0], (vec!["neure", "prelude"], Some("*")));
    assert_eq!(uses[1], (vec!["std", "fs", "read_to_string"], None));
    Ok(())
}
