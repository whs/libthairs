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

//! Wide char support for Thai

use crate::{thchar_t, THCHAR_ERR};

/// Thai character type for storing Unicode character
pub type thwchar_t = libc::wchar_t;

/// Wide-character value indicating error
pub const THWCHAR_ERR: thwchar_t = thwchar_t::MAX;
const WC_ERR: thwchar_t = THWCHAR_ERR;
const TH_ERR: thchar_t = THCHAR_ERR;

#[rustfmt::skip]
const tis620_0_uni_map_: [thwchar_t; 128] = [
    WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR,
    WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR,
    WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR,
    WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR, WC_ERR,
    WC_ERR, 0x0e01, 0x0e02, 0x0e03, 0x0e04, 0x0e05, 0x0e06, 0x0e07,
    0x0e08, 0x0e09, 0x0e0a, 0x0e0b, 0x0e0c, 0x0e0d, 0x0e0e, 0x0e0f,
    0x0e10, 0x0e11, 0x0e12, 0x0e13, 0x0e14, 0x0e15, 0x0e16, 0x0e17,
    0x0e18, 0x0e19, 0x0e1a, 0x0e1b, 0x0e1c, 0x0e1d, 0x0e1e, 0x0e1f,
    0x0e20, 0x0e21, 0x0e22, 0x0e23, 0x0e24, 0x0e25, 0x0e26, 0x0e27,
    0x0e28, 0x0e29, 0x0e2a, 0x0e2b, 0x0e2c, 0x0e2d, 0x0e2e, 0x0e2f,
    0x0e30, 0x0e31, 0x0e32, 0x0e33, 0x0e34, 0x0e35, 0x0e36, 0x0e37,
    0x0e38, 0x0e39, 0x0e3a, WC_ERR, WC_ERR, WC_ERR, WC_ERR, 0x0e3f,
    0x0e40, 0x0e41, 0x0e42, 0x0e43, 0x0e44, 0x0e45, 0x0e46, 0x0e47,
    0x0e48, 0x0e49, 0x0e4a, 0x0e4b, 0x0e4c, 0x0e4d, 0x0e4e, 0x0e4f,
    0x0e50, 0x0e51, 0x0e52, 0x0e53, 0x0e54, 0x0e55, 0x0e56, 0x0e57,
    0x0e58, 0x0e59, 0x0e5a, 0x0e5b, WC_ERR, WC_ERR, WC_ERR, WC_ERR
];

#[rustfmt::skip]
const uni_tis620_0_map_: [thchar_t; 96] = [
  TH_ERR,    0xa1,   0xa2,   0xa3,   0xa4,   0xa5,   0xa6,   0xa7,
    0xa8,    0xa9,   0xaa,   0xab,   0xac,   0xad,   0xae,   0xaf,
    0xb0,    0xb1,   0xb2,   0xb3,   0xb4,   0xb5,   0xb6,   0xb7,
    0xb8,    0xb9,   0xba,   0xbb,   0xbc,   0xbd,   0xbe,   0xbf,
    0xc0,    0xc1,   0xc2,   0xc3,   0xc4,   0xc5,   0xc6,   0xc7,
    0xc8,    0xc9,   0xca,   0xcb,   0xcc,   0xcd,   0xce,   0xcf,
    0xd0,    0xd1,   0xd2,   0xd3,   0xd4,   0xd5,   0xd6,   0xd7,
    0xd8,    0xd9,   0xda, TH_ERR, TH_ERR, TH_ERR, TH_ERR,   0xdf,
    0xe0,    0xe1,   0xe2,   0xe3,   0xe4,   0xe5,   0xe6,   0xe7,
    0xe8,    0xe9,   0xea,   0xeb,   0xec,   0xed,   0xee,   0xef,
    0xf0,    0xf1,   0xf2,   0xf3,   0xf4,   0xf5,   0xf6,   0xf7,
    0xf8,    0xf9,   0xfa,   0xfb, TH_ERR, TH_ERR, TH_ERR, TH_ERR
];

