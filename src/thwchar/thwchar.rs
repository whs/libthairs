use ::libc;
use std::ffi::CStr;
use std::ptr::NonNull;
use std::{mem, slice};
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type thchar_t = libc::c_uchar;
pub type thwchar_t = wchar_t;
pub const THWCHAR_ERR: thwchar_t = !(0 as libc::c_int);
pub const THCHAR_ERR: libc::c_int = !(0 as libc::c_int);
pub const WC_ERR: thwchar_t = THWCHAR_ERR;
pub const TH_ERR: libc::c_int = !(0 as libc::c_int);
static mut tis620_0_uni_map_: [thwchar_t; 128] = [
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    0xe01 as libc::c_int,
    0xe02 as libc::c_int,
    0xe03 as libc::c_int,
    0xe04 as libc::c_int,
    0xe05 as libc::c_int,
    0xe06 as libc::c_int,
    0xe07 as libc::c_int,
    0xe08 as libc::c_int,
    0xe09 as libc::c_int,
    0xe0a as libc::c_int,
    0xe0b as libc::c_int,
    0xe0c as libc::c_int,
    0xe0d as libc::c_int,
    0xe0e as libc::c_int,
    0xe0f as libc::c_int,
    0xe10 as libc::c_int,
    0xe11 as libc::c_int,
    0xe12 as libc::c_int,
    0xe13 as libc::c_int,
    0xe14 as libc::c_int,
    0xe15 as libc::c_int,
    0xe16 as libc::c_int,
    0xe17 as libc::c_int,
    0xe18 as libc::c_int,
    0xe19 as libc::c_int,
    0xe1a as libc::c_int,
    0xe1b as libc::c_int,
    0xe1c as libc::c_int,
    0xe1d as libc::c_int,
    0xe1e as libc::c_int,
    0xe1f as libc::c_int,
    0xe20 as libc::c_int,
    0xe21 as libc::c_int,
    0xe22 as libc::c_int,
    0xe23 as libc::c_int,
    0xe24 as libc::c_int,
    0xe25 as libc::c_int,
    0xe26 as libc::c_int,
    0xe27 as libc::c_int,
    0xe28 as libc::c_int,
    0xe29 as libc::c_int,
    0xe2a as libc::c_int,
    0xe2b as libc::c_int,
    0xe2c as libc::c_int,
    0xe2d as libc::c_int,
    0xe2e as libc::c_int,
    0xe2f as libc::c_int,
    0xe30 as libc::c_int,
    0xe31 as libc::c_int,
    0xe32 as libc::c_int,
    0xe33 as libc::c_int,
    0xe34 as libc::c_int,
    0xe35 as libc::c_int,
    0xe36 as libc::c_int,
    0xe37 as libc::c_int,
    0xe38 as libc::c_int,
    0xe39 as libc::c_int,
    0xe3a as libc::c_int,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    0xe3f as libc::c_int,
    0xe40 as libc::c_int,
    0xe41 as libc::c_int,
    0xe42 as libc::c_int,
    0xe43 as libc::c_int,
    0xe44 as libc::c_int,
    0xe45 as libc::c_int,
    0xe46 as libc::c_int,
    0xe47 as libc::c_int,
    0xe48 as libc::c_int,
    0xe49 as libc::c_int,
    0xe4a as libc::c_int,
    0xe4b as libc::c_int,
    0xe4c as libc::c_int,
    0xe4d as libc::c_int,
    0xe4e as libc::c_int,
    0xe4f as libc::c_int,
    0xe50 as libc::c_int,
    0xe51 as libc::c_int,
    0xe52 as libc::c_int,
    0xe53 as libc::c_int,
    0xe54 as libc::c_int,
    0xe55 as libc::c_int,
    0xe56 as libc::c_int,
    0xe57 as libc::c_int,
    0xe58 as libc::c_int,
    0xe59 as libc::c_int,
    0xe5a as libc::c_int,
    0xe5b as libc::c_int,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
];
static mut uni_tis620_0_map_: [thchar_t; 96] = [
    TH_ERR as thchar_t,
    0xa1 as libc::c_int as thchar_t,
    0xa2 as libc::c_int as thchar_t,
    0xa3 as libc::c_int as thchar_t,
    0xa4 as libc::c_int as thchar_t,
    0xa5 as libc::c_int as thchar_t,
    0xa6 as libc::c_int as thchar_t,
    0xa7 as libc::c_int as thchar_t,
    0xa8 as libc::c_int as thchar_t,
    0xa9 as libc::c_int as thchar_t,
    0xaa as libc::c_int as thchar_t,
    0xab as libc::c_int as thchar_t,
    0xac as libc::c_int as thchar_t,
    0xad as libc::c_int as thchar_t,
    0xae as libc::c_int as thchar_t,
    0xaf as libc::c_int as thchar_t,
    0xb0 as libc::c_int as thchar_t,
    0xb1 as libc::c_int as thchar_t,
    0xb2 as libc::c_int as thchar_t,
    0xb3 as libc::c_int as thchar_t,
    0xb4 as libc::c_int as thchar_t,
    0xb5 as libc::c_int as thchar_t,
    0xb6 as libc::c_int as thchar_t,
    0xb7 as libc::c_int as thchar_t,
    0xb8 as libc::c_int as thchar_t,
    0xb9 as libc::c_int as thchar_t,
    0xba as libc::c_int as thchar_t,
    0xbb as libc::c_int as thchar_t,
    0xbc as libc::c_int as thchar_t,
    0xbd as libc::c_int as thchar_t,
    0xbe as libc::c_int as thchar_t,
    0xbf as libc::c_int as thchar_t,
    0xc0 as libc::c_int as thchar_t,
    0xc1 as libc::c_int as thchar_t,
    0xc2 as libc::c_int as thchar_t,
    0xc3 as libc::c_int as thchar_t,
    0xc4 as libc::c_int as thchar_t,
    0xc5 as libc::c_int as thchar_t,
    0xc6 as libc::c_int as thchar_t,
    0xc7 as libc::c_int as thchar_t,
    0xc8 as libc::c_int as thchar_t,
    0xc9 as libc::c_int as thchar_t,
    0xca as libc::c_int as thchar_t,
    0xcb as libc::c_int as thchar_t,
    0xcc as libc::c_int as thchar_t,
    0xcd as libc::c_int as thchar_t,
    0xce as libc::c_int as thchar_t,
    0xcf as libc::c_int as thchar_t,
    0xd0 as libc::c_int as thchar_t,
    0xd1 as libc::c_int as thchar_t,
    0xd2 as libc::c_int as thchar_t,
    0xd3 as libc::c_int as thchar_t,
    0xd4 as libc::c_int as thchar_t,
    0xd5 as libc::c_int as thchar_t,
    0xd6 as libc::c_int as thchar_t,
    0xd7 as libc::c_int as thchar_t,
    0xd8 as libc::c_int as thchar_t,
    0xd9 as libc::c_int as thchar_t,
    0xda as libc::c_int as thchar_t,
    TH_ERR as thchar_t,
    TH_ERR as thchar_t,
    TH_ERR as thchar_t,
    TH_ERR as thchar_t,
    0xdf as libc::c_int as thchar_t,
    0xe0 as libc::c_int as thchar_t,
    0xe1 as libc::c_int as thchar_t,
    0xe2 as libc::c_int as thchar_t,
    0xe3 as libc::c_int as thchar_t,
    0xe4 as libc::c_int as thchar_t,
    0xe5 as libc::c_int as thchar_t,
    0xe6 as libc::c_int as thchar_t,
    0xe7 as libc::c_int as thchar_t,
    0xe8 as libc::c_int as thchar_t,
    0xe9 as libc::c_int as thchar_t,
    0xea as libc::c_int as thchar_t,
    0xeb as libc::c_int as thchar_t,
    0xec as libc::c_int as thchar_t,
    0xed as libc::c_int as thchar_t,
    0xee as libc::c_int as thchar_t,
    0xef as libc::c_int as thchar_t,
    0xf0 as libc::c_int as thchar_t,
    0xf1 as libc::c_int as thchar_t,
    0xf2 as libc::c_int as thchar_t,
    0xf3 as libc::c_int as thchar_t,
    0xf4 as libc::c_int as thchar_t,
    0xf5 as libc::c_int as thchar_t,
    0xf6 as libc::c_int as thchar_t,
    0xf7 as libc::c_int as thchar_t,
    0xf8 as libc::c_int as thchar_t,
    0xf9 as libc::c_int as thchar_t,
    0xfa as libc::c_int as thchar_t,
    0xfb as libc::c_int as thchar_t,
    TH_ERR as thchar_t,
    TH_ERR as thchar_t,
    TH_ERR as thchar_t,
    TH_ERR as thchar_t,
];
static mut tis620_1_uni_map_: [thwchar_t; 128] = [
    0xab as libc::c_int,
    0xbb as libc::c_int,
    0x2026 as libc::c_int,
    0xf88c as libc::c_int,
    0xf88f as libc::c_int,
    0xf892 as libc::c_int,
    0xf895 as libc::c_int,
    0xf898 as libc::c_int,
    0xf88b as libc::c_int,
    0xf88e as libc::c_int,
    0xf891 as libc::c_int,
    0xf894 as libc::c_int,
    0xf897 as libc::c_int,
    0x201c as libc::c_int,
    0x201d as libc::c_int,
    0xf899 as libc::c_int,
    WC_ERR,
    0x2022 as libc::c_int,
    0xf884 as libc::c_int,
    0xf889 as libc::c_int,
    0xf885 as libc::c_int,
    0xf886 as libc::c_int,
    0xf887 as libc::c_int,
    0xf888 as libc::c_int,
    0xf88a as libc::c_int,
    0xf88d as libc::c_int,
    0xf890 as libc::c_int,
    0xf893 as libc::c_int,
    0xf896 as libc::c_int,
    0x2018 as libc::c_int,
    0x2019 as libc::c_int,
    WC_ERR,
    0xa0 as libc::c_int,
    0xe01 as libc::c_int,
    0xe02 as libc::c_int,
    0xe03 as libc::c_int,
    0xe04 as libc::c_int,
    0xe05 as libc::c_int,
    0xe06 as libc::c_int,
    0xe07 as libc::c_int,
    0xe08 as libc::c_int,
    0xe09 as libc::c_int,
    0xe0a as libc::c_int,
    0xe0b as libc::c_int,
    0xe0c as libc::c_int,
    0xe0d as libc::c_int,
    0xe0e as libc::c_int,
    0xe0f as libc::c_int,
    0xe10 as libc::c_int,
    0xe11 as libc::c_int,
    0xe12 as libc::c_int,
    0xe13 as libc::c_int,
    0xe14 as libc::c_int,
    0xe15 as libc::c_int,
    0xe16 as libc::c_int,
    0xe17 as libc::c_int,
    0xe18 as libc::c_int,
    0xe19 as libc::c_int,
    0xe1a as libc::c_int,
    0xe1b as libc::c_int,
    0xe1c as libc::c_int,
    0xe1d as libc::c_int,
    0xe1e as libc::c_int,
    0xe1f as libc::c_int,
    0xe20 as libc::c_int,
    0xe21 as libc::c_int,
    0xe22 as libc::c_int,
    0xe23 as libc::c_int,
    0xe24 as libc::c_int,
    0xe25 as libc::c_int,
    0xe26 as libc::c_int,
    0xe27 as libc::c_int,
    0xe28 as libc::c_int,
    0xe29 as libc::c_int,
    0xe2a as libc::c_int,
    0xe2b as libc::c_int,
    0xe2c as libc::c_int,
    0xe2d as libc::c_int,
    0xe2e as libc::c_int,
    0xe2f as libc::c_int,
    0xe30 as libc::c_int,
    0xe31 as libc::c_int,
    0xe32 as libc::c_int,
    0xe33 as libc::c_int,
    0xe34 as libc::c_int,
    0xe35 as libc::c_int,
    0xe36 as libc::c_int,
    0xe37 as libc::c_int,
    0xe38 as libc::c_int,
    0xe39 as libc::c_int,
    0xe3a as libc::c_int,
    0xfeff as libc::c_int,
    0x200b as libc::c_int,
    0x2013 as libc::c_int,
    0x2014 as libc::c_int,
    0xe3f as libc::c_int,
    0xe40 as libc::c_int,
    0xe41 as libc::c_int,
    0xe42 as libc::c_int,
    0xe43 as libc::c_int,
    0xe44 as libc::c_int,
    0xe45 as libc::c_int,
    0xe46 as libc::c_int,
    0xe47 as libc::c_int,
    0xe48 as libc::c_int,
    0xe49 as libc::c_int,
    0xe4a as libc::c_int,
    0xe4b as libc::c_int,
    0xe4c as libc::c_int,
    0xe4d as libc::c_int,
    0x2122 as libc::c_int,
    0xe4f as libc::c_int,
    0xe50 as libc::c_int,
    0xe51 as libc::c_int,
    0xe52 as libc::c_int,
    0xe53 as libc::c_int,
    0xe54 as libc::c_int,
    0xe55 as libc::c_int,
    0xe56 as libc::c_int,
    0xe57 as libc::c_int,
    0xe58 as libc::c_int,
    0xe59 as libc::c_int,
    0xae as libc::c_int,
    0xa9 as libc::c_int,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
];
static mut tis620_2_uni_map_: [thwchar_t; 128] = [
    0xf700 as libc::c_int,
    0xf701 as libc::c_int,
    0xf702 as libc::c_int,
    0xf703 as libc::c_int,
    0xf704 as libc::c_int,
    0x2026 as libc::c_int,
    0xf705 as libc::c_int,
    0xf706 as libc::c_int,
    0xf707 as libc::c_int,
    0xf708 as libc::c_int,
    0xf709 as libc::c_int,
    0xf70a as libc::c_int,
    0xf70b as libc::c_int,
    0xf70c as libc::c_int,
    0xf70d as libc::c_int,
    0xf70e as libc::c_int,
    0xf70f as libc::c_int,
    0x2018 as libc::c_int,
    0x2019 as libc::c_int,
    0x201c as libc::c_int,
    0x201d as libc::c_int,
    0x2022 as libc::c_int,
    0x2013 as libc::c_int,
    0x2014 as libc::c_int,
    0xf710 as libc::c_int,
    0xf711 as libc::c_int,
    0xf712 as libc::c_int,
    0xf713 as libc::c_int,
    0xf714 as libc::c_int,
    0xf715 as libc::c_int,
    0xf716 as libc::c_int,
    0xf717 as libc::c_int,
    0xa0 as libc::c_int,
    0xe01 as libc::c_int,
    0xe02 as libc::c_int,
    0xe03 as libc::c_int,
    0xe04 as libc::c_int,
    0xe05 as libc::c_int,
    0xe06 as libc::c_int,
    0xe07 as libc::c_int,
    0xe08 as libc::c_int,
    0xe09 as libc::c_int,
    0xe0a as libc::c_int,
    0xe0b as libc::c_int,
    0xe0c as libc::c_int,
    0xe0d as libc::c_int,
    0xe0e as libc::c_int,
    0xe0f as libc::c_int,
    0xe10 as libc::c_int,
    0xe11 as libc::c_int,
    0xe12 as libc::c_int,
    0xe13 as libc::c_int,
    0xe14 as libc::c_int,
    0xe15 as libc::c_int,
    0xe16 as libc::c_int,
    0xe17 as libc::c_int,
    0xe18 as libc::c_int,
    0xe19 as libc::c_int,
    0xe1a as libc::c_int,
    0xe1b as libc::c_int,
    0xe1c as libc::c_int,
    0xe1d as libc::c_int,
    0xe1e as libc::c_int,
    0xe1f as libc::c_int,
    0xe20 as libc::c_int,
    0xe21 as libc::c_int,
    0xe22 as libc::c_int,
    0xe23 as libc::c_int,
    0xe24 as libc::c_int,
    0xe25 as libc::c_int,
    0xe26 as libc::c_int,
    0xe27 as libc::c_int,
    0xe28 as libc::c_int,
    0xe29 as libc::c_int,
    0xe2a as libc::c_int,
    0xe2b as libc::c_int,
    0xe2c as libc::c_int,
    0xe2d as libc::c_int,
    0xe2e as libc::c_int,
    0xe2f as libc::c_int,
    0xe30 as libc::c_int,
    0xe31 as libc::c_int,
    0xe32 as libc::c_int,
    0xe33 as libc::c_int,
    0xe34 as libc::c_int,
    0xe35 as libc::c_int,
    0xe36 as libc::c_int,
    0xe37 as libc::c_int,
    0xe38 as libc::c_int,
    0xe39 as libc::c_int,
    0xe3a as libc::c_int,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    WC_ERR,
    0xe3f as libc::c_int,
    0xe40 as libc::c_int,
    0xe41 as libc::c_int,
    0xe42 as libc::c_int,
    0xe43 as libc::c_int,
    0xe44 as libc::c_int,
    0xe45 as libc::c_int,
    0xe46 as libc::c_int,
    0xe47 as libc::c_int,
    0xe48 as libc::c_int,
    0xe49 as libc::c_int,
    0xe4a as libc::c_int,
    0xe4b as libc::c_int,
    0xe4c as libc::c_int,
    0xe4d as libc::c_int,
    0xe4e as libc::c_int,
    0xe4f as libc::c_int,
    0xe50 as libc::c_int,
    0xe51 as libc::c_int,
    0xe52 as libc::c_int,
    0xe53 as libc::c_int,
    0xe54 as libc::c_int,
    0xe55 as libc::c_int,
    0xe56 as libc::c_int,
    0xe57 as libc::c_int,
    0xe58 as libc::c_int,
    0xe59 as libc::c_int,
    0xe5a as libc::c_int,
    0xe5b as libc::c_int,
    0xf718 as libc::c_int,
    0xf719 as libc::c_int,
    0xf71a as libc::c_int,
    WC_ERR,
];
#[no_mangle]
pub unsafe extern "C" fn th_tis2uni(mut c: thchar_t) -> thwchar_t {
    return if (c as libc::c_int) < 0x80 as libc::c_int {
        c as thwchar_t
    } else {
        tis620_0_uni_map_[(c as libc::c_int - 0x80 as libc::c_int) as usize]
    };
}

