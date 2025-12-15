# neure

A fast little combinational parsing library

## Why neure

* Better performance

* Fewer dependencies

* Faster compilation

## Example

For more, reference [`examples`](https://docs.rs/crate/neure/latest/source/examples/)

### Example 1

```rust
use neure::prelude::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let year = regex!(['0' - '9']+); // match digit from 0 to 9 more than once
    let year = year.try_map(map::from_str::<i32>()); // map it to i32
    let name = neu::ascii_alphabetic().many1(); // match ascii alphabetic
    let mut ctx = CharsCtx::new("2024rust");

    // .then construct a tuple
    assert_eq!(ctx.ctor(&year.then(name))?, (2024, "rust"));
    Ok(())
}

```

### Code comparation between crate `regex`

```rust
mod neure_ {
    use neure::prelude::*;

    fn parser(str: &str) -> Result<(), neure::err::Error> {
        let mut ctx = RegexCtx::new(str);
        let alpha = neu::range('a'..='z');
        let num = neu::digit(10);
        let name = regex!((alpha, num, '_', '.', '+', '-')+);
        let cond = ".".then('.'.not().many0()).then(regex::end());
        let cond = neu::regex_cond(regex::not(cond));
        let domain = alpha
            .or(num)
            .or('.')
            .or('-')
            .at_most::<256>()
            .set_cond(cond);
        let email = regex::start()
            .then(name)
            .sep_once(
                "@",
                domain.sep_once(".", neu!((alpha, '.')).between::<2, 6>()),
            )
            .then(regex::end());

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