#[rustfmt::skip]
const tis620_1_uni_map_: [thwchar_t; 128] = [
    0x00ab, 0x00bb, 0x2026, 0xf88c, 0xf88f, 0xf892, 0xf895, 0xf898,
    0xf88b, 0xf88e, 0xf891, 0xf894, 0xf897, 0x201c, 0x201d, 0xf899,
    WC_ERR, 0x2022, 0xf884, 0xf889, 0xf885, 0xf886, 0xf887, 0xf888,
    0xf88a, 0xf88d, 0xf890, 0xf893, 0xf896, 0x2018, 0x2019, WC_ERR,
    0x00a0, 0x0e01, 0x0e02, 0x0e03, 0x0e04, 0x0e05, 0x0e06, 0x0e07,
    0x0e08, 0x0e09, 0x0e0a, 0x0e0b, 0x0e0c, 0x0e0d, 0x0e0e, 0x0e0f,
    0x0e10, 0x0e11, 0x0e12, 0x0e13, 0x0e14, 0x0e15, 0x0e16, 0x0e17,
    0x0e18, 0x0e19, 0x0e1a, 0x0e1b, 0x0e1c, 0x0e1d, 0x0e1e, 0x0e1f,
    0x0e20, 0x0e21, 0x0e22, 0x0e23, 0x0e24, 0x0e25, 0x0e26, 0x0e27,
    0x0e28, 0x0e29, 0x0e2a, 0x0e2b, 0x0e2c, 0x0e2d, 0x0e2e, 0x0e2f,
    0x0e30, 0x0e31, 0x0e32, 0x0e33, 0x0e34, 0x0e35, 0x0e36, 0x0e37,
    0x0e38, 0x0e39, 0x0e3a, 0xfeff, 0x200b, 0x2013, 0x2014, 0x0e3f,
    0x0e40, 0x0e41, 0x0e42, 0x0e43, 0x0e44, 0x0e45, 0x0e46, 0x0e47,
    0x0e48, 0x0e49, 0x0e4a, 0x0e4b, 0x0e4c, 0x0e4d, 0x2122, 0x0e4f,
    0x0e50, 0x0e51, 0x0e52, 0x0e53, 0x0e54, 0x0e55, 0x0e56, 0x0e57,
    0x0e58, 0x0e59, 0x00ae, 0x00a9, WC_ERR, WC_ERR, WC_ERR, WC_ERR
];