/// Convert string from TIS-620 to Unicode
#[no_mangle]
pub extern "C" fn th_tis2uni_line(
    s: *const thchar_t,
    mut result: NonNull<thwchar_t>,
    n: usize,
) -> i32 {
    let s = unsafe { CStr::from_ptr(s as *const libc::c_char) };
    let mut result = unsafe { slice::from_raw_parts_mut(result.as_ptr(), n) };

    let mut out_len = 0;
    for (i, (src, dst)) in s
        .to_bytes()
        .iter()
        .zip(result[..n - 1].iter_mut())
        .enumerate()
    {
        *dst = unsafe { th_tis2uni(*src) };
        out_len = i;
    }
    result[out_len + 1] = 0;

    out_len as i32
}

#[no_mangle]
pub unsafe extern "C" fn th_winthai2uni(mut c: thchar_t) -> thwchar_t {
    return if (c as libc::c_int) < 0x80 as libc::c_int {
        c as thwchar_t
    } else {
        tis620_2_uni_map_[(c as libc::c_int - 0x80 as libc::c_int) as usize]
    };
}
#[no_mangle]
pub unsafe extern "C" fn th_macthai2uni(mut c: thchar_t) -> thwchar_t {
    return if (c as libc::c_int) < 0x80 as libc::c_int {
        c as thwchar_t
    } else {
        tis620_1_uni_map_[(c as libc::c_int - 0x80 as libc::c_int) as usize]
    };
}
#[no_mangle]
pub unsafe extern "C" fn th_uni2tis(mut wc: thwchar_t) -> thchar_t {
    if wc < 0x80 as libc::c_int {
        return wc as thchar_t;
    } else if 0xe00 as libc::c_int <= wc && wc <= 0xe5f as libc::c_int {
        return uni_tis620_0_map_[(wc - 0xe00 as libc::c_int) as usize];
    } else {
        return TH_ERR as thchar_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn th_uni2tis_line(
    mut s: *const thwchar_t,
    mut result: *mut thchar_t,
    mut n: size_t,
) -> libc::c_int {
    let mut left: libc::c_int = n as libc::c_int;
    while *s != 0 && left > 1 as libc::c_int {
        let fresh2 = s;
        s = s.offset(1);
        let fresh3 = result;
        result = result.offset(1);
        *fresh3 = th_uni2tis(*fresh2);
        left -= 1;
        left;
    }
    *result = 0 as libc::c_int as thchar_t;
    return n.wrapping_sub(left as size_t) as libc::c_int;
}
unsafe extern "C" fn uni2thai_ext_(mut wc: thwchar_t, mut rev_map: *const thwchar_t) -> thchar_t {
    let mut c: thchar_t = 0x80 as libc::c_int as thchar_t;
    loop {
        if *rev_map.offset((c as libc::c_int - 0x80 as libc::c_int) as isize) == wc {
            return c;
        }
        let fresh4 = c;
        c = c.wrapping_add(1);
        if !(fresh4 as libc::c_int != 0xff as libc::c_int) {
            break;
        }
    }
    return TH_ERR as thchar_t;
}
#[no_mangle]
pub unsafe extern "C" fn th_uni2winthai(mut wc: thwchar_t) -> thchar_t {
    let mut c: thchar_t = th_uni2tis(wc);
    return (if c as libc::c_int == TH_ERR {
        uni2thai_ext_(wc, tis620_2_uni_map_.as_mut_ptr() as *const thwchar_t) as libc::c_int
    } else {
        c as libc::c_int
    }) as thchar_t;
}
#[no_mangle]
pub unsafe extern "C" fn th_uni2macthai(mut wc: thwchar_t) -> thchar_t {
    let mut c: thchar_t = th_uni2tis(wc);
    return (if c as libc::c_int == TH_ERR {
        uni2thai_ext_(wc, tis620_1_uni_map_.as_mut_ptr() as *const thwchar_t) as libc::c_int
    } else {
        c as libc::c_int
    }) as thchar_t;
}
