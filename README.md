# neure

A fast little combinational parsing library

## Example

```rust
use neure::*;

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
    let digit = neure!(['0' - '9']+); // match digit from 0 to 9 more than once
    let mut ctx = CharsCtx::default().with_capacity(1).with_str("2023rust");

    ctx.try_cap(0, digit)?;
    assert_eq!(ctx.spans(0), Some(&vec![Span { beg: 0, len: 4 }]));

    Ok(())
}
```

## Performance match between crate `regex`

```rust
use ::regex::Regex;
use neure::*;

thread_local! {
    static REGEX: Regex = Regex::new(r"^([a-z0-9_\.\+-]+)@([\da-z\.-]+)\.([a-z\.]{2,6})$").unwrap();
}

pub fn main() -> Result<(), Box<dyn std::error::Error>> {
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

    // Size = 100000, Cost time 0.068436 with test 100000 times: 0.0000006843599999999999 --> 300000
    measure(100000, 100000, || {
        let mut count = 0;
        test_cases.iter().for_each(|test| {
            ctx.reset_with(*test);
            parser(&mut ctx).is_ok().then(|| count += 1);
        });
        count
    });
    // Size = 100000, Cost time 0.1784131 with test 100000 times: 0.000001784131 --> 300000
    measure(100000, 100000, || {
        let mut count = 0;
        test_cases.iter().for_each(|test| {
            REGEX
                .try_with(|regex| {
                    regex.captures(test).is_some().then(|| count += 1);
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