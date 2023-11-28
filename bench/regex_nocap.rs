use ::regex::Regex;
use criterion::{black_box, Criterion};
use neure::prelude::*;

fn bench_color(c: &mut Criterion) {
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
    let re: Regex = Regex::new(r"^([a-z0-9_\.\+-]+)@([\da-z\.-]+)\.([a-z\.]{2,6})$").unwrap();

    c.bench_function("email of regex no capture", {
        move |b| {
            b.iter(|| {
                black_box(email_regex::parse(
                    black_box(&re),
                    black_box(&test_cases),
                    black_box(&results),
                ))
            })
        }
    });

    c.bench_function("email of neure no capture", {
        move |b| {
            b.iter(|| {
                black_box(email_neure::parse(
                    black_box(&test_cases),
                    black_box(&results),
                ))
            })
        }
    });

    c.bench_function("email of neure 2 no capture", {
        move |b| {
            b.iter(|| {
                black_box(email_neure2::parse(
                    black_box(&test_cases),
                    black_box(&results),
                ))
            })
        }
    });
}

criterion::criterion_group!(
    name = benches;
    config = Criterion::default().configure_from_args();
    targets = bench_color
);

criterion::criterion_main!(benches);

mod email_neure {
    use super::*;

    fn parser(str: &str) -> Result<(), neure::err::Error> {
        let mut ctx = RegexCtx::new(str);
        let alpha = neu!(['a' - 'z']);
        let num = neu!(['0' - '9']);
        let name = re!((alpha, num, '_', '.', '+', '-')+);
        let domain = alpha.or(num).or('.').or('-').repeat_full().set_cond(
            |ctx: &CharsCtx, item: &(usize, char)| {
                Ok(!(item.1 == '.' && ctx.orig_at(ctx.offset() + item.0 + 1)?.find('.').is_none()))
            },
        );
        let post = re!((alpha, '.'){2,6});
        let start = re::start();
        let end = re::end();

        ctx.try_mat(&start)?;
        ctx.try_mat(&name)?;
        ctx.try_mat(&"@")?;
        ctx.try_mat(&domain)?;
        ctx.try_mat(&".")?;
        ctx.try_mat(&post)?;
        ctx.try_mat(&end)?;
        Ok(())
    }

    pub fn parse(tests: &[&str], results: &[bool]) {
        for (test, result) in tests.iter().zip(results.iter()) {
            assert_eq!(parser(test).is_ok(), *result, "test = {}", test);
        }
    }
}

mod email_neure2 {
    use super::*;

    fn parser(str: &str) -> Result<(), neure::err::Error> {
        let mut ctx = RegexCtx::new(str);
        let alpha = neu!(['a' - 'z']);
        let num = neu!(['0' - '9']);
        let name = re!((alpha, num, '_', '.', '+', '-')+);
        let domain = alpha.or(num).or('.').or('-').repeat_full().set_cond(
            |ctx: &CharsCtx, item: &(usize, char)| {
                Ok(!(item.1 == '.' && ctx.orig_at(ctx.offset() + item.0 + 1)?.find('.').is_none()))
            },
        );
        let email = re::start()
            .and(name)
            .and("@")
            .and(domain)
            .and(".")
            .and(re!((alpha, '.'){2,6}))
            .and(re::end());

        ctx.try_mat(&email)?;
        Ok(())
    }

    pub fn parse(tests: &[&str], results: &[bool]) {
        for (test, result) in tests.iter().zip(results.iter()) {
            assert_eq!(parser(test).is_ok(), *result, "test = {}", test);
        }
    }
}

mod email_regex {
    use super::*;

    pub fn parse(re: &Regex, tests: &[&str], results: &[bool]) {
        for (test, result) in tests.iter().zip(results.iter()) {
            assert_eq!(re.is_match(test), *result);
        }
    }
}
