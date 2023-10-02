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
use neure::prelude::*;
use neure::*;

#[derive(Debug, Default)]
pub struct JsonParser;

impl JsonParser {
    const SPACE: neure::neure::WhiteSpace = neure::whitespace();

    pub fn parse<'a>(pat: &'a [u8]) -> Result<JsonZero<'a>, Error> {
        let mut ctx = BytesCtx::new(pat);
        let ret = Self::parse_object(&mut ctx);

        if ret.is_err() {
            Self::parse_array(&mut ctx)
        } else {
            ret
        }
    }

    pub fn parse_object<'a>(ctx: &mut BytesCtx<'a>) -> Result<JsonZero<'a>, Error> {
        let hash_beg = neure!(b'{');
        let hash_end = neure!(b'}');
        let sep = neure!(b':');
        let comma = neure!(b',');

        if Self::try_mat(ctx, &hash_beg).is_ok() {
            let mut objs = Vec::default();

            while let Ok(key) = Self::parse_key(ctx) {
                Self::try_mat(ctx, &sep)?;

                if let Ok(value) = Self::parse_bool_or_null(ctx) {
                    objs.push((key, value));
                } else if let Ok(str) = Self::parse_string(ctx) {
                    objs.push((key, str));
                } else if let Ok(num) = Self::parse_number(ctx) {
                    objs.push((key, num));
                } else if let Ok(array) = Self::parse_array(ctx) {
                    objs.push((key, array));
                } else if let Ok(object) = Self::parse_object(ctx) {
                    objs.push((key, object));
                } else {
                    break;
                }
                if Self::try_mat(ctx, &comma).is_err() {
                    break;
                }
            }
            Self::try_mat(ctx, &hash_end).unwrap();
            return Ok(JsonZero::Object(objs));
        }
        Err(Error::Null)
    }

    pub fn parse_array<'a>(ctx: &mut BytesCtx<'a>) -> Result<JsonZero<'a>, Error> {
        let array_beg = neure!(b'[');
        let array_end = neure!(b']');
        let comma = neure!(b',');

        if Self::try_mat(ctx, &array_beg).is_ok() {
            let mut objs = vec![];

            loop {
                if let Ok(value) = Self::parse_bool_or_null(ctx) {
                    objs.push(value);
                } else if let Ok(str) = Self::parse_string(ctx) {
                    objs.push(str);
                } else if let Ok(num) = Self::parse_number(ctx) {
                    objs.push(num);
                } else if let Ok(array) = Self::parse_array(ctx) {
                    objs.push(array);
                } else if let Ok(object) = Self::parse_object(ctx) {
                    objs.push(object);
                } else {
                    break;
                }
                if Self::try_mat(ctx, &comma).is_err() {
                    break;
                }
            }
            Self::try_mat(ctx, &array_end)?;
            return Ok(JsonZero::Array(objs));
        }
        Err(Error::Null)
    }

    pub fn try_mat<'a, 'c>(
        ctx: &mut BytesCtx<'c>,
        parser: impl Parse<BytesCtx<'c>, Ret = Return>,
    ) -> Result<Return, Error> {
        let space = parser::zero_more(|byte| char::from_u32(*byte as u32).unwrap().is_whitespace());

        ctx.try_mat_policy(
            parser,
            |ctx| {
                ctx.try_mat(&space)?;
                Ok(())
            },
            |ctx, ret| {
                if let Ok(ret) = &ret {
                    ctx.inc(ret.snd());
                }
                ret
            },
        )
    }

    pub fn parse_key<'a>(ctx: &mut BytesCtx<'a>) -> Result<&'a [u8], Error> {
        let space = parser::zero_more(|byte| char::from_u32(*byte as u32).unwrap().is_whitespace());
        let str_quote = neure!(b'"');
        let alpha = regex!( [b'a' - b'z' b'A' - b'Z' b'0' - b'9']);
        let under_score = regex!(b'_');
        let key = neure!((group!(&alpha, &under_score))+);

        ctx.try_mat(space)?;
        ctx.lazy()
            .quote(&str_quote, &str_quote)
            .pat(&key)
            .map(|str: &'a [u8]| Ok(str))
    }

    pub fn parse_bool_or_null<'a>(ctx: &mut BytesCtx<'a>) -> Result<JsonZero<'a>, Error> {
        let space = parser::zero_more(|byte| char::from_u32(*byte as u32).unwrap().is_whitespace());
        let true_ = parser::bytes(b"true");
        let false_ = parser::bytes(b"false");
        let null = parser::bytes(b"null");

        ctx.try_mat(space)?;
        ctx.lazy()
            .pat(&true_)
            .with(JsonZero::Bool(true))
            .or_with(&false_, JsonZero::Bool(false))
            .or_with(&null, JsonZero::Null)
            .map(|v: JsonZero<'a>| Ok(v))
    }

    pub fn parse_string<'a>(ctx: &mut BytesCtx<'a>) -> Result<JsonZero<'a>, Error> {
        let space = parser::zero_more(|byte| char::from_u32(*byte as u32).unwrap().is_whitespace());
        let str_quote = neure!(b'"');
        let str_val = neure!( [^ b'"' ]*);

        ctx.try_mat(space)?;
        ctx.lazy()
            .quote(&str_quote, &str_quote)
            .pat(&str_val)
            .map(|str: &'a [u8]| Ok(JsonZero::Str(str)))
    }

    pub fn parse_number<'a>(ctx: &mut BytesCtx<'a>) -> Result<JsonZero<'a>, Error> {
        let space = Self::SPACE.repeat(0..);
        let sign = regex!(['+' '-']{0,1});
        let digit = regex!(['0' - '9']{1,});
        let dot = regex!('.');
        let r#if = |ctx: &BytesCtx<'a>| ctx.orig().map(|v| v.get(0) == Some(&b'.'));
        let f64_ = space.then(sign).then(digit).then(regex::branch(
            r#if,
            dot.then(digit),
            regex::consume(0),
        ));

        ctx.map_orig(&f64_, |str| {
            std::str::from_utf8(str)
                .map(|v| v.parse::<f64>())
                .unwrap()
                .unwrap()
        })
    }
}

pub fn main() {
    dbg!(JsonParser::parse(&JSON).unwrap());
}
