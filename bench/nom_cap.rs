use criterion::{black_box, Criterion};
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

fn bench_color(c: &mut Criterion) {
    let color_str = "#2F14DF";

    c.bench_function("color of nom", {
        move |b| b.iter(|| black_box(color_nom::parse(black_box(color_str))))
    });

    c.bench_function("color of neure", {
        move |b| b.iter(|| black_box(color_neure::parse(black_box(color_str))))
    });
}

criterion::criterion_group!(
    name = benches;
    config = Criterion::default().configure_from_args();
    targets = bench_color
);

criterion::criterion_main!(benches);

mod color_nom {
    use super::*;

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

    pub fn parse(str: &str) {
        let (_, color) = hex_color(str).unwrap();

        assert_eq!(
            color,
            Color {
                red: 47,
                green: 20,
                blue: 223,
            }
        );
    }
}

mod color_neure {
    use super::*;

    fn parser(str: &str) -> Result<Color, Box<dyn std::error::Error>> {
        let pound = neure!('#');
        let hex = neure!(['0' - '9' 'A' - 'F']{2});
        let from_str = |str: &str| Ok(u8::from_str_radix(str, 16));
        let hex = hex.map(from_str);
        let mut ctx = RegexCtx::new(str);

        ctx.reset();
        ctx.try_mat(&pound)?;

        Ok(Color {
            red: ctx.invoke(&hex)??,
            green: ctx.invoke(&hex)??,
            blue: ctx.invoke(&hex)??,
        })
    }

    pub fn parse(str: &str) {
        let color = parser(str).unwrap();

        assert_eq!(
            color,
            Color {
                red: 47,
                green: 20,
                blue: 223,
            }
        );
    }
}
