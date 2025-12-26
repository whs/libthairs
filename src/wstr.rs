//! Thai wide-char string manipulators

use ::libc;
extern "C" {
    fn th_uni2tis(wc: thwchar_t) -> thchar_t;
    fn th_tis2uni(c: thchar_t) -> thwchar_t;
    fn th_normalize(dest: *mut thchar_t, src: *const thchar_t, n: size_t) -> size_t;
    fn free(_: *mut libc::c_void);
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type thchar_t = libc::c_uchar;
pub type thwchar_t = wchar_t;
pub const THCHAR_ERR: libc::c_int = !(0 as libc::c_int);
unsafe extern "C" fn th_wthaichunk(
    mut dest: *mut thchar_t,
    mut wsrc: *const thwchar_t,
    mut n: size_t,
) -> libc::c_int {
    if *wsrc == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    if th_uni2tis(*wsrc) as libc::c_int == THCHAR_ERR {
        let mut len: libc::c_int = 0 as libc::c_int;
        while *wsrc != 0 && th_uni2tis(*wsrc) as libc::c_int == THCHAR_ERR {
            wsrc = wsrc.offset(1);
            wsrc;
            len += 1;
            len;
        }
        return -len;
    } else {
        let mut left: size_t = n;
        let mut c: thchar_t = 0;
        while left > 1 as libc::c_int as size_t && *wsrc != 0 && {
            c = th_uni2tis(*wsrc);
            c as libc::c_int != THCHAR_ERR
        } {
            let fresh0 = dest;
            dest = dest.offset(1);
            *fresh0 = c;
            wsrc = wsrc.offset(1);
            wsrc;
            left = left.wrapping_sub(1);
            left;
        }
        *dest = 0 as libc::c_int as thchar_t;
        return n.wrapping_sub(left) as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn th_wnormalize(
    mut wdest: *mut thwchar_t,
    mut wsrc: *const thwchar_t,
    mut n: size_t,
) -> size_t {
    let mut left: size_t = n;
    let mut src8: *mut thchar_t =
        malloc(n.wrapping_mul(::core::mem::size_of::<thchar_t>() as libc::c_ulong))
            as *mut thchar_t;
    let mut norm8: *mut thchar_t =
        malloc(n.wrapping_mul(::core::mem::size_of::<thchar_t>() as libc::c_ulong))
            as *mut thchar_t;
    while left > 1 as libc::c_int as size_t && *wsrc != 0 {
        let mut chunk_len: libc::c_int =
            th_wthaichunk(src8, wsrc, n.wrapping_sub(1 as libc::c_int as size_t));
        *src8.offset(n.wrapping_sub(1 as libc::c_int as size_t) as isize) =
            0 as libc::c_int as thchar_t;
        if chunk_len > 0 as libc::c_int {
            let mut i: libc::c_int = 0;
            let mut norm_len: libc::c_int = th_normalize(norm8, src8, n) as libc::c_int;
            i = 0 as libc::c_int;
            while left > 1 as libc::c_int as size_t && i < norm_len {
                let fresh1 = wdest;
                wdest = wdest.offset(1);
                *fresh1 = th_tis2uni(*norm8.offset(i as isize));
                left = left.wrapping_sub(1);
                left;
                i += 1;
                i;
            }
        } else {
            let mut i_0: libc::c_int = 0;
            chunk_len = -chunk_len;
            i_0 = 0 as libc::c_int;
            while left > 1 as libc::c_int as size_t && i_0 < chunk_len {
                let fresh2 = wdest;
                wdest = wdest.offset(1);
                *fresh2 = *wsrc.offset(i_0 as isize);
                left = left.wrapping_sub(1);
                left;
                i_0 += 1;
                i_0;
            }
        }
        wsrc = wsrc.offset(chunk_len as isize);
    }
    *wdest = 0 as libc::c_int;
    free(norm8 as *mut libc::c_void);
    free(src8 as *mut libc::c_void);
    return n.wrapping_sub(left);
}
