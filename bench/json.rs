use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};

#[derive(Debug, Clone, PartialEq)]
pub enum Json {
    Null,
    Bool(bool),
    Str(String),
    Num(f64),
    Array(Vec<Json>),
    Object(Vec<(String, Json)>),
}

#[derive(Debug, Clone, PartialEq)]
pub enum JsonZero<'a> {
    Null,
    Bool(bool),
    Str(&'a [u8]),
    Num(f64),
    Array(Vec<JsonZero<'a>>),
    Object(Vec<(&'a [u8], JsonZero<'a>)>),
}

static JSON: &[u8] = include_bytes!("../examples/samples/sample.json");

fn bench_json(c: &mut Criterion) {
    c.bench_function("json neure", {
        move |b| b.iter(|| black_box(neure_json::JsonParser::parse(black_box(JSON)).unwrap()))
    });

    c.bench_function("json neure zero", {
        move |b| b.iter(|| black_box(neure_json_zero::JsonParser::parse(black_box(JSON)).unwrap()))
    });
}

criterion_group!(
    name = benches;
    config = Criterion::default().configure_from_args();
    targets = bench_json
);

criterion_main!(benches);

mod neure_json {
    use neure::err::Error;
    use neure::prelude::*;

    use super::Json;

    #[derive(Debug, Default)]
    pub struct JsonParser;

    pub fn to_str(val: &[u8]) -> Result<&str, Error> {
        std::str::from_utf8(val).map_err(|_| Error::Utf8Error)
    }

    impl JsonParser {
        pub fn parse(pat: &[u8]) -> Result<Json, Error> {
            let parser = regex::rec_parser(|ctor| {
                let ws = u8::is_ascii_whitespace.many0();
                let sign = b'+'.or(b'-').opt();
                let digit = neu::range(b'0'..=b'9').many1();
                let dec = b".".then(digit).pat();
                let num = sign.then(digit).then(dec.or(regex::empty()));
                let num = ctor::Adapter::dyn_box(
                    num.pat()
                        .try_map(to_str)
                        .try_map(map::from_str::<f64>())
                        .map(Json::Num),
                );

                let escape = b'\r'.or(b'\t').or(b'\n').or(b'\\').or(b'\"');
                let escape = b'\\'.then(escape);
                let cond = neu::regex_cond(regex::not(escape));
                let str_val = b'\"'
                    .not()
                    .many1()
                    .set_cond(cond)
                    .or(escape)
                    .repeat(0..)
                    .pat();
                let str = ctor::Adapter::dyn_box(
                    str_val
                        .try_map(to_str)
                        .try_map(map::from_str::<String>())
                        .try_map(|v| Ok(Json::Str(v)))
                        .enclose(b"\"", b"\""),
                );

                let bool_t = regex::lit_slice(b"true").try_map(|_| Ok(Json::Bool(true)));
                let bool_f = regex::lit_slice(b"false").try_map(|_| Ok(Json::Bool(false)));
                let empty = regex::lit_slice(b"empty").try_map(|_| Ok(Json::Null));

                let ele = num.or(str.or(bool_t.or(bool_f.or(empty.or(ctor.clone())))));
                let ele = ctor::Adapter::rc(ele.suffix(ws).prefix(ws));

                let key = regex!((u8::is_ascii_alphabetic.or(u8::is_ascii_digit), b'_')+)
                    .try_map(to_str)
                    .try_map(map::from_str::<String>())
                    .enclose(b"\"", b"\"");
                let key = key.suffix(ws).prefix(ws);
                let obj = key.sep_once(b":", ele.clone());
                let obj = obj.sep(b",").enclose(b"{", b"}").map(Json::Object);
                let obj = ctor::Adapter::dyn_box(obj);

                let array = ele.sep(b",").enclose(b"[", b"]");
                let array = array.try_map(|v| Ok(Json::Array(v)));

                obj.or(array)
            });
            let mut ctx = BytesCtx::new(pat);

            ctx.ctor(&parser)
        }
    }
}

mod neure_json_zero {
    use neure::err::Error;
    use neure::prelude::*;

    use super::JsonZero;

    #[derive(Debug, Default)]
    pub struct JsonParser;

    impl JsonParser {
        pub fn parse(pat: &[u8]) -> Result<JsonZero<'_>, Error> {
            let parser = regex::rec_parser(|ctor| {
                let ws = u8::is_ascii_whitespace.many0();
                let sign = regex!((b'+', b'-'){0,1});
                let digit = neu::range(b'0'..=b'9').many1();
                let dec = b".".then(digit).pat();
                let num = sign.then(digit).then(dec.or(regex::empty()));
                let num = ctor::Adapter::dyn_box(num.pat()).try_map(&Self::to_digit);

                let escape = neu!((b'\r', b'\t', b'\n', b'\\', b'\"'));
                let escape = b'\\'.then(escape);
                let cond = neu::regex_cond(regex::not(escape));
                let str_val = b'\"'
                    .not()
                    .many1()
                    .set_cond(cond)
                    .or(escape)
                    .repeat(0..)
                    .pat();
                let str = ctor::Adapter::dyn_box(str_val.enclose(b"\"", b"\""));
                let str = str.try_map(|v| Ok(JsonZero::Str(v)));

                let bool_t = regex::lit_slice(b"true").try_map(|_| Ok(JsonZero::Bool(true)));
                let bool_f = regex::lit_slice(b"false").try_map(|_| Ok(JsonZero::Bool(false)));
                let empty = regex::lit_slice(b"empty").try_map(|_| Ok(JsonZero::Null));

                let ele = num.or(str.or(bool_t.or(bool_f.or(empty.or(ctor.clone())))));
                let ele = ctor::Adapter::rc(ele.suffix(ws).prefix(ws));

                let key = regex!((u8::is_ascii_alphabetic.or(u8::is_ascii_digit), b'_')+);
                let key = key.enclose(b"\"", b"\"");
                let key = key.suffix(ws).prefix(ws);
                let obj = key.sep_once(b":", ele.clone());
                let obj = ctor::Adapter::dyn_box(obj.sep(b",").enclose(b"{", b"}"))
                    .try_map(|v| Ok(JsonZero::Object(v)));

                let array = ctor::Adapter::dyn_box(ele.sep(b",").enclose(b"[", b"]"));
                let array = array.try_map(|v| Ok(JsonZero::Array(v)));

                obj.or(array)
            });
            let mut ctx = BytesCtx::new(pat);

            ctx.ctor(&parser)
        }

        pub fn to_digit<'a>(val: &[u8]) -> Result<JsonZero<'a>, Error> {
            std::str::from_utf8(val)
                .map_err(|_| Error::Utf8Error)?
                .parse::<f64>()
                .map_err(|_| Error::FromStr)
                .map(JsonZero::Num)
        }
    }
}
