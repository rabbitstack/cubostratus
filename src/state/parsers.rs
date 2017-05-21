//! Contains useful [nom](https://github.com/Geal/nom) parsers.

use std::path::PathBuf;
use nom::not_line_ending;
use std::str::{self, FromStr};
use std::borrow::ToOwned;
use nom::{digit, alphanumeric};

named!(pub consume_until_line_ending, take_until_and_consume!("\n"));

named!(pub parse_line<String>,
       map!(map_res!(not_line_ending, str::from_utf8), ToOwned::to_owned));

named!(pub parse_u32_octal<u32>,
       map_res!(map_res!(alphanumeric, str::from_utf8),
                |s| u32::from_str_radix(s, 8)));

named!(pub parse_u8<u8>,
       map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

named!(pub parse_u32<u32>,
       map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));

named!(pub parse_u64<u64>,
       map_res!(map_res!(digit, str::from_utf8), FromStr::from_str));
