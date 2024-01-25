use criterion::{black_box, criterion_group, criterion_main, Criterion};

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
        std::str::from_utf8(val).map_err(|_| Error::Other)
    }

    impl JsonParser {
        pub fn parse(pat: &[u8]) -> Result<Json, Error> {
            let parser = re::rec_parser_with(|ctor| {
                let ws = u8::is_ascii_whitespace.repeat_full();
                let sign = b'+'.or(b'-').repeat_zero_one();
                let digit = neu::range(b'0'..=b'9').repeat_one_more();
                let dec = b".".then(digit).pat();
                let num = sign.then(digit).then(dec.or(re::null()));
                let num = num
                    .pat()
                    .map(to_str)
                    .map(map::from_str::<f64>())
                    .map(|v| Ok(Json::Num(v)))
                    .into_dyn();

                let escape = b'\r'.or(b'\t').or(b'\n').or(b'\\').or(b'\"');
                let escape = b'\\'.then(escape);
                let cond = neu::re_cond(re::not(escape));
                let str_val = b'\"'
                    .not()
                    .repeat_one_more()
                    .set_cond(cond)
                    .or(escape)
                    .repeat(0..)
                    .pat();
                let str = str_val
                    .map(to_str)
                    .map(map::from_str::<String>())
                    .map(|v| Ok(Json::Str(v)))
                    .quote(b"\"", b"\"")
                    .into_dyn();

                let bool_t = re::lit_slice(b"true").map(|_| Ok(Json::Bool(true)));
                let bool_f = re::lit_slice(b"false").map(|_| Ok(Json::Bool(false)));
                let null = re::lit_slice(b"null").map(|_| Ok(Json::Null));

                let ele = num.or(str.or(bool_t.or(bool_f.or(null.or(ctor.clone())))));
                let ele = ele.pad(ws).padded(ws).into_rc();

                let key = re!((u8::is_ascii_alphabetic.or(u8::is_ascii_digit), b'_')+)
                    .map(to_str)
                    .map(map::from_str::<String>())
                    .quote(b"\"", b"\"");
                let key = key.pad(ws).padded(ws);
                let obj = key.sep_once(b":", ele.clone());
                let obj = obj.sep(b",").quote(b"{", b"}").map(|v| Ok(Json::Object(v)));
                let obj = obj.into_dyn();

                let array = ele.sep(b",").quote(b"[", b"]");
                let array = array.map(|v| Ok(Json::Array(v)));

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
            let parser = re::rec_parser_with(|ctor| {
                let ws = u8::is_ascii_whitespace.repeat_full();
                let sign = re!((b'+', b'-'){0,1});
                let digit = neu::range(b'0'..=b'9').repeat_one_more();
                let dec = b".".then(digit).pat();
                let num = sign.then(digit).then(dec.or(re::null()));
                let num = num.pat().into_dyn().map(&Self::to_digit);

                let escape = neu!((b'\r', b'\t', b'\n', b'\\', b'\"'));
                let escape = b'\\'.then(escape);
                let cond = neu::re_cond(re::not(escape));
                let str_val = b'\"'
                    .not()
                    .repeat_one_more()
                    .set_cond(cond)
                    .or(escape)
                    .repeat(0..)
                    .pat();
                let str = str_val.quote(b"\"", b"\"").into_dyn();
                let str = str.map(|v| Ok(JsonZero::Str(v)));

                let bool_t = re::lit_slice(b"true").map(|_| Ok(JsonZero::Bool(true)));
                let bool_f = re::lit_slice(b"false").map(|_| Ok(JsonZero::Bool(false)));
                let null = re::lit_slice(b"null").map(|_| Ok(JsonZero::Null));

                let ele = num.or(str.or(bool_t.or(bool_f.or(null.or(ctor.clone())))));
                let ele = ele.pad(ws).padded(ws).into_rc();

                let key = re!((u8::is_ascii_alphabetic.or(u8::is_ascii_digit), b'_')+);
                let key = key.quote(b"\"", b"\"");
                let key = key.pad(ws).padded(ws);
                let obj = key.sep_once(b":", ele.clone());
                let obj = obj
                    .sep(b",")
                    .quote(b"{", b"}")
                    .into_dyn()
                    .map(|v| Ok(JsonZero::Object(v)));

                let array = ele.sep(b",").quote(b"[", b"]").into_dyn();
                let array = array.map(|v| Ok(JsonZero::Array(v)));

                obj.or(array)
            });
            let mut ctx = BytesCtx::new(pat);

            ctx.ctor(&parser)
        }

        pub fn to_digit<'a>(val: &[u8]) -> Result<JsonZero<'a>, Error> {
            std::str::from_utf8(val)
                .map_err(|_| Error::Other)?
                .parse::<f64>()
                .map_err(|_| Error::Other)
                .map(JsonZero::Num)
        }
    }
}
