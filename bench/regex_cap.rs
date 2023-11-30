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
    let mut storer = SimpleStorer::new(3);

    c.bench_function("email of nom", {
        move |b| {
            b.iter(|| {
                black_box(email_nom::parse(
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
                    black_box(&mut storer),
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

    fn parser(storer: &mut SimpleStorer, str: &str) -> Result<(), neure::err::Error> {
        let start = re::start();
        let end = re::end();
        let letter = neu!(['a' - 'z']);
        let number = neu!(['0' - '9']);
        let pre = re!((letter, number, '_', '.', '+', '-')+);
        let domain = letter
            .or(number)
            .or('.')
            .or('-')
            .repeat_to::<30>()
            .set_cond(move |ctx: &CharsCtx, &(length, ch): &(usize, char)| {
                Ok(!(ch == '.' && ctx.orig_at(ctx.offset() + length + 1)?.find('.').is_none()))
            });
        let post = re!((letter, '.'){2,6});
        let mut ctx = RegexCtx::new(str);

        ctx.try_mat(&start)?;
        storer.try_cap(0, &mut ctx, &pre)?;
        ctx.try_mat(&"@")?;
        storer.try_cap(1, &mut ctx, &domain)?;
        ctx.try_mat(&".")?;
        storer.try_cap(2, &mut ctx, &post)?;
        ctx.try_mat(&end)?;
        Ok(())
    }

    pub fn parse(
        storer: &mut SimpleStorer,
        tests: &[&str],
        results: &[Option<(&str, &str, &str)>],
    ) {
        for (test, result) in tests.iter().zip(results.iter()) {
            let ret = parser(storer.reset(), test).is_ok();

            assert_eq!(ret, result.is_some(), "test = {}", test);
            if let Some(result) = result {
                assert_eq!(storer.slice(test, 0, 0).unwrap(), result.0);
                assert_eq!(storer.slice(test, 1, 0).unwrap(), result.1);
                assert_eq!(storer.slice(test, 2, 0).unwrap(), result.2);
            }
        }
    }
}

mod email_nom {
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
