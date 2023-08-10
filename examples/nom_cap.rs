use neure::*;
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
    let mut storer = SpanStorer::new(1);
    let color_str = "#2F14DF";

    fn parser(storer: &mut SpanStorer, str: &str) -> Result<(), neure::err::Error> {
        let pound = neure!('#');
        let hex = neure!(['0' - '9' 'A' - 'F']{2});
        let mut ctx = CharsCtx::default().with_str(str);

        ctx.reset();
        ctx.try_mat(&pound)?;
        ctx.try_cap(0, storer, &hex)?;
        ctx.try_cap(0, storer, &hex)?;
        ctx.try_cap(0, storer, &hex)?;
        Ok(())
    }

    measure(1000_0000, 1000_0000, || {
        if parser(storer.reset(), color_str).is_ok() {
            let mut strs = storer.substrs(color_str, 0).unwrap();
            assert_eq!(
                Color {
                    red: u8::from_str_radix(strs.next().unwrap(), 16).unwrap(),
                    green: u8::from_str_radix(strs.next().unwrap(), 16).unwrap(),
                    blue: u8::from_str_radix(strs.next().unwrap(), 16).unwrap(),
                },
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
        time.as_micros() as f64 / n as f64,
        sum,
    );
}
