# neure

A fast little combinational parsing library

## Performance

`rel` is mean release, `fat` is mean release with lto=fat

![img](https://github.com/araraloren/neure/blob/e1965e572d7d88406d962a33569c1cfd175296bf/performance.png)

See [`examples`](https://github.com/araraloren/neure/tree/main/examples)

## Example

```rust
use neure::neure;
use neure::prelude::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let digit = neure!(['0' - '9']+); // match digit from 0 to 9 more than once
    let mut ctx = CharsCtx::new("2023rust");

    assert_eq!(
        ctx.lazy()
            .pat(&digit)
            .map(|v: &str| Ok(v.parse::<u64>()))??,
        2023
    );

    Ok(())
}
```

## Code comparation between crate `regex`

```rust
use ::regex::Regex;
use neure::group;
use neure::neure;
use neure::parser;
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

    let letter = regex!(['a' - 'z']);
    let number = regex!(['0' - '9']);
    let under_score = regex!('_');
    let dot = regex!('.');
    let plus = regex!('+');
    let minus = regex!('-');
    let at = neure!('@');
    let prefix = neure!((group!(&letter, &number, &under_score, &dot, &plus, &minus))+);
    let start = parser::start();
    let domain = parser::count_if::<0, { usize::MAX }, _>(
        group!(&letter, &number, &dot, &minus),
        |ctx: &CharsCtx, char| {
            if char.1 == '.' {
                // don't match dot if we don't have more dot
                if let Ok(str) = ctx.orig_at(ctx.offset() + char.0 + 1) {
                    return str.find('.').is_some();
                }
            }
            true
        },
    );
    let postfix = neure!((group!(&letter, &dot)){2,6});
    let dot = neure!('.');
    let end = parser::end();
    let parser = |storer: &mut SimpleStorer, str| -> Result<(), neure::err::Error> {
        let mut ctx = CharsCtx::new(str);

        ctx.try_mat(&start)?;
        storer.try_cap(0, &mut ctx, &prefix)?;
        ctx.try_mat(&at)?;
        storer.try_cap(1, &mut ctx, &domain)?;
        ctx.try_mat(&dot)?;
        storer.try_cap(2, &mut ctx, &postfix)?;
        ctx.try_mat(&end)?;
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