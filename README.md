# neure

A little combinational parsing library

## Example

```rust
use regex::Regex;
use neure::*;

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
        |ctx: &CharsCtx, offset, ch| {
            if ch == '.' {
                // don't match dot if we don't have more dot
                if let Ok(str) = ctx.peek_at(offset + 1) {
                    return str.find('.').is_some();
                }
            }
            true
        },
    );
    let postfix = neure!((group!(&letter, &dot)){2,6});
    let dot = neure!('.');
    let end = neure::end();
    let mut ctx = CharsCtx::default().with_capacity(3);
    let parser = |ctx: &mut CharsCtx| -> Result<(), neure::err::Error> {
        ctx.try_mat(&start)?;
        ctx.try_cap(0, &prefix)?;
        ctx.try_mat(&at)?;
        ctx.try_cap(1, &domain)?;
        ctx.try_mat(&dot)?;
        ctx.try_cap(2, &postfix)?;
        ctx.try_mat(&end)?;
        Ok(())
    };

    measure(100000, 100000, || {
        let mut count = 0;

        for test in test_cases {
            ctx.reset_with(test);
            if let Ok(_) = parser(&mut ctx) {
                count += 1;
            }
        }

        count
    });

    measure(100000, 100000, || {
        let mut count = 0;

        for test in test_cases {
            REGEX
                .try_with(|regex| {
                    if let Some(_) = regex.captures(test) {
                        count += 1;
                    }
                })
                .unwrap();
        }
        count
    });

    Ok(())
}
```

## LICENSE

MPL-2.0