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

use crate::thbrk::datrie::default_breaker;
use crate::thbrk::{DatrieBrk, TisBreaker};
use crate::utils;
use itertools::Itertools;
use libc::{c_char, c_int, c_uchar};
use std::ffi::{CStr, OsStr};
use std::io::{Cursor, Write};
use std::path::Path;
use std::ptr::null_mut;
use std::slice;

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
pub unsafe extern "C" fn th_brk_new(dictpath: *const c_char) -> *mut DatrieBrk {
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
    let brk = match path {
        Some(path) => DatrieBrk::from_datrie_path(Path::new(path)),
        None => default_breaker(),
    };
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
pub unsafe extern "C" fn th_brk_delete(brk: *mut DatrieBrk) {
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
    brk: &DatrieBrk,
    s: *const c_uchar,
    pos: *const c_int,
    pos_sz: libc::size_t,
) -> c_int {
    if s.is_null() || pos.is_null() || pos_sz == 0 {
        return 0;
    }

    let input = slice::from_raw_parts(s, utils::uchar_len(s));

    let out = brk.find_breaks_tis(input, pos_sz);
    let out_len = out.len();
    let pos = slice::from_raw_parts_mut(pos as *mut i32, pos_sz);
    for (idx, v) in out.into_iter().take(pos_sz).enumerate() {
        pos[idx] = v as i32
    }
    (out_len as c_int).min(pos_sz as c_int)
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
    brk: &DatrieBrk,
    s: *const c_uchar,
    out: *mut c_uchar,
    out_sz: libc::size_t,
    delim: *const c_char,
) -> c_int {
    if s.is_null() || out.is_null() || delim.is_null() || out_sz == 0 {
        return 0;
    }

    let input = slice::from_raw_parts(s, utils::uchar_len(s));
    let delim_s = slice::from_raw_parts(delim as *const u8, libc::strlen(delim));
    let out = slice::from_raw_parts_mut(out, out_sz);
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
    let out = cur.into_inner();
    out[(last_pos as usize).min(out.len() - 1)] = 0;

    (last_pos - 1) as c_int
}

#[cfg(test)]
mod tests {
    use crate::thbrk::c_tis_api::{
        th_brk_delete, th_brk_find_breaks, th_brk_insert_breaks, th_brk_new,
    };
    use crate::thbrk::test::{test_thbrk, TEST_SAMPLES};
    use crate::{thwchar, DatrieBrk, TisBreaker};
    use itertools::Itertools;
    use libc::{c_int, c_uchar};
    use std::ffi::CString;
    use std::ptr::null;

    struct CBreaker {
        brk: *mut DatrieBrk,
    }

    impl CBreaker {
        fn new() -> CBreaker {
            let brk = unsafe { th_brk_new(null()) };
            CBreaker { brk }
        }
        fn as_ptr(&self) -> &DatrieBrk {
            unsafe { &*self.brk as &DatrieBrk }
        }
    }

    impl TisBreaker for CBreaker {
        fn find_breaks_tis<'a>(&'a self, input: &'a [u8], max_out: usize) -> Vec<usize> {
            let mut out = vec![c_int::MAX; max_out];
            let mut input_null_terminated = Vec::from(input);
            input_null_terminated.push(0);
            let count = unsafe {
                th_brk_find_breaks(
                    self.as_ptr(),
                    input_null_terminated.as_ptr(),
                    out.as_mut_ptr(),
                    max_out,
                )
            } as usize;

            out.into_iter()
                .take(count)
                .map(|item| item as usize)
                .collect_vec()
        }
    }

    impl Drop for CBreaker {
        fn drop(&mut self) {
            unsafe { th_brk_delete(self.brk) }
        }
    }

    #[test]
    fn find_breaks() {
        let breaker = CBreaker::new();
        test_thbrk(&breaker);
    }

    #[test]
    fn find_breaks_zero() {
        let breaker = CBreaker::new();
        for case in TEST_SAMPLES.iter() {
            let mut input_tis = thwchar::str2tis(&case.txt);
        }
    }

    #[test]
    fn find_breaks_under_buffer() {
        let breaker = CBreaker::new();
        for case in TEST_SAMPLES.iter() {
            let mut input_tis = thwchar::str2tis(&case.txt);
            input_tis.push(0);
            let mut out = vec![c_int::MAX; 5];
            let out_len = unsafe {
                th_brk_find_breaks(
                    breaker.as_ptr(),
                    input_tis.as_ptr(),
                    out.as_mut_ptr(),
                    out.len(),
                )
            } as usize;
            assert_eq!(out_len, case.brk_pos.len().min(5));
            assert_eq!(
                out.into_iter()
                    .take(out_len)
                    .map(|v| v as usize)
                    .collect_vec(),
                case.brk_pos.iter().copied().take(out_len).collect_vec(),
            );
        }
    }

    #[test]
    fn find_breaks_exact_buffer() {
        let breaker = CBreaker::new();
        for case in TEST_SAMPLES.iter() {
            let mut input_tis = thwchar::str2tis(&case.txt);
            input_tis.push(0);
            let mut out = vec![c_int::MAX; case.brk_pos.len()];
            let out_len = unsafe {
                th_brk_find_breaks(
                    breaker.as_ptr(),
                    input_tis.as_ptr(),
                    out.as_mut_ptr(),
                    out.len(),
                )
            } as usize;
            assert_eq!(out_len, case.brk_pos.len());
            assert_eq!(
                out.into_iter().map(|v| v as usize).collect_vec(),
                case.brk_pos,
            );
        }
    }

    #[test]
    fn insert_breaks() {
        let breaker = CBreaker::new();
        let delim = CString::new("|").unwrap();
        let mut output_buf = Vec::new();

        for case in TEST_SAMPLES.iter() {
            let mut input_tis = thwchar::str2tis(&case.txt);
            let mut output_tis = thwchar::str2tis(&case.ins_str);
            input_tis.push(0);
            output_tis.push(0);

            output_buf.resize(input_tis.len() * 2, c_uchar::MAX);

            let len = unsafe {
                th_brk_insert_breaks(
                    breaker.as_ptr(),
                    input_tis.as_ptr(),
                    output_buf.as_mut_ptr(),
                    output_buf.len(),
                    delim.as_ptr(),
                )
            } as usize;
            output_buf.truncate(len + 1);

            assert_eq!(output_tis, output_buf);
        }
    }

    #[test]
    fn insert_breaks_under_buffer() {
        let breaker = CBreaker::new();
        let delim = CString::new("|").unwrap();
        let mut output_buf = vec![c_uchar::MAX; 5];

        for case in TEST_SAMPLES.iter() {
            let mut input_tis = thwchar::str2tis(&case.txt);
            input_tis.push(0);

            let mut output_tis = thwchar::str2tis(&case.ins_str);
            output_tis.truncate(5);
            if output_tis.len() == 5 {
                output_tis[4] = 0;
            } else {
                output_tis.push(0);
            }
            assert!(output_tis.len() <= 5);

            output_buf.fill(c_uchar::MAX);

            let len = unsafe {
                th_brk_insert_breaks(
                    breaker.as_ptr(),
                    input_tis.as_ptr(),
                    output_buf.as_mut_ptr(),
                    output_buf.len(),
                    delim.as_ptr(),
                )
            } as usize;
            assert_eq!(len, output_tis.len() - 1);

            assert_eq!(output_tis, &output_buf[0..(len + 1).min(output_buf.len())]);
        }
    }

    #[test]
    fn insert_breaks_exact_buffer() {
        let breaker = CBreaker::new();
        let delim = CString::new("|").unwrap();
        let mut output_buf = Vec::new();

        for case in TEST_SAMPLES.iter() {
            let mut input_tis = thwchar::str2tis(&case.txt);
            input_tis.push(0);

            let mut output_tis = thwchar::str2tis(&case.ins_str);
            output_tis.push(0);

            output_buf.resize(output_tis.len(), c_uchar::MAX);

            let len = unsafe {
                th_brk_insert_breaks(
                    breaker.as_ptr(),
                    input_tis.as_ptr(),
                    output_buf.as_mut_ptr(),
                    output_buf.len(),
                    delim.as_ptr(),
                )
            } as usize;
            assert_eq!(len, output_tis.len() - 1);
            output_buf.truncate(len + 1);

            assert_eq!(&output_tis, &output_buf);
        }
    }
}
