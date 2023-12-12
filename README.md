# neure

A fast little combinational parsing library

## Performance

`rel` is mean release, `fat` is mean release with lto=fat

![img](https://github.com/araraloren/neure/blob/e1965e572d7d88406d962a33569c1cfd175296bf/performance.png)

See [`examples`](https://github.com/araraloren/neure/tree/main/examples)

## Example

```rust
use neure::prelude::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let digit = re!(['0' - '9']+); // match digit from 0 to 9 more than once
    let mut ctx = CharsCtx::new("2023rust");

    assert_eq!(ctx.map(&digit, |v: &str| Ok(v.parse::<u64>()))??, 2023);

    Ok(())
}
```

## Code comparation between crate `regex`

```rust
use ::regex::Regex;
use neure::prelude::*;
use neure::regex;

thread_local! {
    static REGEX: Regex = Regex::new(r"^([a-z0-9_\.\+-]+)@([\da-z\.-]+)\.([a-z\.]{2,6})$").unwrap();
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let test_cases = [
        "plainaddress",
        "#@%^%#$@#$@#.com",
        "@example.com",
        "joe smith <email@example.com>",
        "”(),:;<>[ ]@example.com",
        "much.”more unusual”@example.com",
        "very.unusual.”@”.unusual.com@example.com",
        "email@example.com",
        "firstname.lastname@example.com",
        "email@subdomain.example.com",
    ];

    let un_letter = unit!(['a' - 'z']);
    let un_number = unit!(['0' - '9']);
    let un_us = unit!('_');
    let un_dot = unit!('.');
    let un_plus = unit!('+');
    let un_minus = unit!('-');
    let re_at = regex!('@');
    let un_postfix = un_letter.or(un_dot);
    let un_domain = un_postfix.or(un_number.or(un_minus));
    let re_prefix = regex!((un_domain.or(un_us.or(un_plus)))+);
    let re_domain =
        regex::count_if::<0, { usize::MAX }, _, _>(un_domain, |ctx: &CharsCtx, char| {
            if char.1 == '.' {
                // don't match dot if we don't have more dot
                if let Ok(str) = ctx.orig_at(ctx.offset() + char.0 + 1) {
                    return str.find('.').is_some();
                }
            }
            true
        });
    let re_postfix = regex!((un_postfix){2,6});
    let re_dot = regex!('.');
    let re_start = regex::start();
    let re_end = regex::end();
    let parser = |storer: &mut SimpleStorer, str| -> Result<(), neure::err::Error> {
        let mut ctx = CharsCtx::new(str);

        ctx.try_mat(&re_start)?;
        storer.try_cap(0, &mut ctx, &re_prefix)?;
        ctx.try_mat(&re_at)?;
        storer.try_cap(1, &mut ctx, &re_domain)?;
        ctx.try_mat(&re_dot)?;
        storer.try_cap(2, &mut ctx, &re_postfix)?;
        ctx.try_mat(&re_end)?;
        Ok(())
    };

    let mut storer = SimpleStorer::new(3);
    let mut locs = REGEX.try_with(|re| re.capture_locations()).unwrap();

    test_cases.iter().for_each(|test| {
        let res1 = parser(storer.reset(), test).is_ok();
        let res2 = REGEX
            .try_with(|regex| regex.captures_read(&mut locs, test).is_some())
            .unwrap();

        assert_eq!(res1, res2);
    });
    Ok(())
}
```

## LICENSE

MPL-2.0