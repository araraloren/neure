use neure::*;
use std::{cell::RefCell, process::exit};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    if let Some(file) = std::env::args().skip(1).next() {
        let head: &'static [u8] = &[137, 80, 78, 71, 13, 10, 26, 10];
        let bytes = std::fs::read(file)?;
        let head = neure::bytes(head);
        let int32 = neure::consume(4);
        let int8 = neure::consume(1);
        let mut ctx = BytesCtx::new(&bytes);
        let mut storer = SpanStorer::default().with_capacity(5);

        if ctx.try_cap(0, &mut storer, &head).is_err() {
            println!("Not a png file");
            exit(1)
        } else {
            println!("Matching the head, the file seems like a png file");
        }
        for idx in 0.. {
            if ctx.cap(1, &mut storer, &int32) {
                let length = storer.slice(&bytes, 1, idx)?;
                let length = i32::from_be_bytes([length[0], length[1], length[2], length[3]]);
                let data = neure::consume(length as usize);
                let crc_offset_beg = ctx.offset();

                println!("In trunk {idx}: length = {length}");
                ctx.try_cap(2, &mut storer, &int8)?;
                ctx.try_cap(2, &mut storer, &int8)?;
                ctx.try_cap(2, &mut storer, &int8)?;
                ctx.try_cap(2, &mut storer, &int8)?;

                let type_code = storer.slice_iter(&bytes, 2)?;
                let mut type_code = type_code.skip(idx * 4);
                let ancillary = u8::from_be_bytes([type_code.next().unwrap()[0]]);
                let ancillary = char::from_u32(ancillary as u32).unwrap();
                let private = u8::from_be_bytes([type_code.next().unwrap()[0]]);
                let private = char::from_u32(private as u32).unwrap();
                let reserved = u8::from_be_bytes([type_code.next().unwrap()[0]]);
                let reserved = char::from_u32(reserved as u32).unwrap();
                let safe_copy = u8::from_be_bytes([type_code.next().unwrap()[0]]);
                let safe_copy = char::from_u32(safe_copy as u32).unwrap();

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
                println!("skip data data = {:?}", ctx.try_cap(3, &mut storer, &data)?);
                let crc_data = &bytes[crc_offset_beg..ctx.offset()];

                ctx.try_cap(4, &mut storer, &int32)?;
                let crc_value = storer.slice(&bytes, 4, idx)?;
                let crc_value =
                    u32::from_be_bytes([crc_value[0], crc_value[1], crc_value[2], crc_value[3]]);

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
