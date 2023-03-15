const MASK_7: u8 = 0b01111111;
const SET_8: u8 = 0b10000000;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IncompleteNumber,
    Overflow,
}

/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut bytes = Vec::with_capacity(values.len() * 4);

    for &value in values {
        for i in (1..byte_count(value)).rev() {
            let shifted = (value >> (7 * i)) as u8;
            bytes.push((shifted & MASK_7) | SET_8);
        }

        bytes.push(value as u8 & MASK_7);
    }

    bytes
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, Error> {
    let mut numbers = Vec::with_capacity(bytes.len() / 4 + 1);
    let mut number_bytes = Vec::new();

    for &byte in bytes.into_iter() {
        let (bits, control_bit) = (byte & MASK_7, byte & SET_8);
        number_bytes.push(bits);

        if control_bit == 0 {
            numbers.push(value_from(&mut number_bytes)?);
        }; 
    }

    match number_bytes.len() {
        0 => Ok(numbers),
        _ => Err(Error::IncompleteNumber)
    }
    
}

fn byte_count(value: u32) -> u32 {
    match value {
        0..=127 => 1,
        128..=16383 => 2,
        16384..=2097151 => 3,
        2097152..=268435455 => 4,
        _ => 5
    }
}

fn value_from(bytes: &mut Vec<u8>) -> Result<u32, Error> {
    if bytes.len() == 5 && &bytes[..] > &[0x0f, 0xef, 0xef, 0xef, 0x7f] {
        return Err(Error::Overflow);
    }

    let mut value = 0;
    let mut i = 0;

    while let Some(byte) = bytes.pop() {
        let shifted = (byte as u32) << (7 * i);
        value |= shifted;
        i += 1;
    }

    Ok(value)
}