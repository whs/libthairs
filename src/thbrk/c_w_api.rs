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

use crate::thbrk::c_tis_api::DefaultBreaker;
use crate::thbrk::datrie::SHARED_BRK;
use crate::thbrk::StrBreaker;
use crate::utils;
use itertools::Itertools;
use libc::{c_int, size_t, wchar_t};
use std::slice;

/**
 * @brief  Find word break positions in Thai wide-char string
 *
 * @param  brk : the word breaker
 * @param  s   : the input string to be processed
 * @param  pos : array to keep breaking positions
 * @param  pos_sz : size of @a pos[]
 *
 * @return  the actual number of breaking positions occurred
 *
 * Finds word break positions in Thai string @a s and stores at most @a pos_sz
 * breaking positions in @a pos[], from left to right.
 *
 * (Available since version 0.1.25, libthai.so.0.3.0)
 */
#[no_mangle]
pub unsafe extern "C" fn th_brk_wc_find_breaks(
    brk: &DefaultBreaker,
    s: *const wchar_t,
    pos: *mut c_int,
    pos_sz: size_t,
) -> c_int {
    if s.is_null() || pos.is_null() || pos_sz == 0 {
        return 0;
    }

    let input = slice::from_raw_parts(s, utils::wchar_len(s));
    let input_str_opt = input
        .iter()
        .copied()
        .map(|i| char::from_u32(i as u32))
        .collect::<Option<String>>();
    let input_str = match input_str_opt {
        Some(v) => v,
        None => return 0,
    };

    let out = brk.find_breaks(&input_str, pos_sz);
    let out_len = out.len();
    let pos = slice::from_raw_parts_mut(pos as *mut i32, pos_sz);
    for (idx, v) in out.into_iter().take(pos_sz).enumerate() {
        pos[idx] = v as i32
    }
    out_len as i32
}

#[no_mangle]
#[deprecated = "Use th_brk_wc_find_breaks"]
pub unsafe extern "C" fn th_wbrk(s: *const wchar_t, pos: *mut c_int, pos_sz: size_t) -> c_int {
    th_brk_wc_find_breaks(&SHARED_BRK, s, pos, pos_sz)
}

/**
 * @brief  Insert word delimitors in given wide-char string
 *
 * @param  brk : the word breaker
 * @param  in  : the input wide-char string to be processed
 * @param  out : the output wide-char buffer
 * @param  out_sz : the size of @a out (as number of elements)
 * @param  delim : the wide-char word delimitor to insert
 *
 * @return  the actual size of the processed string (as number of elements)
 *
 * Analyzes the input string and store the string in output buffer
 * with the given word delimitor inserted at every word boundary.
 *
 * (Available since version 0.1.25, libthai.so.0.3.0)
 */
#[no_mangle]
pub unsafe extern "C" fn th_brk_wc_insert_breaks(
    brk: &DefaultBreaker,
    s: *const wchar_t,
    out: *mut wchar_t,
    out_sz: size_t,
    delim: *const wchar_t,
) -> c_int {
    if s.is_null() || out.is_null() || delim.is_null() || out_sz == 0 {
        return 0;
    }

    let input = slice::from_raw_parts(s, utils::wchar_len(s));
    let input_str_opt = input
        .iter()
        .copied()
        .map(|i| char::from_u32(i as u32))
        .collect::<Option<String>>();
    let input_str = match input_str_opt {
        Some(v) => v,
        None => return 0,
    };
    let delim_slice = slice::from_raw_parts(delim, utils::wchar_len(delim));
    let delim_str_opt = delim_slice
        .iter()
        .copied()
        .map(|i| char::from_u32(i as u32))
        .collect::<Option<String>>();
    let delim_str = match delim_str_opt {
        Some(v) => v,
        None => return 0,
    };
    let out = slice::from_raw_parts_mut(out, out_sz);

    let out_str = Itertools::intersperse(brk.split(&input_str).into_iter(), &delim_str);
    let mut pos = 0;
    for piece in out_str {
        for ch in piece.chars() {
            out[pos] = ch as i32;
            pos += 1;
        }
    }
    out[pos] = 0;

    pos as c_int
}

#[no_mangle]
#[deprecated = "Use th_brk_wc_insert_breaks"]
pub unsafe extern "C" fn th_wbrk_line(
    input: *const wchar_t,
    out: *mut wchar_t,
    out_sz: size_t,
    delim: *const wchar_t,
) -> c_int {
    th_brk_wc_insert_breaks(&SHARED_BRK, input, out, out_sz, delim)
}
