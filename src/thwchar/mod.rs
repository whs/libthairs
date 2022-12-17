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

use std::slice;

use libc::{c_int, c_uchar, size_t, wchar_t};

use crate::utils;

const WC_ERR: wchar_t = wchar_t::MAX;
const TH_ERR: c_uchar = c_uchar::MAX;

/// thwchar module implements Thai <> UTF16 conversion
///
/// It is not recommended to use this module in Rust as it has nonstandard handling of invalid
/// codepoints.

const TIS2UNI_TABLE: [i32; 128] = [
    WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR,
    WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR,
    WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, 0x0e01, 0x0e02, 0x0e03,
    0x0e04, 0x0e05, 0x0e06, 0x0e07, 0x0e08, 0x0e09, 0x0e0a, 0x0e0b, 0x0e0c, 0x0e0d, 0x0e0e, 0x0e0f,
    0x0e10, 0x0e11, 0x0e12, 0x0e13, 0x0e14, 0x0e15, 0x0e16, 0x0e17, 0x0e18, 0x0e19, 0x0e1a, 0x0e1b,
    0x0e1c, 0x0e1d, 0x0e1e, 0x0e1f, 0x0e20, 0x0e21, 0x0e22, 0x0e23, 0x0e24, 0x0e25, 0x0e26, 0x0e27,
    0x0e28, 0x0e29, 0x0e2a, 0x0e2b, 0x0e2c, 0x0e2d, 0x0e2e, 0x0e2f, 0x0e30, 0x0e31, 0x0e32, 0x0e33,
    0x0e34, 0x0e35, 0x0e36, 0x0e37, 0x0e38, 0x0e39, 0x0e3a, WC_ERR, WC_ERR, WC_ERR, WC_ERR, 0x0e3f,
    0x0e40, 0x0e41, 0x0e42, 0x0e43, 0x0e44, 0x0e45, 0x0e46, 0x0e47, 0x0e48, 0x0e49, 0x0e4a, 0x0e4b,
    0x0e4c, 0x0e4d, 0x0e4e, 0x0e4f, 0x0e50, 0x0e51, 0x0e52, 0x0e53, 0x0e54, 0x0e55, 0x0e56, 0x0e57,
    0x0e58, 0x0e59, 0x0e5a, 0x0e5b, WC_ERR, WC_ERR, WC_ERR, WC_ERR,
];

const UNI2TIS_TABLE: [u8; 96] = [
    TH_ERR, 0xa1, 0xa2, 0xa3, 0xa4, 0xa5, 0xa6, 0xa7, 0xa8, 0xa9, 0xaa, 0xab, 0xac, 0xad, 0xae,
    0xaf, 0xb0, 0xb1, 0xb2, 0xb3, 0xb4, 0xb5, 0xb6, 0xb7, 0xb8, 0xb9, 0xba, 0xbb, 0xbc, 0xbd, 0xbe,
    0xbf, 0xc0, 0xc1, 0xc2, 0xc3, 0xc4, 0xc5, 0xc6, 0xc7, 0xc8, 0xc9, 0xca, 0xcb, 0xcc, 0xcd, 0xce,
    0xcf, 0xd0, 0xd1, 0xd2, 0xd3, 0xd4, 0xd5, 0xd6, 0xd7, 0xd8, 0xd9, 0xda, TH_ERR, TH_ERR, TH_ERR,
    TH_ERR, 0xdf, 0xe0, 0xe1, 0xe2, 0xe3, 0xe4, 0xe5, 0xe6, 0xe7, 0xe8, 0xe9, 0xea, 0xeb, 0xec,
    0xed, 0xee, 0xef, 0xf0, 0xf1, 0xf2, 0xf3, 0xf4, 0xf5, 0xf6, 0xf7, 0xf8, 0xf9, 0xfa, 0xfb,
    TH_ERR, TH_ERR, TH_ERR, TH_ERR,
];

