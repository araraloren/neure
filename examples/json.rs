#[derive(Debug, Clone, PartialEq)]
pub enum JsonZero<'a> {
    Null,
    Bool(bool),
    Str(&'a [u8]),
    Num(f64),
    Array(Vec<JsonZero<'a>>),
    Object(Vec<(&'a [u8], JsonZero<'a>)>),
}

static JSON: &'static [u8] = include_bytes!("samples/sample.json");

use neure::err::Error;
use neure::neu::range;
use neure::prelude::*;
use neure::re::RecursiveCtorWith;

#[derive(Debug, Default)]
pub struct JsonParser;

impl JsonParser {
    pub fn parse<'a>(pat: &'a [u8]) -> Result<JsonZero<'a>, Error> {
        let parser = re::rec_parser_with(Self::parser);
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

    pub fn parser<'a: 'b, 'b>(
        ctor: RecursiveCtorWith<'b, BytesCtx<'a>, JsonZero<'a>>,
    ) -> impl Fn(&mut BytesCtx<'a>) -> Result<JsonZero<'a>, Error> + 'b {
        move |ctx| {
            let ws = u8::is_ascii_whitespace.repeat_full();
            let sign = neu!((b'+', b'-')).repeat_zero_one();
            let digit = range(b'0'..=b'9').repeat_one_more();
            let dec = b".".then(digit).pat();
            let num = sign.then(digit).then(dec.or(re::null()));
            let num = num.pat().map(&Self::to_digit);

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
            let str = str_val.quote(b"\"", b"\"");
            let str = str.map(|v| Ok(JsonZero::Str(v)));

            let bool_t = re::lit_slice(b"true").map(|_| Ok(JsonZero::Bool(true)));
            let bool_f = re::lit_slice(b"false").map(|_| Ok(JsonZero::Bool(false)));
            let null = re::lit_slice(b"null").map(|_| Ok(JsonZero::Null));

            let ele = num.or(str.or(bool_t.or(bool_f.or(null.or(ctor.clone())))));
            let ele = ele.pad(ws).padded(ws);

            let key = re!((u8::is_ascii_alphabetic.or(u8::is_ascii_digit), b'_')+);
            let key = key.quote(b"\"", b"\"");
            let key = key.pad(ws).padded(ws);
            let obj = key.sep_once(b":", ele);
            let obj = obj
                .sep(b",")
                .quote(b"{", b"}")
                .map(|v| Ok(JsonZero::Object(v)));

            let ele = num.or(str.or(bool_t.or(bool_f.or(null.or(ctor.clone())))));
            let ele = ele.pad(ws).padded(ws);

            let array = ele.sep(b",").quote(b"[", b"]");
            let array = array.map(|v| Ok(JsonZero::Array(v)));

            ctx.ctor(&obj.or(array))
        }
    }
}

pub fn main() {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    dbg!(JsonParser::parse(&JSON).unwrap());
}
