use neure::err::Error;
use neure::prelude::*;
use nom::AsBytes;
use std::{cell::RefCell, process::exit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    // png reference http://www.libpng.org/pub/png/spec/1.2/PNG-Structure.html
    if let Some(file) = std::env::args().skip(1).next() {
        let head: &[u8] = &[137, 80, 78, 71, 13, 10, 26, 10];
        let bytes = std::fs::read(file)?;
        let uint32 = parser::consume(4);
        let uint8 = parser::consume(1);
        let as_uint = |dat: &[u8]| {
            assert_eq!(dat.len(), 4);
            Ok(u32::from_be_bytes([dat[0], dat[1], dat[2], dat[3]]))
        };
        let as_char = |dat: &[u8]| {
            assert_eq!(dat.len(), 1);
            char::from_u32(u8::from_be_bytes([dat[0]]) as u32).ok_or(Error::Convert)
        };
        let mut ctx = Parser::new(bytes.as_bytes());

        if let Ok::<Span, _>(_) = ctx.try_mat(&head) {
            println!("Matching the head, the file seems like a png file");
        } else {
            println!("Not a png file");
            exit(1)
        }
        for idx in 0.. {
            if let Ok(length) = ctx.map_orig(&uint32, &as_uint) {
                let crc_offset_beg = ctx.offset();
                let ancillary = ctx.map_orig(&uint8, as_char)?;
                let private = ctx.map_orig(&uint8, as_char)?;
                let reserved = ctx.map_orig(&uint8, as_char)?;
                let safe_copy = ctx.map_orig(&uint8, as_char)?;

                println!(
                    "In trunk {idx}: ancillary = `{}`, bit 5 = {}: {}",
                    ancillary,
                    if ancillary.is_uppercase() { 0 } else { 1 },
                    if ancillary.is_uppercase() {
                        "critical"
                    } else {
                        "ancillary"
                    }
                );
                println!(
                    "In trunk {idx}: private = `{}`, bit 5 = {}: {}",
                    private,
                    if private.is_uppercase() { 0 } else { 1 },
                    if private.is_uppercase() {
                        "public"
                    } else {
                        "private"
                    }
                );
                println!(
                    "In trunk {idx}: reserved = `{}`, bit 5 = {}",
                    reserved,
                    if reserved.is_uppercase() { 0 } else { 1 },
                );
                println!(
                    "In trunk {idx}: safe_copy = `{}`, bit 5 = {}: {}",
                    safe_copy,
                    if safe_copy.is_uppercase() { 0 } else { 1 },
                    if safe_copy.is_uppercase() {
                        "unsafe to copy"
                    } else {
                        "safe to copy"
                    }
                );
                let data = parser::consume(length as usize);

                println!("In trunk {idx}: data length = {length}");
                println!(
                    "skip data data = {:?}",
                    ctx.map_span(&data, |span| Ok(span))?
                );

                let crc_data = ctx.orig_sub(crc_offset_beg, ctx.offset() - crc_offset_beg)?;
                let crc_value = ctx.map_orig(&uint32, as_uint)?;

                println!(
                    "Checking the crc value = {}",
                    crc_value == calc_crc(crc_data)
                );
            } else {
                break;
            }
        }
    }
    Ok(())
}

thread_local! {
    static CRC_TABLE: RefCell<Option<[u32; 256]>> = RefCell::new(None);
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
                                c = c >> 1;
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
