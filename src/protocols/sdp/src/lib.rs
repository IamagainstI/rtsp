use std::{io::{Error, ErrorKind}, net::AddrParseError};

use abstractions::extensions::utf8_array_extensions::U8ArrayExt;

pub mod media_session;
pub mod bandwidth;
pub mod origin;
pub mod media_attribute;
pub mod media_description;
pub mod data_transfer_mode;
pub mod payload_type;
pub mod network_type;
pub mod address_type;
pub mod time;

const RAW_SEPARATOR: u8 = '\n' as u8;
const KEY_VALUE_SEPARATOR: u8 = '=' as u8;
const TRIM: u8 = ' ' as u8;
const TRIM_REF: &u8 = &TRIM;