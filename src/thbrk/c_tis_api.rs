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

use crate::thbrk::{DatrieBrk, TisBreaker};
use crate::utils;
use itertools::Itertools;
use libc::{c_char, c_int, c_uchar};
use std::ffi::{CStr, OsStr};
use std::io::{Cursor, Write};
use std::path::Path;
use std::ptr::null_mut;
use std::slice;

pub type DefaultBreaker = DatrieBrk;

/**
 * @brief  Create a dictionary-based word breaker
 *
 * @param  dictpath : the dictionary path, or NULL for default
 *
 * @return  the created instance, or NULL on failure
 *
 * Loads the dictionary from the given file and returns the created word
 * breaker. If @a dictpath is NULL, first searches in the directory given
 * by the LIBTHAI_DICTDIR environment variable, then in the library
 * installation directory. Returns NULL if the dictionary file is not
 * found or cannot be loaded.
 *
 * The returned ThBrk object should be destroyed after use using
 * th_brk_delete().
 *
 * In multi-thread environments, th_brk_new() and th_brk_delete()
 * should be used to create and destroy a word breaker instance inside
 * critical sections (i.e. with mutex). And the word breaker methods
 * can then be safely called in parallel during its lifetime.
 *
 * (Available since version 0.1.25, libthai.so.0.3.0)
 */
#[no_mangle]
pub unsafe extern "C" fn th_brk_new(dictpath: *const c_char) -> *mut DefaultBreaker {
    let path = dictpath.as_ref().map(|v| {
        let path_str = CStr::from_ptr(v);
        #[cfg(unix)]
        {
            use std::os::unix::ffi::OsStrExt;
            OsStr::from_bytes(path_str.to_bytes())
        }
        // TODO: Confirm that the Windows version do this
        #[cfg(windows)]
        {
            use std::os::windows::prelude::*;
            OsString::from_wide(path_str.to_bytes())
        }
    });
    let brk = DefaultBreaker::new(path.map(|v| Path::new(v)));
    match brk {
        Ok(brk) => Box::into_raw(Box::new(brk)),
        Err(e) => {
            println!("Init error: {}", e);
            null_mut()
        }
    }
}

/**
 * @brief  Delete a word breaker
 *
 * @param  brk : the word breaker
 *
 * Frees memory associated with the word breaker.
 */
#[no_mangle]
pub unsafe extern "C" fn th_brk_delete(brk: *mut DefaultBreaker) {
    drop(Box::from_raw(brk));
}

/**
 * @brief  Find word break positions in Thai string
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
 */
#[no_mangle]
pub unsafe extern "C" fn th_brk_find_breaks(
    brk: &mut DefaultBreaker,
    s: *const c_uchar,
    pos: *const c_int,
    pos_sz: libc::size_t,
) -> c_int {
    if s.is_null() || pos.is_null() {
        return 0;
    }

    let input = slice::from_raw_parts(s, utils::uchar_len(s));

    let out = brk.find_breaks_tis(input, pos_sz); // TODO: Optimize
    let pos = slice::from_raw_parts_mut(pos as *mut i32, pos_sz);
    pos[..out.len()].copy_from_slice(
        &out.iter()
            .take(pos_sz)
            .map(|i| *i as i32)
            .collect::<Vec<i32>>(),
    );
    out.len() as i32
}

/**
 * @brief  Insert word delimiters in given string
 *
 * @param  brk : the word breaker
 * @param  in  : the input string to be processed
 * @param  out : the output buffer
 * @param  out_sz : the size of @a out
 * @param  delim  : the word delimiter to insert
 *
 * @return  the actual size of the processed string
 *
 * Analyzes the input string and store the string in output buffer
 * with the given word delimitor inserted at every word boundary.
 */
#[no_mangle]
pub unsafe extern "C" fn th_brk_insert_breaks(
    brk: &mut DefaultBreaker,
    s: *const c_uchar,
    out: *mut c_uchar,
    out_sz: libc::size_t,
    delim: *const c_char,
) -> c_int {
    let input = slice::from_raw_parts(s, utils::uchar_len(s));
    let delim_s = CStr::from_ptr(delim).to_bytes();
    let out = slice::from_raw_parts_mut(out as *mut u8, out_sz);
    let mut cur = Cursor::new(out);

    // TODO: Use builtin intersperse (rust#79524)
    let pieces = Itertools::intersperse(brk.split_tis(input).into_iter(), delim_s);
    for piece in pieces {
        match cur.write(piece) {
            Ok(v) if v == piece.len() => {}
            _ => break,
        }
    }
    let _ = cur.write(&[0]);
    // ensure the last character is null terminated
    let last_pos = cur.position();
    cur.get_mut()[last_pos as usize] = 0;

    last_pos as c_int
}
