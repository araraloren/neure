use neure::map::{FromBeBytes, from_be_bytes};
use neure::prelude::*;
use neure::{err::Error, map::FallibleMap};
use std::mem::size_of;
use std::process::exit;
use std::{cell::RefCell, marker::PhantomData};

#[derive(Debug, Clone, Copy)]
pub struct PngParser<T> {
    size: usize,

    marker: PhantomData<T>,
}

impl<T> Default for PngParser<T> {
    fn default() -> Self {
        Self {
            size: size_of::<T>(),
            marker: Default::default(),
        }
    }
}

impl<T> PngParser<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(size: usize) -> Self {
        Self {
            size,
            marker: PhantomData,
        }
    }

    pub fn parse(self, ctx: &mut BytesCtx) -> Result<T, Error>
    where
        Self: for<'a> FallibleMap<&'a [u8], T>,
    {
        ctx.ctor(&regex::consume(self.out_size()).try_map(self))
    }
}

impl<'a, T> FallibleMap<&'a [u8], T> for PngParser<T>
where
    FromBeBytes<T>: FallibleMap<&'a [u8], T>,
{
    fn out_size(&self) -> usize {
        self.size
    }

    // map all data from big endian
    fn try_map(&self, val: &'a [u8]) -> Result<T, Error> {
        from_be_bytes::<T>().try_map(val)
    }
}

pub struct Data(Vec<u8>);

impl<'a> FallibleMap<&'a [u8], Data> for PngParser<Data> {
    fn out_size(&self) -> usize {
        self.size
    }

    fn try_map(&self, val: &'a [u8]) -> Result<Data, Error> {
        Ok(Data(val.to_vec()))
    }
}

#[derive(Debug)]
pub struct Trunk {
    ancillary: u8,
    private: u8,
    reserverd: u8,
    safe_copy: u8,
    crc_value: u32,
    data: Vec<u8>,
}

impl<'a> FallibleMap<&'a [u8], Trunk> for PngParser<Trunk> {
    fn out_size(&self) -> usize {
        self.size
            + PngParser::<u8>::default().out_size() * 4
            + PngParser::<u32>::default().out_size()
    }

    fn try_map(&self, val: &'a [u8]) -> Result<Trunk, Error> {
        let u8_parser = PngParser::new();
        let inner_ctx = &mut BytesCtx::new(val);
        let ancillary = u8_parser.parse(inner_ctx)?;
        let private = u8_parser.parse(inner_ctx)?;
        let reserverd = u8_parser.parse(inner_ctx)?;
        let safe_copy = u8_parser.parse(inner_ctx)?;
        let data: Data = PngParser::with_capacity(self.size).parse(inner_ctx)?;
        let crc_data = inner_ctx.orig_sub(0, inner_ctx.offset())?;
        let crc_value: u32 = PngParser::new().parse(inner_ctx)?;
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
            data: data.0,
        })
    }
}

impl Trunk {
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
        &self.data
    }

    pub fn crc_value(&self) -> u32 {
        self.crc_value
    }

    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        self.data.len()
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
        let mut trunks = vec![];

        for idx in 0.. {
            if let Ok(length) = PngParser::<u32>::new().parse(ctx) {
                // pass data length to PngParser
                let trunk: Trunk = PngParser::with_capacity(length as usize).parse(ctx)?;

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
                trunks.push(trunk);
            } else {
                break;
            }
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
