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

use libc::{c_uchar, wchar_t};

/// Get the size of NULL terminated *wchar_t
#[inline]
pub unsafe fn wchar_len(s: *const wchar_t) -> usize {
    libc::wcslen(s) as usize
}

/// Get the size of NULL terminated *uchar_t
#[inline]
pub unsafe fn uchar_len(s: *const c_uchar) -> usize {
    libc::strlen(s.cast()) as usize
}

pub fn as_str(s: &[char]) -> String {
    let mut out = String::new();
    as_str_buf(s, &mut out);
    out
}

pub fn as_str_buf(s: &[char], out: &mut String) {
    out.clear();
    out.extend(s);
}

pub fn len_utf8(s: &[char]) -> usize {
    s.iter().map(|c| c.len_utf8()).sum()
}

pub fn chars_as_bytes(s: &[char], out: &mut Vec<u8>) {
    out.clear();
    out.resize(len_utf8(s), 0);
    let mut cur: &mut [u8] = out;

    for ch in s {
        let encoded_len = {
            let encode_out = ch.encode_utf8(cur);
            encode_out.len()
        };
        cur = &mut cur[encoded_len..];
    }
}
