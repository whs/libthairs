use ::libc;
extern "C" {
    pub type _ThBrk;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn wcscpy(__dest: *mut wchar_t, __src: *const wchar_t) -> *mut wchar_t;
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    fn th_uni2tis_line(s: *const thwchar_t, result: *mut thchar_t, n: size_t) -> libc::c_int;
    fn th_brk_find_breaks(
        brk: *mut ThBrk,
        s: *const thchar_t,
        pos: *mut libc::c_int,
        pos_sz: size_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type thchar_t = libc::c_uchar;
pub type thwchar_t = wchar_t;
pub type ThBrk = _ThBrk;
pub const SIZE_MAX: libc::c_ulong = 18446744073709551615 as libc::c_ulong;
pub const NULL: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn th_brk_wc_find_breaks(
    mut brk: *mut ThBrk,
    mut s: *const thwchar_t,
    mut pos: *mut libc::c_int,
    mut pos_sz: size_t,
) -> libc::c_int {
    let mut tis_str: *mut thchar_t = 0 as *mut thchar_t;
    let mut alloc_size: size_t = 0;
    let mut ret: libc::c_int = 0;
    alloc_size = (wcslen(s)).wrapping_add(1 as libc::c_int as libc::c_ulong);
    tis_str = malloc(alloc_size) as *mut thchar_t;
    if tis_str.is_null() {
        return 0 as libc::c_int;
    }
    th_uni2tis_line(s, tis_str, alloc_size);
    ret = th_brk_find_breaks(brk, tis_str, pos, pos_sz);
    free(tis_str as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn th_brk_wc_insert_breaks(
    mut brk: *mut ThBrk,
    mut in_0: *const thwchar_t,
    mut out: *mut thwchar_t,
    mut out_sz: size_t,
    mut delim: *const thwchar_t,
) -> libc::c_int {
    let mut brk_pos: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut n_brk_pos: size_t = 0;
    let mut i: size_t = 0;
    let mut j: size_t = 0;
    let mut delim_len: libc::c_int = 0;
    let mut p_out: *mut thwchar_t = 0 as *mut thwchar_t;
    n_brk_pos = wcslen(in_0);
    if n_brk_pos > SIZE_MAX.wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong) {
        return 0 as libc::c_int;
    }
    brk_pos = malloc(n_brk_pos.wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong))
        as *mut libc::c_int;
    if brk_pos.is_null() {
        return 0 as libc::c_int;
    }
    n_brk_pos = th_brk_wc_find_breaks(brk, in_0, brk_pos, n_brk_pos) as size_t;
    delim_len = wcslen(delim) as libc::c_int;
    j = 0 as libc::c_int as size_t;
    i = j;
    p_out = out;
    while out_sz > 1 as libc::c_int as size_t && i < n_brk_pos {
        while out_sz > 1 as libc::c_int as size_t && j < *brk_pos.offset(i as isize) as size_t {
            let fresh0 = j;
            j = j.wrapping_add(1);
            let fresh1 = p_out;
            p_out = p_out.offset(1);
            *fresh1 = *in_0.offset(fresh0 as isize);
            out_sz = out_sz.wrapping_sub(1);
            out_sz;
        }
        if out_sz > (delim_len + 1 as libc::c_int) as size_t {
            wcscpy(p_out, delim);
            p_out = p_out.offset(delim_len as isize);
            out_sz = out_sz.wrapping_sub(delim_len as size_t);
        }
        i = i.wrapping_add(1);
        i;
    }
    while out_sz > 1 as libc::c_int as size_t && *in_0.offset(j as isize) != 0 {
        let fresh2 = j;
        j = j.wrapping_add(1);
        let fresh3 = p_out;
        p_out = p_out.offset(1);
        *fresh3 = *in_0.offset(fresh2 as isize);
        out_sz = out_sz.wrapping_sub(1);
        out_sz;
    }
    *p_out = 0 as libc::c_int;
    free(brk_pos as *mut libc::c_void);
    return p_out.offset_from(out) as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wbrk(
    mut s: *const thwchar_t,
    mut pos: *mut libc::c_int,
    mut pos_sz: size_t,
) -> libc::c_int {
    return th_brk_wc_find_breaks(NULL as *mut libc::c_void as *mut ThBrk, s, pos, pos_sz);
}
#[no_mangle]
pub unsafe extern "C" fn th_wbrk_line(
    mut in_0: *const thwchar_t,
    mut out: *mut thwchar_t,
    mut out_sz: size_t,
    mut delim: *const thwchar_t,
) -> libc::c_int {
    return th_brk_wc_insert_breaks(
        NULL as *mut libc::c_void as *mut ThBrk,
        in_0,
        out,
        out_sz,
        delim,
    );
}
