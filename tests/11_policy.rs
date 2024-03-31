use neure::ctx::re_policy;
use neure::prelude::*;

#[test]
fn policy() {
    assert!(policy_impl().is_ok());
}

fn policy_impl() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let dat = std::fs::read_to_string(file!())?;
    let ident = char::is_ascii_alphabetic
        .or('_')
        .repeat_one()
        .then(char::is_ascii_alphanumeric.or('_').repeat_zero_more())
        .pat();
    let path = ident.sep("::").with_skip(true);
    let use_parser = path.then("*".opt()).padded("use").pad(";");

    // ignore white space using re_policy
    let mut ctx = CharsCtx::new(&dat).with_policy(re_policy(neu::whitespace().repeat_full()));

    let uses = ctx.ctor(&use_parser.collect::<_, Vec<_>>())?;

    assert_eq!(uses.len(), 2);
    assert_eq!(uses[0], (vec!["neure", "ctx", "re_policy"], None));
    assert_eq!(uses[1], (vec!["neure", "prelude"], Some("*")));
    Ok(())
}
