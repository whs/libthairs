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

use encoding_rs::{EncoderResult, WINDOWS_874};
use libc::{c_uchar, wchar_t};

/// Get the size of NULL terminated *wchar_t
pub unsafe fn wchar_len(s: *const wchar_t) -> usize {
    let mut cur = s;
    for i in 0usize.. {
        if *cur == 0 {
            return i;
        }
        cur = cur.add(1usize);
    }
    unreachable!()
}

/// Get the size of NULL terminated *uchar_t
pub unsafe fn uchar_len(s: *const c_uchar) -> usize {
    let mut cur = s;
    for i in 0usize.. {
        if *cur == 0 {
            return i;
        }
        cur = cur.add(1usize);
    }
    unreachable!()
}

pub fn as_str(s: &[char]) -> String {
    let len = s.iter().map(|v| v.len_utf8()).sum();
    let mut buf = vec![0; len];
    let mut cur: &mut [u8] = &mut buf;

    for c in s {
        let c_len = c.len_utf8();
        c.encode_utf8(cur);
        cur = &mut cur[c_len..];
    }

    unsafe { String::from_utf8_unchecked(buf) }
}

/// Encode a UTF-8 string to Windows-874, replacing any invalid characters with err_replacement
pub fn to_windows874(txt: &str, err_replacement: u8) -> Vec<u8> {
    let mut encoder = WINDOWS_874.new_encoder();
    let mut out = vec![
        0;
        encoder
            .max_buffer_length_from_utf8_if_no_unmappables(txt.len())
            .unwrap()
    ];
    let mut cur_in = txt;
    let mut cur_out = out.as_mut_slice();
    let mut out_len = 0;
    while !cur_in.is_empty() {
        let (res, isize, osize) =
            encoder.encode_from_utf8_without_replacement(cur_in, cur_out, true);
        cur_in = &cur_in[isize..];
        cur_out = &mut cur_out[osize..];
        out_len += osize;

        match res {
            EncoderResult::InputEmpty => break,
            EncoderResult::OutputFull => unreachable!(),
            EncoderResult::Unmappable(_) => {
                cur_out[0] = err_replacement;
                cur_out = &mut cur_out[1..];
                out_len += 1;
            }
        }
    }
    out.truncate(out_len);
    debug_assert_eq!(out_len, txt.len());

    out
}
