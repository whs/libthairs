use ::libc;
extern "C" {
    static _th_ctype_tbl: [libc::c_ushort; 0];
    fn th_char_weight_(c: thchar_t, level: libc::c_int) -> thchar_t;
    fn th_char_weight_delim_(level: libc::c_int) -> thchar_t;
}
pub type size_t = libc::c_ulong;
pub type thchar_t = libc::c_uchar;
pub const _th_VCldvowel: C2RustUnnamed = 48;
pub const _th_VClassMsk: C2RustUnnamed = 112;
pub type C2RustUnnamed = libc::c_uint;
pub const _th_ISpunct: C2RustUnnamed = 1024;
pub const _th_ISdigit: C2RustUnnamed = 512;
pub const _th_ISdiac: C2RustUnnamed = 256;
pub const _th_IStone: C2RustUnnamed = 128;
pub const _th_VCblvowel: C2RustUnnamed = 112;
pub const _th_VCupvowel: C2RustUnnamed = 80;
pub const _th_VCflvowel: C2RustUnnamed = 16;
pub const _th_ISvowel: C2RustUnnamed = 16;
pub const _th_CCundersplit: C2RustUnnamed = 14;
pub const _th_CCundershoot: C2RustUnnamed = 10;
pub const _th_CCovershoot: C2RustUnnamed = 6;
pub const _th_CCtailless: C2RustUnnamed = 2;
pub const _th_CClassMsk: C2RustUnnamed = 14;
pub const _th_IScons: C2RustUnnamed = 2;
pub const _th_IStis: C2RustUnnamed = 1;
pub const TOT_LEVELS: libc::c_int = 4 as libc::c_int;
pub const IGNORE: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn th_non_ignore_(
    mut p: *const thchar_t,
    mut level: libc::c_int,
) -> *const thchar_t {
    while *p as libc::c_int != 0 && th_char_weight_(*p, level) as libc::c_int == IGNORE {
        p = p.offset(1);
        p;
    }
    return p;
}
#[no_mangle]
pub unsafe extern "C" fn th_strcoll(
    mut s1: *const thchar_t,
    mut s2: *const thchar_t,
) -> libc::c_int {
    let mut p1: *const thchar_t = 0 as *const thchar_t;
    let mut p2: *const thchar_t = 0 as *const thchar_t;
    let mut l: libc::c_int = 0;
    while *s1 as libc::c_int == *s2 as libc::c_int && *s1 as libc::c_int != 0 as libc::c_int {
        s1 = s1.offset(1);
        s1;
        s2 = s2.offset(1);
        s2;
    }
    if *s1 as libc::c_int == 0 as libc::c_int && *s2 as libc::c_int == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    p1 = th_non_ignore_(s1, 0 as libc::c_int);
    p2 = th_non_ignore_(s2, 0 as libc::c_int);
    while *p1 as libc::c_int != 0 && *p2 as libc::c_int != 0 {
        let mut w1: thchar_t = th_char_weight_(*p1, 0 as libc::c_int);
        let mut w2: thchar_t = th_char_weight_(*p2, 0 as libc::c_int);
        if *_th_ctype_tbl.as_ptr().offset(*p1 as isize) as libc::c_int
            & _th_VClassMsk as libc::c_int
            == _th_VCldvowel as libc::c_int
        {
            let mut q1: *const thchar_t =
                th_non_ignore_(p1.offset(1 as libc::c_int as isize), 0 as libc::c_int);
            let mut ww1: thchar_t = th_char_weight_(*q1, 0 as libc::c_int);
            if *_th_ctype_tbl.as_ptr().offset(*p2 as isize) as libc::c_int
                & _th_VClassMsk as libc::c_int
                == _th_VCldvowel as libc::c_int
            {
                let mut q2: *const thchar_t =
                    th_non_ignore_(p2.offset(1 as libc::c_int as isize), 0 as libc::c_int);
                let mut ww2: thchar_t = th_char_weight_(*q2, 0 as libc::c_int);
                if ww1 as libc::c_int != ww2 as libc::c_int {
                    return ww1 as libc::c_int - ww2 as libc::c_int;
                }
                if w1 as libc::c_int != w2 as libc::c_int {
                    return w1 as libc::c_int - w2 as libc::c_int;
                }
                p1 = th_non_ignore_(q1.offset(1 as libc::c_int as isize), 0 as libc::c_int);
                p2 = th_non_ignore_(q2.offset(1 as libc::c_int as isize), 0 as libc::c_int);
            } else {
                return if ww1 as libc::c_int != w2 as libc::c_int {
                    ww1 as libc::c_int - w2 as libc::c_int
                } else {
                    1 as libc::c_int
                };
            }
        } else if *_th_ctype_tbl.as_ptr().offset(*p2 as isize) as libc::c_int
            & _th_VClassMsk as libc::c_int
            == _th_VCldvowel as libc::c_int
        {
            let mut q2_0: *const thchar_t =
                th_non_ignore_(p2.offset(1 as libc::c_int as isize), 0 as libc::c_int);
            let mut ww2_0: thchar_t = th_char_weight_(*q2_0, 0 as libc::c_int);
            return if w1 as libc::c_int != ww2_0 as libc::c_int {
                w1 as libc::c_int - ww2_0 as libc::c_int
            } else {
                -(1 as libc::c_int)
            };
        } else {
            if w1 as libc::c_int != w2 as libc::c_int {
                return w1 as libc::c_int - w2 as libc::c_int;
            }
            p1 = th_non_ignore_(p1.offset(1 as libc::c_int as isize), 0 as libc::c_int);
            p2 = th_non_ignore_(p2.offset(1 as libc::c_int as isize), 0 as libc::c_int);
        }
    }
    if *p1 as libc::c_int != 0 as libc::c_int {
        return 1 as libc::c_int;
    }
    if *p2 as libc::c_int != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    l = 1 as libc::c_int;
    while l < TOT_LEVELS {
        p1 = th_non_ignore_(s1, l);
        p2 = th_non_ignore_(s2, l);
        while *p1 as libc::c_int != 0 && *p2 as libc::c_int != 0 {
            let mut w1_0: thchar_t = th_char_weight_(*p1, l);
            let mut w2_0: thchar_t = th_char_weight_(*p2, l);
            if w1_0 as libc::c_int != w2_0 as libc::c_int {
                return w1_0 as libc::c_int - w2_0 as libc::c_int;
            }
            p1 = th_non_ignore_(p1.offset(1 as libc::c_int as isize), l);
            p2 = th_non_ignore_(p2.offset(1 as libc::c_int as isize), l);
        }
        if *p1 as libc::c_int != 0 as libc::c_int {
            return 1 as libc::c_int;
        }
        if *p2 as libc::c_int != 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
        l += 1;
        l;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn th_put_weight_(
    mut dst: *mut thchar_t,
    mut n: size_t,
    mut w: thchar_t,
    mut d: *mut libc::c_int,
) -> libc::c_int {
    if (*d as size_t) < n.wrapping_sub(1 as libc::c_int as size_t) {
        if !dst.is_null() {
            *dst.offset(*d as isize) = w;
        }
        *d += 1;
        *d;
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_strxfrm(
    mut dest: *mut thchar_t,
    mut src: *const thchar_t,
    mut n: size_t,
) -> size_t {
    let mut p: *const thchar_t = 0 as *const thchar_t;
    let mut d: libc::c_int = 0 as libc::c_int;
    let mut l: libc::c_int = 0;
    p = th_non_ignore_(src, 0 as libc::c_int);
    while *p != 0 {
        let mut w: thchar_t = th_char_weight_(*p, 0 as libc::c_int);
        if *_th_ctype_tbl.as_ptr().offset(*p as isize) as libc::c_int & _th_VClassMsk as libc::c_int
            == _th_VCldvowel as libc::c_int
        {
            p = th_non_ignore_(p.offset(1 as libc::c_int as isize), 0 as libc::c_int);
            if *p != 0 {
                if th_put_weight_(dest, n, th_char_weight_(*p, 0 as libc::c_int), &mut d) == 0 {
                    return d as size_t;
                }
                if th_put_weight_(dest, n, w, &mut d) == 0 {
                    return d as size_t;
                }
            }
        } else {
            if th_put_weight_(dest, n, w, &mut d) == 0 {
                return d as size_t;
            }
            p = th_non_ignore_(p.offset(1 as libc::c_int as isize), 0 as libc::c_int);
        }
    }
    l = 1 as libc::c_int;
    while l < TOT_LEVELS {
        if th_put_weight_(dest, n, th_char_weight_delim_(l), &mut d) == 0 {
            return d as size_t;
        }
        p = src;
        while *p != 0 {
            if th_put_weight_(dest, n, th_char_weight_(*p, l), &mut d) == 0 {
                return d as size_t;
            }
            p = th_non_ignore_(p.offset(1 as libc::c_int as isize), l);
        }
        l += 1;
        l;
    }
    if !dest.is_null() {
        *dest.offset(d as isize) = '\0' as i32 as thchar_t;
    }
    return d as size_t;
}
