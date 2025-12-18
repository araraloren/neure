use std::hint::black_box;

use ::regex::Regex;
use criterion::Criterion;
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
                email_regex::parse(black_box(&re), black_box(&test_cases), black_box(&results));
                black_box(())
            })
        }
    });

    c.bench_function("email of neure no capture", {
        move |b| {
            b.iter(|| {
                email_neure::parse(black_box(&test_cases), black_box(&results));
                black_box(())
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
            assert_eq!(parser(test).is_ok(), *result, "test = {test}");
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
