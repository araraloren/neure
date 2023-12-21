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
mod neure_ {
    use neure::prelude::*;

    fn parser(str: &str) -> Result<(), neure::err::Error> {
        let mut ctx = RegexCtx::new(str);
        let alpha = neu::range('a'..='z');
        let num = neu::digit(10);
        let name = neu!((alpha, num, '_', '.', '+', '-')).repeat_one_more();
        let domain = alpha.or(num).or('.').or('-').repeat_to::<256>().set_cond(
            |ctx: &CharsCtx, item: &(usize, char)| {
                Ok(!(item.1 == '.' && ctx.orig_at(ctx.offset() + item.0 + 1)?.find('.').is_none()))
            },
        );
        let email = re::start()
            .then(name)
            .then("@")
            .then(domain)
            .then(".")
            .then(neu!((alpha, '.')).repeat::<2, 6>())
            .then(re::end());

        ctx.try_mat(&email)?;
        Ok(())
    }

    pub fn parse(tests: &[&str], results: &[bool]) {
        for (test, result) in tests.iter().zip(results.iter()) {
            assert_eq!(parser(test).is_ok(), *result, "test = {}", test);
        }
    }
}

mod regex_ {
    use regex::Regex;

    pub fn parse(re: &Regex, tests: &[&str], results: &[bool]) {
        for (test, result) in tests.iter().zip(results.iter()) {
            assert_eq!(re.is_match(test), *result);
        }
    }
}

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

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
    let results = [
        false, false, false, false, false, false, false, true, true, true,
    ];
    let re: regex::Regex =
        regex::Regex::new(r"^([a-z0-9_\.\+-]+)@([\da-z\.-]+)\.([a-z\.]{2,6})$").unwrap();

    regex_::parse(&re, &test_cases, &results);
    neure_::parse(&test_cases, &results);

    Ok(())
}
```

## LICENSE

MPL-2.0