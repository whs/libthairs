use ::libc;
extern "C" {
    static _th_chlevel_tbl: [libc::c_int; 0];
}
pub type size_t = libc::c_ulong;
pub type thchar_t = libc::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn th_normalize(
    mut dest: *mut thchar_t,
    mut src: *const thchar_t,
    mut n: size_t,
) -> size_t {
    let mut top: thchar_t = 0;
    let mut up: thchar_t = 0;
    let mut middle: thchar_t = 0;
    let mut low: thchar_t = 0;
    let mut left: size_t = n;
    low = 0 as libc::c_int as thchar_t;
    middle = low;
    up = middle;
    top = up;
    while *src as libc::c_int != 0 && left > 1 as libc::c_int as size_t {
        match *_th_chlevel_tbl.as_ptr().offset(*src as isize) {
            0 => {
                if middle != 0 {
                    let fresh0 = dest;
                    dest = dest.offset(1);
                    *fresh0 = middle;
                    left = left.wrapping_sub(1);
                    left;
                    if left > 1 as libc::c_int as size_t {
                        if low != 0 {
                            let fresh1 = dest;
                            dest = dest.offset(1);
                            *fresh1 = low;
                            left = left.wrapping_sub(1);
                            left;
                        } else if up != 0 {
                            let fresh2 = dest;
                            dest = dest.offset(1);
                            *fresh2 = up;
                            left = left.wrapping_sub(1);
                            left;
                        }
                    }
                    if left > 1 as libc::c_int as size_t && top as libc::c_int != 0 {
                        let fresh3 = dest;
                        dest = dest.offset(1);
                        *fresh3 = top;
                        left = left.wrapping_sub(1);
                        left;
                    }
                }
                low = 0 as libc::c_int as thchar_t;
                up = low;
                top = up;
                middle = *src;
            }
            -1 => {
                low = *src;
            }
            1 => {
                if up as libc::c_int != 0
                    && *_th_chlevel_tbl.as_ptr().offset(up as isize) == 3 as libc::c_int
                {
                    top = up;
                }
                up = *src;
            }
            2 => {
                top = *src;
            }
            3 => {
                if up == 0 {
                    up = *src;
                } else {
                    top = *src;
                }
            }
            _ => {}
        }
        src = src.offset(1);
        src;
    }
    if left > 1 as libc::c_int as size_t && middle as libc::c_int != 0 {
        let fresh4 = dest;
        dest = dest.offset(1);
        *fresh4 = middle;
        left = left.wrapping_sub(1);
        left;
        if left > 1 as libc::c_int as size_t {
            if low != 0 {
                let fresh5 = dest;
                dest = dest.offset(1);
                *fresh5 = low;
                left = left.wrapping_sub(1);
                left;
            } else if up != 0 {
                let fresh6 = dest;
                dest = dest.offset(1);
                *fresh6 = up;
                left = left.wrapping_sub(1);
                left;
            }
        }
        if left > 1 as libc::c_int as size_t && top as libc::c_int != 0 {
            let fresh7 = dest;
            dest = dest.offset(1);
            *fresh7 = top;
            left = left.wrapping_sub(1);
            left;
        }
    }
    *dest = 0 as libc::c_int as thchar_t;
    return n.wrapping_sub(left);
}
