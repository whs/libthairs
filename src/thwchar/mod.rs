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

use crate::cursor;
use encoding_rs::{DecoderResult, EncoderResult, WINDOWS_874};
use itertools::{Itertools, Position};
use libc::{c_char, c_int, size_t, wchar_t};
use std::ffi::CStr;
use std::io::{Cursor, Write};
use std::slice;

const REPLACEMENT: u16 = 0xFFFD; // unicode REPLACEMENT CHARACTER
const WC_ERR: wchar_t = wchar_t::MAX;
const TH_ERR: c_char = c_char::MAX;

/// thwchar module implements Thai <> UTF16 conversion with encoding_rs crate
/// It is not recommended to use this module in Rust

#[no_mangle]
pub unsafe extern "C" fn th_tis2uni(c: c_char) -> wchar_t {
    // TODO: Some characters in windows-874 are not valid in TIS
    th_winthai2uni(c)
}

#[no_mangle]
pub unsafe extern "C" fn th_winthai2uni(c: c_char) -> wchar_t {
    if c == TH_ERR {
        return WC_ERR;
    }

    let mut decoder = WINDOWS_874.new_decoder_without_bom_handling();
    let mut buf = [0u16; 1];
    let (out, _, _) = decoder.decode_to_utf16_without_replacement(&[c as u8], &mut buf, true);

    match out {
        DecoderResult::InputEmpty => buf[0] as wchar_t,
        DecoderResult::OutputFull => unreachable!(),
        DecoderResult::Malformed(_, _) => WC_ERR,
    }
}

#[no_mangle]
pub unsafe extern "C" fn th_macthai2uni(c: c_char) -> wchar_t {
    // TODO: Some characters in x-mac-thai are different from Windows-874
    th_winthai2uni(c)
}

#[no_mangle]
pub unsafe extern "C" fn th_uni2tis(wc: wchar_t) -> c_char {
    // TODO: Some characters in windows-874 are not valid in TIS
    th_uni2winthai(wc)
}

#[no_mangle]
pub unsafe extern "C" fn th_uni2winthai(wc: wchar_t) -> c_char {
    if wc == WC_ERR {
        return TH_ERR;
    }

    let mut encoder = WINDOWS_874.new_encoder();
    let mut buf = [0u8; 1];
    let (out, _, _) = encoder.encode_from_utf16_without_replacement(&[wc as u16], &mut buf, true);

    match out {
        EncoderResult::InputEmpty => buf[0] as c_char,
        EncoderResult::OutputFull => unreachable!(),
        EncoderResult::Unmappable(_) => TH_ERR,
    }
}

#[no_mangle]
pub unsafe extern "C" fn th_uni2macthai(wc: wchar_t) -> c_char {
    // TODO: Some characters in x-mac-thai are different from Windows-874
    th_uni2winthai(wc)
}

#[no_mangle]
pub unsafe extern "C" fn th_tis2uni_line(
    s: *const c_char,
    result: *mut wchar_t,
    n: size_t,
) -> c_int {
    let input_str = CStr::from_ptr(s);
    let mut result_slice = slice::from_raw_parts_mut(result, n);
    let mut decoder = WINDOWS_874.new_decoder_without_bom_handling();
    let mut cur = cursor::Cursor::new(result_slice);

    let mut output_buf = [0u16; 2];

    for input in input_str.to_bytes().iter().with_position() {
        let last = match input {
            Position::Last(_) => true,
            Position::Only(_) => true,
            _ => false,
        };
        let i = *input.into_inner();
        if i == TH_ERR as u8 {
            cur.write(WC_ERR);
            continue;
        }
        let (result, _, out_len) =
            decoder.decode_to_utf16_without_replacement(&[i], &mut output_buf, last);
        match result {
            DecoderResult::InputEmpty => {
                for item in &output_buf[..out_len] {
                    cur.write(*item as wchar_t);
                }
            }
            DecoderResult::OutputFull => unreachable!(),
            DecoderResult::Malformed(_, _) => {
                cur.write(WC_ERR);
            }
        }
    }
    let sub_null_bytes = cur.write(0);

    (cur.position() - sub_null_bytes) as c_int
}

#[no_mangle]
pub unsafe extern "C" fn th_uni2tis_line(
    s: *const wchar_t,
    result: *mut c_char,
    n: size_t,
) -> c_int {
    let input_len = wchar_len(s);
    let input_str = slice::from_raw_parts(s, input_len);
    let (pre, result_slice, post) = slice::from_raw_parts_mut(result, n).align_to_mut::<u8>();
    debug_assert!(pre.is_empty());
    debug_assert!(post.is_empty());
    let mut cur = Cursor::new(result_slice);
    let mut encoder = WINDOWS_874.new_encoder();

    let mut output_buf = [0u8; 4];
    for input in input_str.iter().with_position() {
        let last = match input {
            Position::Last(_) => true,
            Position::Only(_) => true,
            _ => false,
        };
        let i = *input.into_inner();
        if i == WC_ERR {
            let _ = cur.write(&[TH_ERR as u8]);
            continue;
        }
        let input_u16: u16 = match (i).try_into() {
            Ok(v) => v,
            Err(_) => {
                let _ = cur.write(&[TH_ERR as u8]);
                continue;
            }
        };
        let (result, _, out_len) =
            encoder.encode_from_utf16_without_replacement(&[input_u16], &mut output_buf, last);
        let _ = match result {
            EncoderResult::InputEmpty => cur.write(&output_buf[..out_len]),
            EncoderResult::OutputFull => unreachable!(),
            EncoderResult::Unmappable(_) => cur.write(&[TH_ERR as u8]),
        };
    }

    let sub_null_byte = match cur.write(&[0]) {
        Ok(s) => s as u64,
        Err(_) => 0,
    };

    (cur.position() - sub_null_byte) as c_int
}

/// Get the size of NULL terminated *wchar_t
unsafe fn wchar_len(s: *const wchar_t) -> usize {
    let mut cur = s;
    for i in 0usize.. {
        if *cur == 0 {
            return i;
        }
        cur = cur.add(1usize);
    }
    unreachable!()
}
