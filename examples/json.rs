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
use neure::*;

#[derive(Debug, Default)]
pub struct JsonParser;

impl JsonParser {
    pub fn parse<'a>(pat: &'a [u8]) -> Result<JsonZero<'a>, Error> {
        let mut ctx = BytesCtx::new(pat);
        let ret = Self::parse_object(pat, &mut ctx);

        if ret.is_err() {
            Self::parse_array(pat, &mut ctx)
        } else {
            ret
        }
    }

    pub fn parse_object<'a>(pat: &'a [u8], ctx: &mut BytesCtx) -> Result<JsonZero<'a>, Error> {
        let hash_beg = neure!(b'{');
        let hash_end = neure!(b'}');
        let sep = neure!(b':');
        let comma = neure!(b',');

        if Self::try_mat(ctx, &hash_beg).is_ok() {
            let mut objs = Vec::default();

            while let Ok(key) = Self::parse_key(pat, ctx) {
                Self::try_mat(ctx, &sep)?;

                if let Ok(value) = Self::parse_bool_or_null(ctx) {
                    objs.push((key, value));
                } else if let Ok(str) = Self::parse_string(pat, ctx) {
                    objs.push((key, str));
                } else if let Ok(num) = Self::parse_number(pat, ctx) {
                    objs.push((key, num));
                } else if let Ok(array) = Self::parse_array(pat, ctx) {
                    objs.push((key, array));
                } else if let Ok(object) = Self::parse_object(pat, ctx) {
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

    pub fn parse_array<'a>(pat: &'a [u8], ctx: &mut BytesCtx) -> Result<JsonZero<'a>, Error> {
        let array_beg = neure!(b'[');
        let array_end = neure!(b']');
        let comma = neure!(b',');

        if Self::try_mat(ctx, &array_beg).is_ok() {
            let mut objs = vec![];

            loop {
                if let Ok(value) = Self::parse_bool_or_null(ctx) {
                    objs.push(value);
                } else if let Ok(str) = Self::parse_string(pat, ctx) {
                    objs.push(str);
                } else if let Ok(num) = Self::parse_number(pat, ctx) {
                    objs.push(num);
                } else if let Ok(array) = Self::parse_array(pat, ctx) {
                    objs.push(array);
                } else if let Ok(object) = Self::parse_object(pat, ctx) {
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
        parser: impl Parser<BytesCtx<'c>, Ret = Ret>,
    ) -> Result<Ret, Error> {
        let space = neure::zero_more(|byte| char::from_u32(*byte as u32).unwrap().is_whitespace());

        ctx.try_mat_policy(
            parser,
            |ctx| {
                ctx.try_mat(&space)?;
                Ok(())
            },
            |ctx, ret| {
                if let Ok(ret) = &ret {
                    ctx.inc(ret.offset());
                }
                ret
            },
        )
    }

    pub fn parse_key<'a>(pat: &'a [u8], ctx: &mut BytesCtx) -> Result<&'a [u8], Error> {
        let str_quote = neure!(b'"');
            let alpha = regex!( [b'a' - b'z' b'A' - b'Z' b'0' - b'9']);
            let under_score = regex!(b'_');
            let key = neure!((group!(&alpha, &under_score))+);

            if let Ok(_) = Self::try_mat(ctx, &str_quote) {
                let start = ctx.offset();

                ctx.try_mat(&key)?;
                let ret = pat.get(start..ctx.offset()).ok_or_else(|| Error::Null)?;

                ctx.try_mat(&str_quote)?;
                Ok(ret)
            } else {
                Err(Error::Null)
            }
    }

    pub fn parse_bool_or_null<'a>(ctx: &mut BytesCtx) -> Result<JsonZero<'a>, Error> {
        let space =
                neure::zero_more(|byte| char::from_u32(*byte as u32).unwrap().is_whitespace());
        let true_ = seq!(&space, neure::bytes(b"true"));
        let null_ = seq!(&space, neure::bytes(b"null"));
        let false_ = seq!(&space, neure::bytes(b"false"));

        map!(
            ctx {
                &true_ => { Ok(JsonZero::Bool(true)) },
                &false_ => { Ok(JsonZero::Bool(false)) },
                &null_ => { Ok(JsonZero::Null) },
                { Err(Error::Null) }
            }
        )
    }

    pub fn parse_string<'a>(pat: &'a [u8], ctx: &mut BytesCtx) -> Result<JsonZero<'a>, Error> {
        let str_quote = neure!(b'"');
            let str_val = neure!( [^ b'"' ]*);
            let start = ctx.offset();

            if Self::try_mat(ctx, &str_quote).is_ok() {
                ctx.try_mat(&str_val)?;
                ctx.try_mat(&str_quote)?;
                Ok(JsonZero::Str(
                    pat.get(start..ctx.offset()).ok_or_else(|| Error::Null)?,
                ))
            } else {
                Err(Error::Null)
            }
    }

    pub fn parse_number<'a>(pat: &'a [u8], ctx: &mut BytesCtx) -> Result<JsonZero<'a>, Error> {
        let sign = neure!( [b'+' b'-']);
            let digit = neure!( [b'0' - b'9']+);
            let dot = neure!(b'.');
            let space =
                neure::zero_more(|byte| char::from_u32(*byte as u32).unwrap().is_whitespace());
            let _ = ctx.mat(&space);
            let start = ctx.offset();

            ctx.mat(&sign);
            if let Ok(_) = ctx.try_mat(&digit) {
                if ctx.try_mat(&dot).is_ok() {
                    ctx.try_mat(&digit)?;
                }
                Ok(JsonZero::Num(
                    std::str::from_utf8(pat.get(start..ctx.offset()).ok_or_else(|| Error::Null)?)
                        .unwrap()
                        .parse::<f64>()
                        .unwrap(),
                ))
            } else {
                Err(Error::Null)
            }
    }
}

pub fn main() {
    dbg!(JsonParser::parse(&JSON).unwrap());
}
