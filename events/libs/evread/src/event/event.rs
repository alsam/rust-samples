use ::safer_ffi::prelude::*;
use serde::de::{Deserialize, Deserializer};
use std::error::Error;

pub fn deserialize_hex_str<'de, D>(deserializer: D) -> Result<Vec<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let buf = String::deserialize(deserializer)?;
    let bchar = |i| buf.chars().nth(i).unwrap();
    let (first_char, second_char) = (bchar(0), bchar(1));

    if first_char == '0' && (second_char == 'x' || second_char == 'X') {
        let mut vals: Vec<u64> = buf[2..]
            .split(':')
            .map(|s| u64::from_str_radix(&s, 16).unwrap())
            .collect();
        vals.shrink_to_fit();
        Ok(vals)
    } else {
        Err("hex is not prefixed with 0x or 0X").map_err(serde::de::Error::custom)
    }
}