const MACTHAI2UNI_TABLE: [i32; 128] = [
    0x00ab, 0x00bb, 0x2026, 0xf88c, 0xf88f, 0xf892, 0xf895, 0xf898, 0xf88b, 0xf88e, 0xf891, 0xf894,
    0xf897, 0x201c, 0x201d, 0xf899, WC_ERR, 0x2022, 0xf884, 0xf889, 0xf885, 0xf886, 0xf887, 0xf888,
    0xf88a, 0xf88d, 0xf890, 0xf893, 0xf896, 0x2018, 0x2019, WC_ERR, 0x00a0, 0x0e01, 0x0e02, 0x0e03,
    0x0e04, 0x0e05, 0x0e06, 0x0e07, 0x0e08, 0x0e09, 0x0e0a, 0x0e0b, 0x0e0c, 0x0e0d, 0x0e0e, 0x0e0f,
    0x0e10, 0x0e11, 0x0e12, 0x0e13, 0x0e14, 0x0e15, 0x0e16, 0x0e17, 0x0e18, 0x0e19, 0x0e1a, 0x0e1b,
    0x0e1c, 0x0e1d, 0x0e1e, 0x0e1f, 0x0e20, 0x0e21, 0x0e22, 0x0e23, 0x0e24, 0x0e25, 0x0e26, 0x0e27,
    0x0e28, 0x0e29, 0x0e2a, 0x0e2b, 0x0e2c, 0x0e2d, 0x0e2e, 0x0e2f, 0x0e30, 0x0e31, 0x0e32, 0x0e33,
    0x0e34, 0x0e35, 0x0e36, 0x0e37, 0x0e38, 0x0e39, 0x0e3a, 0xfeff, 0x200b, 0x2013, 0x2014, 0x0e3f,
    0x0e40, 0x0e41, 0x0e42, 0x0e43, 0x0e44, 0x0e45, 0x0e46, 0x0e47, 0x0e48, 0x0e49, 0x0e4a, 0x0e4b,
    0x0e4c, 0x0e4d, 0x2122, 0x0e4f, 0x0e50, 0x0e51, 0x0e52, 0x0e53, 0x0e54, 0x0e55, 0x0e56, 0x0e57,
    0x0e58, 0x0e59, 0x00ae, 0x00a9, WC_ERR, WC_ERR, WC_ERR, WC_ERR,
];

const WINTHAI2UNI_TABLE: [i32; 128] = [
    0xf700, 0xf701, 0xf702, 0xf703, 0xf704, 0x2026, 0xf705, 0xf706, 0xf707, 0xf708, 0xf709, 0xf70a,
    0xf70b, 0xf70c, 0xf70d, 0xf70e, 0xf70f, 0x2018, 0x2019, 0x201c, 0x201d, 0x2022, 0x2013, 0x2014,
    0xf710, 0xf711, 0xf712, 0xf713, 0xf714, 0xf715, 0xf716, 0xf717, 0x00a0, 0x0e01, 0x0e02, 0x0e03,
    0x0e04, 0x0e05, 0x0e06, 0x0e07, 0x0e08, 0x0e09, 0x0e0a, 0x0e0b, 0x0e0c, 0x0e0d, 0x0e0e, 0x0e0f,
    0x0e10, 0x0e11, 0x0e12, 0x0e13, 0x0e14, 0x0e15, 0x0e16, 0x0e17, 0x0e18, 0x0e19, 0x0e1a, 0x0e1b,
    0x0e1c, 0x0e1d, 0x0e1e, 0x0e1f, 0x0e20, 0x0e21, 0x0e22, 0x0e23, 0x0e24, 0x0e25, 0x0e26, 0x0e27,
    0x0e28, 0x0e29, 0x0e2a, 0x0e2b, 0x0e2c, 0x0e2d, 0x0e2e, 0x0e2f, 0x0e30, 0x0e31, 0x0e32, 0x0e33,
    0x0e34, 0x0e35, 0x0e36, 0x0e37, 0x0e38, 0x0e39, 0x0e3a, WC_ERR, WC_ERR, WC_ERR, WC_ERR, 0x0e3f,
    0x0e40, 0x0e41, 0x0e42, 0x0e43, 0x0e44, 0x0e45, 0x0e46, 0x0e47, 0x0e48, 0x0e49, 0x0e4a, 0x0e4b,
    0x0e4c, 0x0e4d, 0x0e4e, 0x0e4f, 0x0e50, 0x0e51, 0x0e52, 0x0e53, 0x0e54, 0x0e55, 0x0e56, 0x0e57,
    0x0e58, 0x0e59, 0x0e5a, 0x0e5b, 0xf718, 0xf719, 0xf71a, WC_ERR,
];

