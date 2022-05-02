#![deny(rust_2018_idioms)]

use std::error::Error;
use std::fmt::{self, Display};

pub use chars::ReplacementChar;
use chars::ThaiChar;

mod chars;

/// Decode arbitrary octets as String. Returns a Result containing a String.
pub fn decode<T: AsRef<[u8]>>(input: T) -> Result<String, DecodeError> {
    let mut buffer = String::with_capacity(input.as_ref().len());
    decode_buf(input, &mut buffer)?;
    Ok(buffer)
}

/// Decode arbitrary octets as String. Writes into the supplied buffer to avoid allocation.
pub fn decode_buf<T: AsRef<[u8]>>(input: T, buffer: &mut String) -> Result<(), DecodeError> {
    let input = input.as_ref();
    for &byte in input {
        if is_ascii(byte) {
            buffer.push(char::from(byte));
        } else {
            buffer.push(ThaiChar::byte_to_char(byte).ok_or(DecodeError(byte))?);
        }
    }
    Ok(())
}

/// Encode arbitrary string as TIS-620. Returns a Result containing a Vec.
pub fn encode(input: &str) -> Result<Vec<u8>, EncodeError> {
    let mut buffer = Vec::with_capacity(input.len());
    encode_buf(input, &mut buffer)?;
    Ok(buffer)
}

/// Encode arbitrary string as TIS-620. Writes into the supplied buffer to avoid allocation.
pub fn encode_buf(input: &str, buffer: &mut Vec<u8>) -> Result<(), EncodeError> {
    for ch in input.chars() {
        if ch.is_ascii() {
            buffer.push(ch as u32 as u8);
        } else if let Some(tc) = ThaiChar::from_char(ch) {
            buffer.push(tc.to_tis620_byte());
        } else {
            return Err(EncodeError(ch));
        }
    }
    Ok(())
}

/// Encode arbitrary string as TIS-620, including invalid characters. Returns a Vec.
///
/// When invalid character is found, call `f` to get replacement character.
/// If `None` is returned, silently drop invalid character.
pub fn encode_lossy<F>(input: &str, f: F) -> Vec<u8>
where
    F: FnMut(char) -> Option<ReplacementChar>,
{
    let mut buffer = Vec::with_capacity(input.len());
    encode_lossy_buf(input, f, &mut buffer);
    buffer
}

/// Encode arbitrary string as TIS-620, including invalid characters.
/// Writes into the supplied buffer to avoid allocation.
///
/// When invalid character is found, call `f` to get replacement character.
/// If `None` is returned, silently drop invalid character.
pub fn encode_lossy_buf<F>(input: &str, mut f: F, buffer: &mut Vec<u8>)
where
    F: FnMut(char) -> Option<ReplacementChar>,
{
    for ch in input.chars() {
        if ch.is_ascii() {
            buffer.push(ch as u32 as u8);
        } else if let Some(tc) = ThaiChar::from_char(ch) {
            buffer.push(tc.to_tis620_byte());
        } else if let Some(replacement) = f(ch) {
            buffer.push(replacement.to_tis620_byte());
        }
    }
}

fn is_ascii(byte: u8) -> bool {
    byte <= 0x7F
}

/// Error that can occur while decoding.
#[derive(Debug)]
pub struct DecodeError(pub u8);

/// Error that can occur while encoding.
#[derive(Debug)]
pub struct EncodeError(pub char);

impl Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:#X?} is invalid TIS-620 byte.", self.0)
    }
}

impl Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} is invalid TIS-620 character.", self.0)
    }
}

impl Error for DecodeError {}

impl Error for EncodeError {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_test_case() {
        let test_cases = include_str!("./tis620-testcases.txt");
        for line in test_cases.lines() {
            if let Some((tis620_char_hex, utf8_char_hex)) = line.split_once(',') {
                let tis620_char_hex = tis620_char_hex.trim_start_matches("0x");
                let tis620_char_byte = hex::decode(tis620_char_hex).expect("TIS-620 byte");
                let str_from_tis620 =
                    decode(&tis620_char_byte).expect("string with single thai character");

                let utf8_u32 = u32::from_str_radix(utf8_char_hex.trim_start_matches("0x"), 16)
                    .expect("utf8 u32");
                let thai_char = std::char::from_u32(utf8_u32).expect("char");

                assert_eq!(str_from_tis620, thai_char.to_string());
            }
        }
    }

    #[ignore]
    #[test]
    fn display_error() {
        println!("display: {}", DecodeError(40));
        println!("display: {}", EncodeError('µ'));
    }
}
