#[derive(Debug, Clone, PartialEq)]
pub enum JsonZero<'a> {
    Null,
    Bool(bool),
    Str(&'a [u8]),
    Num(f64),
    Array(Vec<JsonZero<'a>>),
    Object(Vec<(&'a [u8], JsonZero<'a>)>),
}

static JSON: &[u8] = include_bytes!("samples/sample.json");

use neure::err::Error;
use neure::neu::range;
use neure::prelude::*;

#[derive(Debug, Default)]
pub struct JsonParser;

impl JsonParser {
    pub fn parse(pat: &[u8]) -> Result<JsonZero<'_>, Error> {
        let parser = regex::rec_parser_with(|ctor| {
            let ws = u8::is_ascii_whitespace.repeat_full();
            let sign = neu!((b'+', b'-')).repeat_zero_one();
            let digit = range(b'0'..=b'9').repeat_one_more();
            let dec = b".".then(digit).pat();
            let num = sign.then(digit).then(dec.or(regex::null()));
            let num = ctor::Wrap::dyn_box(num.pat().try_map(&Self::to_digit));

            let escape = neu!((b'\r', b'\t', b'\n', b'\\', b'\"'));
            let escape = b'\\'.then(escape);
            let cond = neu::re_cond(regex::not(escape));
            let str_val = b'\"'
                .not()
                .repeat_one_more()
                .set_cond(cond)
                .or(escape)
                .repeat(0..)
                .pat();
            let str = ctor::Wrap::dyn_box(str_val.quote(b"\"", b"\""));
            let str = str.try_map(|v| Ok(JsonZero::Str(v)));

            let bool_t = regex::lit_slice(b"true").try_map(|_| Ok(JsonZero::Bool(true)));
            let bool_f = regex::lit_slice(b"false").try_map(|_| Ok(JsonZero::Bool(false)));
            let null = regex::lit_slice(b"null").try_map(|_| Ok(JsonZero::Null));

            let ele = num.or(str.or(bool_t.or(bool_f.or(null.or(ctor.clone())))));
            let ele = ctor::Wrap::rc(ele.pad(ws).padded(ws));

            let key = regex!((u8::is_ascii_alphabetic.or(u8::is_ascii_digit), b'_')+);
            let key = key.quote(b"\"", b"\"");
            let key = key.pad(ws).padded(ws);
            let obj = key.sep_once(b":", ele.clone());
            let obj = ctor::Wrap::dyn_box(obj.sep(b",").quote(b"{", b"}"))
                .try_map(|v| Ok(JsonZero::Object(v)));

            let array = ele.clone().sep(b",").quote(b"[", b"]");
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

pub fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    dbg!(JsonParser::parse(JSON).unwrap());
}