#[no_mangle]
pub unsafe extern "C" fn th_tis2uni(c: c_uchar) -> wchar_t {
    match c {
        0..=0x7f => c.into(),
        _ => TIS2UNI_TABLE[(c - 0x80) as usize],
    }
}

#[no_mangle]
pub unsafe extern "C" fn th_winthai2uni(c: c_uchar) -> wchar_t {
    match c {
        0..=0x7f => c.into(),
        _ => WINTHAI2UNI_TABLE[(c - 0x80) as usize],
    }
}

#[no_mangle]
pub unsafe extern "C" fn th_macthai2uni(c: c_uchar) -> wchar_t {
    match c {
        0..=0x7f => c.into(),
        _ => MACTHAI2UNI_TABLE[(c - 0x80) as usize],
    }
}

#[no_mangle]
pub unsafe extern "C" fn th_uni2tis(wc: wchar_t) -> c_uchar {
    match wc {
        0..=0x007f => wc as u8,
        0x0e00..=0x0e5f => UNI2TIS_TABLE[(wc - 0x0e00) as usize],
        _ => TH_ERR,
    }
}

#[no_mangle]
pub unsafe extern "C" fn th_uni2winthai(wc: wchar_t) -> c_uchar {
    let c = th_uni2tis(wc);
    match c {
        TH_ERR => {
            match WINTHAI2UNI_TABLE
                .iter()
                .copied()
                .enumerate()
                .find(|v| v.1 == wc)
            {
                Some(v) => (v.0 as u8) + 0x80,
                None => TH_ERR,
            }
        }
        _ => c,
    }
}

#[no_mangle]
pub unsafe extern "C" fn th_uni2macthai(wc: wchar_t) -> c_uchar {
    let c = th_uni2tis(wc);
    match c {
        TH_ERR => {
            match MACTHAI2UNI_TABLE
                .iter()
                .copied()
                .enumerate()
                .find(|v| v.1 == wc)
            {
                Some(v) => (v.0 as u8) + 0x80,
                None => TH_ERR,
            }
        }
        _ => c,
    }
}

#[no_mangle]
pub unsafe extern "C" fn th_tis2uni_line(
    s: *const c_uchar,
    result: *mut wchar_t,
    n: size_t,
) -> c_int {
    let input_len = utils::uchar_len(s);
    let output_len = input_len.min(n);
    let input_str = slice::from_raw_parts(s, output_len);
    let result_slice = slice::from_raw_parts_mut(result, n);

    for (idx, item) in input_str.iter().enumerate() {
        result_slice[idx] = th_tis2uni(*item);
    }
    result_slice[output_len.min(n - 1)] = 0;
    output_len as c_int
}

#[no_mangle]
pub unsafe extern "C" fn th_uni2tis_line(
    s: *const wchar_t,
    result: *mut c_uchar,
    n: size_t,
) -> c_int {
    let input_len = utils::wchar_len(s);
    let output_len = input_len.min(n);
    let input_str = slice::from_raw_parts(s, output_len);
    let result_slice = slice::from_raw_parts_mut(result, n);

    for (idx, item) in input_str.iter().enumerate() {
        result_slice[idx] = th_uni2tis(*item);
    }
    result_slice[output_len.min(n - 1)] = 0;
    output_len as c_int
}

#[cfg(test)]
mod tests {
    use crate::thwchar::*;

    const MAXLINELENGTH: usize = 100;
    const TIS_SAMPLE: &[u8] = include_bytes!("test_tis_sample.txt");
    const WIN_SAMPLE: &[u8] = include_bytes!("test_win_sample.txt");
    const MAC_SAMPLE: &[u8] = include_bytes!("test_mac_sample.txt");

    #[test]
    fn convert_line() {
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
