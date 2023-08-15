# neure

A fast little combinational parsing library

## Performance

`rel` is mean release, `fat` is mean release with lto=fat

![img](https://raw.githubusercontent.com/araraloren/neure/main/performance.png)

See [`examples`](https://github.com/araraloren/neure/tree/main/examples)

## Example

```rust
use neure::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let digit = neure!(['0' - '9']+); // match digit from 0 to 9 more than once
    let mut storer = SpanStorer::new(1);
    let mut ctx = CharsCtx::default().with_str("2023rust");

    ctx.try_cap(0, &mut storer, digit)?;
    assert_eq!(storer.spans(0)?, &vec![Span { beg: 0, len: 4 }]);

    Ok(())
}
```

## Performance match between crate `regex`

```rust
use ::regex::Regex;
use neure::{span::SpanStorer, *};

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
    let start = neure::start();
    let domain = neure::count_if::<0, { usize::MAX }, _>(
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
    let end = neure::end();
    let mut storer = SpanStorer::new(3);
    let parser = |storer: &mut SpanStorer, str| -> Result<(), neure::err::Error> {
        let mut ctx = CharsCtx::new(str);

        ctx.try_mat(&start)?;
        ctx.try_cap(0, storer, &prefix)?;
        ctx.try_mat(&at)?;
        ctx.try_cap(1, storer, &domain)?;
        ctx.try_mat(&dot)?;
        ctx.try_cap(2, storer, &postfix)?;
        ctx.try_mat(&end)?;
        Ok(())
    };

    measure(1000, 1000, || {
        let mut count = 0;
        test_cases.iter().for_each(|test| {
            parser(&mut storer, test).is_ok().then(|| count += 1);
        });
        count
    });
    let mut locs = REGEX.try_with(|re| re.capture_locations()).unwrap();

    measure(1000, 1000, || {
        let mut count = 0;
        test_cases.iter().for_each(|test| {
            REGEX
                .try_with(|regex| {
                    regex
                        .captures_read(&mut locs, test)
                        .is_some()
                        .then(|| count += 1);
                })
                .unwrap();
        });
        count
    });
    Ok(())
}

pub fn measure(n: usize, size: usize, mut f: impl FnMut() -> i32) {
    use std::time::Instant;

    let start = Instant::now();
    let mut sum = 0;
    for _ in 0..n {
        sum += f();
    }
    let time = start.elapsed();
    println!(
        "Size = {size}, Cost time {} with test {} times: {} --> {}",
        time.as_secs_f64(),
        n,
        time.as_secs_f64() / n as f64,
        sum,
    );
}
```

## LICENSE

MPL-2.0