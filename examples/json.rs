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

use std::cell::RefCell;
use std::rc::Rc;

use neure::err::Error;
use neure::prelude::*;
use neure::re::RecursiveCtor;

#[derive(Debug, Default)]
pub struct JsonParser;

impl JsonParser {
    pub fn parse<'a>(pat: &'a [u8]) -> Result<JsonZero<'a>, Error> {
        let parser = re::rec_parser(Self::parser);
        let mut ctx = BytesCtx::new(pat);

        ctx.ctor(&parser)
    }

    pub fn ws_u8() -> impl Neu<u8> + Clone {
        |byte: &u8| {
            char::from_u32(*byte as u32)
                .map(|v| v.is_whitespace())
                .unwrap_or(false)
        }
    }

    pub fn to_digit<'a>(val: &[u8]) -> Result<JsonZero<'a>, Error> {
        std::str::from_utf8(val)
            .map_err(|_| Error::Other)?
            .parse::<f64>()
            .map_err(|_| Error::Other)
            .map(JsonZero::Num)
    }

    pub fn parser<'a: 'b, 'b>(
        regex: RecursiveCtor<'b, BytesCtx<'a>, JsonZero<'a>>,
    ) -> impl Fn(&mut BytesCtx<'a>) -> Result<JsonZero<'a>, Error> + 'b {
        move |ctx| {
            let ws = Self::ws_u8().repeat_full();
            let sign = re!([b'+' b'-']{0,1});
            let digit = re!([b'0' - b'9']{1,});
            let dec = b".".and(digit).pat();
            let num = sign.and(digit).and(dec.or(re::null()));
            let num = num.pat().map(Self::to_digit);

            let str_val = re!([^ b'"']*);
            let str = str_val.quote(re!(b'"'), re!(b'"'));
            let str = str.map(|v| Ok(JsonZero::Str(v)));

            let bool_t = re::bytes(b"true").map(|_| Ok(JsonZero::Bool(true)));
            let bool_f = re::bytes(b"false").map(|_| Ok(JsonZero::Bool(false)));
            let null = re::bytes(b"null").map(|_| Ok(JsonZero::Null));

            let ele = num.or(str.or(bool_t.or(bool_f.or(null.or(regex.clone())))));
            let ele = ele.pad(ws.clone()).padded(ws.clone());
            let ele = Rc::new(RefCell::new(ele));

            let alpha = neu!([b'a' - b'z' b'A' - b'Z' b'0' - b'9']);
            let under_score = neu!(b'_');
            let key = re!((alpha, under_score)+);
            let key = key.quote(re!(b'"'), re!(b'"'));
            let key = key.pad(ws.clone()).padded(ws.clone());
            let obj = key.sep_once(b":", ele.clone());
            let obj = obj
                .sep(b",")
                .quote(b"{", b"}")
                .map(|v| Ok(JsonZero::Object(v)));

            let array = ele.sep(b",").quote(b"[", b"]");
            let array = array.map(|v| Ok(JsonZero::Array(v)));

            ctx.ctor(&obj.or(array))
        }
    }
}

pub fn main() {
    dbg!(JsonParser::parse(&JSON).unwrap());
}
