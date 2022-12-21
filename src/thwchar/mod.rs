////////////////////////////////////////////////////////////////////////////////
// Copyright (C) 2022 Manatsawin Hanmongkolchai
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Lesser General Public
// License as published by the Free Software Foundation; either
// version 2.1 of the License, or (at your option) any later version.
//
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public
// License along with this library; if not, write to the Free Software
// Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA  02110-1301  USA
////////////////////////////////////////////////////////////////////////////////

/// thwchar module implements Thai <> UTF16 conversion
mod c_api;
mod data;

use crate::thwchar::data::*;

macro_rules! u8_convert {
    ( $name:ident, $table:ident ) => {
        pub const fn $name(c: u8) -> Option<char> {
            match c {
                0..=0x7f => Some(c as char),
                _ => match $table[(c - 0x80) as usize] {
                    WC_ERR => None,
                    v => Some(v),
                },
            }
        }
    };
}

u8_convert!(tis2uni, TIS2UNI_TABLE);
u8_convert!(winthai2uni, WINTHAI2UNI_TABLE);
u8_convert!(macthai2uni, MACTHAI2UNI_TABLE);

pub const fn uni2tis(wc: char) -> Option<u8> {
    match wc as u32 {
        0..=0x007f => Some(wc as u8),
        0x0e00..=0x0e5f => match UNI2TIS_TABLE[((wc as u32) - 0x0e00) as usize] {
            TH_ERR => None,
            v => Some(v),
        },
        _ => None,
    }
}

macro_rules! reverse_table {
    ( $name:ident, $table:ident ) => {
        pub fn $name(wc: char) -> Option<u8> {
            match uni2tis(wc) {
                Some(v) => Some(v),
                None => {
                    let searched_ch = $table.iter().copied().enumerate().find(|v| v.1 == wc);
                    match searched_ch {
                        Some(v) => Some((v.0 as u8) + 0x80),
                        None => None,
                    }
                }
            }
        }
    };
}

reverse_table!(uni2winthai, WINTHAI2UNI_TABLE);
reverse_table!(uni2macthai, MACTHAI2UNI_TABLE);

macro_rules! encoding_to_string {
    ($name:ident, $touni:ident) => {
        /// Decode a byte slice to a new String. Any invalid characters will be replaced with \u{FFFD}
        pub fn $name(input: &[u8]) -> String {
            input
                .iter()
                .map(|c| match $touni(*c) {
                    Some(ch) => ch,
                    None => '\u{FFFD}',
                })
                .collect()
        }
    };
}

encoding_to_string!(tis2string, tis2uni);
encoding_to_string!(winthai2string, winthai2uni);
encoding_to_string!(macthai2string, macthai2uni);

macro_rules! string_to_encoding {
    ($name:ident, $toenc:ident, $err:expr) => {
        /// Encode a string to a byte slice
        pub fn $name(s: &str) -> Vec<u8> {
            s.chars().map(|c| $toenc(c).unwrap_or($err)).collect()
        }
    };
}

string_to_encoding!(str2tis, uni2tis, u8::MAX);
string_to_encoding!(str2winthai, uni2winthai, u8::MAX);
string_to_encoding!(str2macthai, uni2macthai, u8::MAX);
