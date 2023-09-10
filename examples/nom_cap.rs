use neure::{prelude::*, *};
use nom::{
    bytes::complete::{tag, take_while_m_n},
    combinator::map_res,
    sequence::tuple,
    IResult,
};

#[derive(Debug, PartialEq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

fn from_hex(input: &str) -> Result<u8, std::num::ParseIntError> {
    u8::from_str_radix(input, 16)
}

fn is_hex_digit(c: char) -> bool {
    c.is_digit(16)
}

fn hex_primary(input: &str) -> IResult<&str, u8> {
    map_res(take_while_m_n(2, 2, is_hex_digit), from_hex)(input)
}

fn hex_color(input: &str) -> IResult<&str, Color> {
    let (input, _) = tag("#")(input)?;
    let (input, (red, green, blue)) = tuple((hex_primary, hex_primary, hex_primary))(input)?;

    Ok((input, Color { red, green, blue }))
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let color_str = "#2F14DF";

    fn parser(str: &str) -> Result<Color, neure::err::Error> {
        let pound = neure!('#');
        let hex = neure!(['0' - '9' 'A' - 'F']{2});
        let mut ctx = Parser::new(str);
        let mut ctx = ctx.non_lazy();
        let from_str =
            |str: &str| u8::from_str_radix(str, 16).map_err(|_| neure::err::Error::Match);

        ctx.reset();
        ctx.try_mat(&pound)?;

        Ok(Color {
            red: ctx.pat(&hex)?.map(&from_str)?,
            green: ctx.pat(&hex)?.map(&from_str)?,
            blue: ctx.pat(&hex)?.map(&from_str)?,
        })
    }

    measure(1000_0000, 1000_0000, || {
        if let Ok(color) = parser(color_str) {
            assert_eq!(
                color,
                Color {
                    red: 47,
                    green: 20,
                    blue: 223,
                }
            );
            1
        } else {
            0
        }
    });

    measure(1000_0000, 1000_0000, || {
        if hex_color("#2F14DF").unwrap()
            == (
                "",
                Color {
                    red: 47,
                    green: 20,
                    blue: 223,
                },
            )
        {
            1
        } else {
            0
        }
    });
    Ok(())
}

pub fn measure(n: usize, size: usize, mut f: impl FnMut() -> i32) {
    use std::time::Instant;

    let start = Instant::now();
    let mut sum = 0;
    for _ in 0..n {
        sum += f();
    }
    let time = start.elapsed();
    println!(
        "Size = {size}, Cost = {}, Test = {}, Avg = {}, Count = {}",
        time.as_millis(),
        n,
        time.as_nanos() as f64 / n as f64,
        sum,
    );
}
