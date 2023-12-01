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
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(("email", "example", "com")),
        Some(("firstname.lastname", "example", "com")),
        Some(("email", "subdomain.example", "com")),
    ];
    let re: Regex = Regex::new(r"^([a-z0-9_\.\+-]+)@([\da-z\.-]+)\.([a-z\.]{2,6})$").unwrap();
    let mut locs = re.capture_locations();

    c.bench_function("email of regex", {
        move |b| {
            b.iter(|| {
                black_box(email_regex::parse(
                    black_box(&re),
                    black_box(&mut locs),
                    black_box(&test_cases),
                    black_box(&results),
                ))
            })
        }
    });

    c.bench_function("email of neure", {
        move |b| {
            b.iter(|| {
                black_box(email_neure::parse(
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

    fn parser(str: &str) -> Result<(&str, &str, &str), neure::err::Error> {
        let letter = neu::range('a' ..= 'z');
        let number = neu::digit(10);
        let name = re!((letter, number, '_', '.', '+', '-')+);
        let domain = neu!((letter, number, '.', '-'))
            .repeat_to::<256>()
            .set_cond(move |ctx: &CharsCtx, &(length, ch): &(usize, char)| {
                Ok(!(ch == '.' && ctx.orig_at(ctx.offset() + length + 1)?.find('.').is_none()))
            });
        let post = neu!((letter, '.')).repeat::<2, 6>();
        let email = name
            .sep_once("@", domain.sep_once(".", post))
            .map(|(v1, (v2, v3))| Ok((v1, v2, v3)))
            .quote(re::start(), re::end());
        let mut ctx = RegexCtx::new(str);

        ctx.ctor(&email)
    }

    pub fn parse(tests: &[&str], results: &[Option<(&str, &str, &str)>]) {
        for (test, result) in tests.iter().zip(results.iter()) {
            let ret = parser(test);

            assert_eq!(ret.is_ok(), result.is_some(), "test = {}", test);
            if let Some(result) = result {
                let ret = ret.unwrap();

                assert_eq!(ret.0, result.0);
                assert_eq!(ret.1, result.1);
                assert_eq!(ret.2, result.2);
            }
        }
    }
}

mod email_regex {
    use super::*;
    use ::regex::CaptureLocations;

    pub fn parse(
        re: &Regex,
        locs: &mut CaptureLocations,
        tests: &[&str],
        results: &[Option<(&str, &str, &str)>],
    ) {
        for (test, result) in tests.iter().zip(results.iter()) {
            let ret = re.captures_read(locs, test);

            assert_eq!(ret.is_some(), result.is_some());
            if let Some(result) = result {
                let res1 = locs.get(1).unwrap();
                let res2 = locs.get(2).unwrap();
                let res3 = locs.get(3).unwrap();

                assert_eq!(test.get(res1.0..res1.1).unwrap(), result.0);
                assert_eq!(test.get(res2.0..res2.1).unwrap(), result.1);
                assert_eq!(test.get(res3.0..res3.1).unwrap(), result.2);
            }
        }
    }
}
