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

use crate::thwchar::{tis2uni, uni2tis, uni2winthai, winthai2uni};
use crate::{macthai2uni, uni2macthai, utils};
use libc::{c_int, c_uchar, size_t, wchar_t};
use std::slice;

#[no_mangle]
unsafe extern "C" fn th_tis2uni(c: c_uchar) -> wchar_t {
    tis2uni(c).map(|v| v as wchar_t).unwrap_or(wchar_t::MAX)
}

#[no_mangle]
pub unsafe extern "C" fn th_winthai2uni(c: c_uchar) -> wchar_t {
    winthai2uni(c).map(|v| v as wchar_t).unwrap_or(wchar_t::MAX)
}

#[no_mangle]
unsafe extern "C" fn th_macthai2uni(c: c_uchar) -> wchar_t {
    macthai2uni(c).map(|v| v as wchar_t).unwrap_or(wchar_t::MAX)
}

#[no_mangle]
unsafe extern "C" fn th_uni2tis(wc: wchar_t) -> c_uchar {
    let ch = match char::from_u32(wc as u32) {
        Some(c) => c,
        None => return c_uchar::MAX,
    };
    uni2tis(ch).unwrap_or(c_uchar::MAX)
}

#[no_mangle]
unsafe extern "C" fn th_uni2winthai(wc: wchar_t) -> c_uchar {
    let ch = match char::from_u32(wc as u32) {
        Some(c) => c,
        None => return c_uchar::MAX,
    };
    uni2winthai(ch).unwrap_or(c_uchar::MAX)
}

#[no_mangle]
unsafe extern "C" fn th_uni2macthai(wc: wchar_t) -> c_uchar {
    let ch = match char::from_u32(wc as u32) {
        Some(c) => c,
        None => return c_uchar::MAX,
    };
    uni2macthai(ch).unwrap_or(c_uchar::MAX)
}

#[no_mangle]
unsafe extern "C" fn th_tis2uni_line(s: *const c_uchar, result: *mut wchar_t, n: size_t) -> c_int {
    if s.is_null() || result.is_null() || n == 0 {
        return 0;
    }

    let input_len = utils::uchar_len(s);
    let output_len = input_len.min(n);
    let input_str = slice::from_raw_parts(s, output_len);
    let result_slice = slice::from_raw_parts_mut(result, n);

    for (idx, item) in input_str.iter().enumerate() {
        result_slice[idx] = th_tis2uni(*item);
    }
    let last_pos = output_len.min(n - 1);
    result_slice[last_pos] = 0;
    last_pos as c_int
}

#[no_mangle]
unsafe extern "C" fn th_uni2tis_line(s: *const wchar_t, result: *mut c_uchar, n: size_t) -> c_int {
    if s.is_null() || result.is_null() || n == 0 {
        return 0;
    }

    let input_len = utils::wchar_len(s);
    let output_len = input_len.min(n);
    let input_str = slice::from_raw_parts(s, output_len);
    let result_slice = slice::from_raw_parts_mut(result, n);

    for (idx, item) in input_str.iter().enumerate() {
        result_slice[idx] = th_uni2tis(*item);
    }
    let last_pos = output_len.min(n - 1);
    result_slice[last_pos] = 0;
    last_pos as c_int
}

#[cfg(test)]
mod tests {
    use crate::thwchar::c_api::*;
    use std::ptr::null_mut;

    const MAXLINELENGTH: usize = 100;
    const TIS_SAMPLE: &[u8] = include_bytes!("test_tis_sample.txt");
    const WIN_SAMPLE: &[u8] = include_bytes!("test_win_sample.txt");
    const MAC_SAMPLE: &[u8] = include_bytes!("test_mac_sample.txt");

    #[test]
    fn convert_line_invalids() {
        let uni_len = unsafe { th_tis2uni_line(TIS_SAMPLE.as_ptr(), null_mut(), 0) };
        assert_eq!(uni_len, 0);

        let mut utf = [i32::MAX; 10];
        let uni_len = unsafe { th_tis2uni_line([0].as_ptr(), utf.as_mut_ptr(), 10) };
        assert_eq!(uni_len, 0);

        let tis_len = unsafe { th_uni2tis_line(utf.as_ptr(), null_mut(), 0) };
        assert_eq!(tis_len, 0);

        let mut tis = [u8::MAX, 10];
        let tis_len = unsafe { th_uni2tis_line([0].as_ptr(), tis.as_mut_ptr(), 10) };
        assert_eq!(tis_len, 0);
    }

    #[test]
    fn convert_line_exact_buf() {
        const str_len: usize = TIS_SAMPLE.len();
        let mut uni = [i32::MAX; str_len];
        let uni_len = unsafe { th_tis2uni_line(TIS_SAMPLE.as_ptr(), uni.as_mut_ptr(), str_len) };
        assert_eq!(uni_len as usize, str_len - 1);

        let mut tis = [u8::MAX; str_len];
        let tis_len = unsafe { th_uni2tis_line(uni.as_ptr(), tis.as_mut_ptr(), str_len) };
        assert_eq!(tis_len, uni_len);

        assert_eq!(TIS_SAMPLE, &tis);
    }

    #[test]
    fn convert_line_bigger_buf() {
        let mut uni = [i32::MAX; MAXLINELENGTH];
        let uni_len =
            unsafe { th_tis2uni_line(TIS_SAMPLE.as_ptr(), uni.as_mut_ptr(), MAXLINELENGTH) };
        assert_eq!(uni_len as usize, TIS_SAMPLE.len() - 1);

        let mut tis = [u8::MAX; MAXLINELENGTH];
        let tis_len = unsafe { th_uni2tis_line(uni.as_ptr(), tis.as_mut_ptr(), MAXLINELENGTH) };
        assert_eq!(tis_len, uni_len);

        assert_eq!(TIS_SAMPLE, &tis[..(tis_len as usize) + 1]);
    }

    #[test]
    fn convert_line_smaller_buf() {
        let mut uni = [i32::MAX; 10];
        let uni_len = unsafe { th_tis2uni_line(TIS_SAMPLE.as_ptr(), uni.as_mut_ptr(), 10) };
        assert_eq!(uni_len, 9);
        assert_eq!(uni[9], 0);

        let mut tis = [u8::MAX; 10];
        let tis_len = unsafe { th_uni2tis_line(uni.as_ptr(), tis.as_mut_ptr(), 10) };
        assert_eq!(tis_len, 9);

        assert_eq!(&TIS_SAMPLE[0..9], &tis[0..9]);
        assert_eq!(tis[9], 0);
    }

    #[test]
    fn test_winthai() {
        for ch in WIN_SAMPLE {
            let uni_ch = unsafe { th_winthai2uni(*ch) };
            let rev_ch = unsafe { th_uni2winthai(uni_ch) };
            assert_eq!(*ch, rev_ch, "Inconsistent uni<->winthai conv");
        }
    }

    #[test]
    fn test_macthai() {
        for ch in MAC_SAMPLE {
            let uni_ch = unsafe { th_macthai2uni(*ch) };
            let rev_ch = unsafe { th_uni2macthai(uni_ch) };
            assert_eq!(*ch, rev_ch, "Inconsistent uni<->macthai conv");
        }
    }
}
