use ::regex::Regex;
use neure::*;

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
    let parser = |str| -> Result<(), neure::err::Error> {
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
                    if let Ok(str) = Context::orig_at(ctx, ctx.offset() + char.0 + 1) {
                        return str.find('.').is_some();
                    }
                }
                true
            },
        );
        let postfix = neure!((group!(&letter, &dot)){2,6});
        let dot = neure!('.');
        let end = neure::end();
        let mut ctx = CharsCtx::new(str);

        ctx.try_mat(&start)?;
        ctx.try_mat(&prefix)?;
        ctx.try_mat(&at)?;
        ctx.try_mat(&domain)?;
        ctx.try_mat(&dot)?;
        ctx.try_mat(&postfix)?;
        ctx.try_mat(&end)?;
        Ok(())
    };

    measure(1000000, 1000000, || {
        let mut count = 0;
        test_cases.iter().for_each(|test| {
            parser(test).is_ok().then(|| count += 1);
        });
        count
    });
    let re: Regex = Regex::new(r"^([a-z0-9_\.\+-]+)@([\da-z\.-]+)\.([a-z\.]{2,6})$").unwrap();

    measure(1000000, 1000000, || {
        let mut count = 0;
        test_cases.iter().for_each(|test| {
            re.is_match(test).then(|| count += 1);
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
        "Size = {size}, Cost = {}, Test = {}, Avg = {}, Count = {}",
        time.as_millis(),
        n,
        time.as_micros() as f64 / n as f64,
        sum,
    );
}