#[rustfmt::skip]
const tis620_2_uni_map_: [thwchar_t; 128] = [
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

/// Convert character code from TIS-620 to Unicode.
pub const fn tis2uni(c: thchar_t) -> thwchar_t {
    if c < 0x80 {
        c as thwchar_t
    } else {
        tis620_0_uni_map_[(c - 0x80) as usize]
    }
}

/// Convert string from TIS-620 to Unicode
pub fn tis2uni_str(s: &[thchar_t]) -> Vec<thwchar_t> {
    s.iter().map(|c| tis2uni(*c)).collect()
}

/// Convert [`thwchar_t`] string to Rust String
pub fn uni2rust(s: &[thwchar_t]) -> String {
    s.iter()
        .map(|c| match *c {
            WC_ERR => char::REPLACEMENT_CHARACTER,
            c => char::from_u32(c as u32).unwrap_or(char::REPLACEMENT_CHARACTER),
        })
        .collect()
}

/// Convert character code from Thai Windows extended code to Unicode.
pub const fn winthai2uni(c: thchar_t) -> thwchar_t {
    if c < 0x80 {
        c as thwchar_t
    } else {
        tis620_2_uni_map_[(c - 0x80) as usize]
    }
}

/// Convert character code from Mac Thai extended code to Unicode
pub const fn macthai2uni(c: thchar_t) -> thwchar_t {
    if c < 0x80 {
        c as thwchar_t
    } else {
        tis620_1_uni_map_[(c - 0x80) as usize]
    }
}

/// Convert character code from Unicode to TIS-620
pub const fn uni2tis(wc: thwchar_t) -> thchar_t {
    match wc {
        wc if wc < 0x0080 => wc as thchar_t,
        wc if 0x0e00 <= wc && wc <= 0x0e5f => uni_tis620_0_map_[(wc - 0xe00) as usize],
        _ => TH_ERR,
    }
}

/// Convert string from Unicode to TIS-620
///
/// May contain internal [`THCHAR_ERR`] when characters are out of range
pub fn uni2tis_str(s: &[thwchar_t]) -> Vec<thchar_t> {
    s.iter().map(|c| uni2tis(*c)).collect()
}

fn uni2thai_ext(wc: thwchar_t, rev_map: &[thwchar_t]) -> thchar_t {
    // wc assumed out of TIS range
    for c in 0x80..0xff {
        if rev_map[(c - 0x80) as usize] == wc {
            return c;
        }
    }

    TH_ERR
}

/// Convert character code from Unicode to Thai Windows extended code
pub fn uni2winthai(wc: thwchar_t) -> thchar_t {
    let c = uni2tis(wc);
    match c {
        TH_ERR => uni2thai_ext(wc, &tis620_2_uni_map_),
        _ => c,
    }
}

/// Convert character code from Unicode to Mac Thai extended code
pub fn uni2macthai(wc: thwchar_t) -> thchar_t {
    let c = uni2tis(wc);
    match c {
        TH_ERR => uni2thai_ext(wc, &tis620_1_uni_map_),
        _ => c,
    }
}

#[cfg(feature = "cffi")]
mod cffi {
    use super::*;
    use crate::thchar_t;
    use null_terminated::Nul;
    use std::ptr::NonNull;
    use std::slice;

    /// Convert character code from TIS-620 to Unicode.
    #[no_mangle]
    pub const extern "C" fn th_tis2uni(c: thchar_t) -> thwchar_t {
        tis2uni(c)
    }

    /// Convert string from TIS-620 to Unicode
    #[no_mangle]
    pub extern "C" fn th_tis2uni_line(
        source: *const thchar_t,
        mut result: NonNull<thwchar_t>,
        n: usize,
    ) -> libc::c_int {
        if n == 0 {
            return 0;
        }

        let source = unsafe { Nul::new_unchecked(source) };
        let mut result = unsafe { slice::from_raw_parts_mut(result.as_ptr(), n) };

        let mut out_len = 0;
        for (src, dst) in source.iter().zip(result[..n - 1].iter_mut()) {
            *dst = tis2uni(*src);
            out_len += 1;
        }
        result[out_len] = 0;

        out_len.try_into().unwrap_or(libc::c_int::MAX)
    }

    /// Convert character code from Thai Windows extended code to Unicode.
    #[no_mangle]
    pub const extern "C" fn th_winthai2uni(c: thchar_t) -> thwchar_t {
        winthai2uni(c)
    }

    /// Convert character code from Mac Thai extended code to Unicode
    #[no_mangle]
    pub const extern "C" fn th_macthai2uni(c: thchar_t) -> thwchar_t {
        macthai2uni(c)
    }

    /// Convert character code from Unicode to TIS-620
    #[no_mangle]
    pub const extern "C" fn th_uni2tis(wc: thwchar_t) -> thchar_t {
        uni2tis(wc)
    }

    /// Convert string from Unicode to TIS-620.
    #[no_mangle]
    pub extern "C" fn th_uni2tis_line(
        source: *const thwchar_t,
        mut result: NonNull<thchar_t>,
        n: usize,
    ) -> libc::c_int {
        if n == 0 {
            return 0;
        }

        let source = unsafe { Nul::new_unchecked(source) };
        let mut result = unsafe { slice::from_raw_parts_mut(result.as_ptr(), n) };

        let mut out_len = 0;
        for (src, dst) in source.iter().zip(result[..n - 1].iter_mut()) {
            *dst = uni2tis(*src);
            out_len += 1;
        }
        result[out_len] = 0;

        out_len.try_into().unwrap_or(libc::c_int::MAX)
    }

    /// Convert character code from Unicode to Thai Windows extended code
    #[no_mangle]
    pub extern "C" fn th_uni2winthai(wc: thwchar_t) -> thchar_t {
        uni2winthai(wc)
    }

    /// Convert character code from Unicode to Mac Thai extended code
    #[no_mangle]
    pub extern "C" fn th_uni2macthai(wc: thwchar_t) -> thchar_t {
        uni2macthai(wc)
    }
}

mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr::NonNull;

    const tis_input: [u8; 32] = [
        0xca, 0xc7, 0xd1, 0xca, 0xb4, 0xd5, 0xa4, 0xc3, 0xd1, 0xba, 0x20, 0xb9, 0xd5, 0xe8, 0xe0,
        0xbb, 0xe7, 0xb9, 0xa1, 0xd2, 0xc3, 0xb7, 0xb4, 0xca, 0xcd, 0xba, 0xb5, 0xd1, 0xc7, 0xe0,
        0xcd, 0xa7,
    ];

    const win_sample: [u8; 45] = [
        0xbe, 0x8b, 0xcd, 0xbb, 0xd9, 0x86, 0xbe, 0xd5, 0xe8, 0xbb, 0x82, 0x9b, 0xae, 0xfc, 0x80,
        0xd8, 0x90, 0xd8, 0xa1, 0xed, 0xd2, 0xbb, 0x99, 0xd2, 0xa1, 0xed, 0xe9, 0xd2, 0xbb, 0x99,
        0x9c, 0xd2, 0xbb, 0x99, 0xd2, 0x20, 0x8c, 0xb7, 0x8b, 0x20, 0xd5, 0xa1, 0xe7, 0xbb, 0x9a,
    ];

    const mac_sample: [u8; 45] = [
        0xbe, 0x88, 0xcd, 0xbb, 0xd9, 0x83, 0xbe, 0xd5, 0xe8, 0xbb, 0x95, 0x98, 0xae, 0xd8, 0xb0,
        0xd8, 0xad, 0xd8, 0xa1, 0xed, 0xd2, 0xbb, 0x8f, 0xd2, 0xa1, 0xed, 0xe9, 0xd2, 0xbb, 0x8f,
        0x99, 0xd2, 0xbb, 0x8f, 0xd2, 0x20, 0x89, 0xb7, 0x88, 0x20, 0xd5, 0xa1, 0xe7, 0xbb, 0x93,
    ];

    #[cfg(feature = "cffi")]
    #[test]
    fn test_c_tis2uni_line() {
        let input = CString::new(tis_input).unwrap();
        let mut buf = vec![0; 1000];
        let out_len = cffi::th_tis2uni_line(
            input.as_ptr().cast(),
            NonNull::new(buf.as_mut_ptr()).unwrap(),
            buf.len(),
        );
        assert_eq!(input.count_bytes(), out_len as usize);
        assert_eq!(buf[out_len as usize], 0);
    }

    #[cfg(feature = "cffi")]
    #[test]
    fn test_c_tis2uni_line_underflow() {
        let input = CString::new(tis_input).unwrap();
        let mut buf = vec![0; 10];
        let out_len = cffi::th_tis2uni_line(
            input.as_ptr().cast(),
            NonNull::new(buf.as_mut_ptr()).unwrap(),
            buf.len(),
        );
        assert_eq!(buf.len() - 1, out_len as usize);
        assert_eq!(buf[out_len as usize], 0);
    }

    #[test]
    fn test_tis2uni_line() {
        assert_eq!(
            uni2rust(&tis2uni_str(&tis_input)),
            "สวัสดีครับ นี่เป็นการทดสอบตัวเอง"
        );
    }

    #[cfg(feature = "cffi")]
    #[test]
    fn test_c_convert_reversible() {
        let input = CString::new(tis_input).unwrap();

        let mut wchar_buf = vec![0; 1000];
        cffi::th_tis2uni_line(
            input.as_ptr().cast(),
            NonNull::new(wchar_buf.as_mut_ptr()).unwrap(),
            wchar_buf.len(),
        );

        let mut tis_buf = vec![0; 1000];
        let tis_len = cffi::th_uni2tis_line(
            wchar_buf.as_ptr().cast(),
            NonNull::new(tis_buf.as_mut_ptr()).unwrap(),
            tis_buf.len(),
        );

        assert_eq!(tis_input.len(), tis_len as usize);
        assert_eq!(tis_input, tis_buf[..tis_len as usize]);
        assert_eq!(tis_buf[tis_len as usize], 0);
    }

    #[test]
    fn test_convert_reversible() {
        let uni = tis2uni_str(&tis_input);
        let tis = uni2tis_str(&uni);

        assert_eq!(tis_input, tis.as_slice());
    }

    #[cfg(feature = "cffi")]
    #[test]
    fn test_c_convert_reversible_win() {
        for ch in win_sample {
            assert_eq!(ch, cffi::th_uni2winthai(cffi::th_winthai2uni(ch)))
        }
    }

    #[test]
    fn test_convert_reversible_win() {
        for ch in win_sample {
            assert_eq!(ch, uni2winthai(winthai2uni(ch)))
        }
    }

    #[cfg(feature = "cffi")]
    #[test]
    fn test_c_convert_reversible_mac() {
        for ch in mac_sample {
            assert_eq!(ch, cffi::th_uni2macthai(cffi::th_macthai2uni(ch)))
        }
    }

    #[test]
    fn test_convert_reversible_mac() {
        for ch in mac_sample {
            assert_eq!(ch, uni2macthai(macthai2uni(ch)))
        }
    }
}
