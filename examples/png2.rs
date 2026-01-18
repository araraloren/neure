use core::cell::RefCell;
use neure::map::{fixed_size, from_be_bytes};
use neure::prelude::*;
use neure::{err::Error, map::FallibleMap};
use std::process::exit;

pub fn parse<'a, P, O>(parser: P, bc: &mut BytesCtx<'a>) -> Result<O, Error>
where
    P: FallibleMap<&'a [u8], O>,
{
    let size = parser.out_size();

    bc.ctor(&regex::consume(size).try_map(parser))
}

#[derive(Debug)]
pub struct Trunk<'a> {
    ancillary: u8,
    private: u8,
    reserverd: u8,
    safe_copy: u8,
    crc_value: u32,
    data: &'a [u8],
}

impl<'a> Trunk<'a> {
    pub fn new(bc: &mut BytesCtx<'a>, length: usize) -> Result<Self, neure::err::Error> {
        let start = bc.offset();
        let ancillary = parse(from_be_bytes(), bc)?;
        let private = parse(from_be_bytes(), bc)?;
        let reserverd = parse(from_be_bytes(), bc)?;
        let safe_copy = parse(from_be_bytes(), bc)?;

        let data: &[u8] = parse(fixed_size(length), bc)?;

        let crc_data = bc.orig_sub(start, bc.offset() - start)?;
        let crc_value: u32 = parse(from_be_bytes(), bc)?;
        let calc_value = calc_crc(crc_data);

        if crc_value != calc_value {
            panic!("Incorrectly crc value => got {crc_value} in file <-> {calc_value}");
        }
        Ok(Trunk {
            ancillary,
            private,
            reserverd,
            safe_copy,
            crc_value,
            data,
        })
    }

    pub fn is_ancillary(&self) -> bool {
        self.ancillary.is_ascii_lowercase()
    }

    pub fn is_private(&self) -> bool {
        self.private.is_ascii_lowercase()
    }

    pub fn is_reserved(&self) -> bool {
        self.reserverd.is_ascii_lowercase()
    }

    pub fn is_safe_copy(&self) -> bool {
        self.safe_copy.is_ascii_lowercase()
    }

    pub fn data(&self) -> &[u8] {
        self.data
    }

    pub fn crc_value(&self) -> u32 {
        self.crc_value
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub struct Png<'a> {
    trunks: Vec<Trunk<'a>>,
}

impl<'a> Png<'a> {
    pub fn new(bc: &mut BytesCtx<'a>) -> Result<Self, neure::err::Error> {
        let mut trunks = vec![];

        while let Ok(length) = parse(from_be_bytes::<u32>(), bc) {
            trunks.push(Trunk::new(bc, length as usize)?);
        }

        Ok(Self { trunks })
    }

    fn trunks(&self) -> &[Trunk<'a>] {
        &self.trunks
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // png reference http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
    if let Some(file) = std::env::args().nth(1) {
        let buff = std::fs::read(file)?;
        let ctx = &mut BytesCtx::new(&buff);

        if ctx.ctor(&[137, 80, 78, 71, 13, 10, 26, 10]).is_ok() {
            println!("Matching the head, the file seems like a png file");
        } else {
            println!("Not a png file");
            exit(1)
        }
        let png = Png::new(ctx)?;

        for (idx, trunk) in png.trunks().iter().enumerate() {
            println!(
                "In trunk {idx}: {}",
                if trunk.is_ancillary() {
                    "critical"
                } else {
                    "ancillary"
                }
            );
            println!(
                "In trunk {idx}: {}",
                if !trunk.is_private() {
                    "public"
                } else {
                    "private"
                }
            );
            println!("In trunk {idx}: {}", !trunk.is_reserved());
            println!(
                "In trunk {idx}: {}",
                if !trunk.is_safe_copy() {
                    "unsafe to copy"
                } else {
                    "safe to copy"
                }
            );
            println!("In trunk {idx}: data length = {}", trunk.len());
        }
    }
    Ok(())
}

thread_local! {
    static CRC_TABLE: RefCell<Option<[u32; 256]>> = const { RefCell::new(None) };
}

pub fn initialize_crc_table() {
    CRC_TABLE
        .try_with(|table| {
            let mut table = table.borrow_mut();
            if table.is_none() {
                *table = Some({
                    let mut table: [u32; 256] = [0; 256];

                    for n in 0..256 {
                        let mut c = n;

                        for _ in 0..8 {
                            if c & 1 == 1 {
                                c = 0xedb88320u32 ^ (c >> 1);
                            } else {
                                c >>= 1;
                            }
                        }
                        table[n as usize] = c;
                    }
                    table
                })
            }
        })
        .unwrap()
}

// from https://www.w3.org/TR/PNG-CRCAppendix.html
pub fn calc_crc(buff: &[u8]) -> u32 {
    let mut c = 0xffffffffu32;

    initialize_crc_table();
    CRC_TABLE
        .try_with(|table| {
            let table = table.borrow();
            if let Some(table) = *table {
                for n in 0..buff.len() {
                    c = table[((c ^ (buff[n] as u32)) & 0xff) as usize] ^ (c >> 8);
                }
            }
        })
        .unwrap();
    c ^ 0xffffffffu32
}
