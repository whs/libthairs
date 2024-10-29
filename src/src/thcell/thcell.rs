use ::libc;
extern "C" {
    static _th_ctype_tbl: [libc::c_ushort; 0];
    static _th_chlevel_tbl: [libc::c_int; 0];
    static mut TACchtype_: [libc::c_short; 256];
    static mut TACio_op_: [[libc::c_short; 17]; 17];
}
pub type size_t = libc::c_ulong;
pub type thchar_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thcell_t {
    pub base: thchar_t,
    pub hilo: thchar_t,
    pub top: thchar_t,
}
pub const _th_IScons: C2RustUnnamed = 2;
pub const CP: C2RustUnnamed_0 = 1;
pub const _th_IStone: C2RustUnnamed = 128;
pub type C2RustUnnamed = libc::c_uint;
pub const _th_ISpunct: C2RustUnnamed = 1024;
pub const _th_ISdigit: C2RustUnnamed = 512;
pub const _th_ISdiac: C2RustUnnamed = 256;
pub const _th_VCblvowel: C2RustUnnamed = 112;
pub const _th_VCupvowel: C2RustUnnamed = 80;
pub const _th_VCldvowel: C2RustUnnamed = 48;
pub const _th_VCflvowel: C2RustUnnamed = 16;
pub const _th_VClassMsk: C2RustUnnamed = 112;
pub const _th_ISvowel: C2RustUnnamed = 16;
pub const _th_CCundersplit: C2RustUnnamed = 14;
pub const _th_CCundershoot: C2RustUnnamed = 10;
pub const _th_CCovershoot: C2RustUnnamed = 6;
pub const _th_CCtailless: C2RustUnnamed = 2;
pub const _th_CClassMsk: C2RustUnnamed = 14;
pub const _th_IStis: C2RustUnnamed = 1;
pub type C2RustUnnamed_0 = libc::c_uint;
pub const SR: C2RustUnnamed_0 = 5;
pub const RJ: C2RustUnnamed_0 = 4;
pub const AC: C2RustUnnamed_0 = 3;
pub const XC: C2RustUnnamed_0 = 2;
pub const TIS_SARA_AM: libc::c_int = 0xd3 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn th_init_cell(mut cell: *mut thcell_t) {
    (*cell).base = 0 as libc::c_int as thchar_t;
    (*cell).hilo = 0 as libc::c_int as thchar_t;
    (*cell).top = 0 as libc::c_int as thchar_t;
}
#[no_mangle]
pub unsafe extern "C" fn th_next_cell(
    mut s: *const thchar_t,
    mut len: size_t,
    mut cell: *mut thcell_t,
    mut is_decomp_am: libc::c_int,
) -> size_t {
    let mut n: size_t = 0 as libc::c_int as size_t;
    let mut acell: thcell_t = thcell_t {
        base: 0,
        hilo: 0,
        top: 0,
    };
    acell.top = 0 as libc::c_int as thchar_t;
    acell.hilo = acell.top;
    acell.base = acell.hilo;
    if len > 0 as libc::c_int as size_t {
        loop {
            match *_th_chlevel_tbl.as_ptr().offset(*s as isize) {
                0 => {
                    if is_decomp_am != 0 && *s as libc::c_int == TIS_SARA_AM {
                        let fresh0 = s;
                        s = s.offset(1);
                        acell.hilo = *fresh0;
                    } else {
                        let fresh1 = s;
                        s = s.offset(1);
                        acell.base = *fresh1;
                    }
                }
                -1 | 1 => {
                    let fresh2 = s;
                    s = s.offset(1);
                    acell.hilo = *fresh2;
                }
                2 => {
                    let fresh3 = s;
                    s = s.offset(1);
                    acell.top = *fresh3;
                }
                3 => {
                    if acell.hilo == 0 {
                        let fresh4 = s;
                        s = s.offset(1);
                        acell.hilo = *fresh4;
                    } else {
                        let fresh5 = s;
                        s = s.offset(1);
                        acell.top = *fresh5;
                    }
                }
                _ => {}
            }
            n = n.wrapping_add(1);
            n;
            len = len.wrapping_sub(1);
            len;
            if !(len > 0 as libc::c_int as size_t
                && (TACio_op_[TACchtype_[*s.offset(-(1 as libc::c_int) as isize) as usize] as usize]
                    [TACchtype_[*s.offset(0 as libc::c_int as isize) as usize] as usize]
                    as libc::c_int
                    == CP as libc::c_int
                    || is_decomp_am != 0
                        && *s.offset(0 as libc::c_int as isize) as libc::c_int == TIS_SARA_AM
                        && *_th_ctype_tbl.as_ptr().offset(acell.base as isize) as libc::c_int
                            & _th_IScons as libc::c_int
                            != 0
                        && acell.hilo as libc::c_int == 0 as libc::c_int))
            {
                break;
            }
        }
    }
    if !cell.is_null() {
        *cell = acell;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn th_prev_cell(
    mut s: *const thchar_t,
    mut pos: size_t,
    mut cell: *mut thcell_t,
    mut is_decomp_am: libc::c_int,
) -> size_t {
    let mut n: size_t = 0 as libc::c_int as size_t;
    let mut acell: thcell_t = thcell_t {
        base: 0,
        hilo: 0,
        top: 0,
    };
    acell.top = 0 as libc::c_int as thchar_t;
    acell.hilo = acell.top;
    acell.base = acell.hilo;
    if pos > 0 as libc::c_int as size_t {
        loop {
            let mut c: thchar_t = *s.offset(pos.wrapping_sub(1 as libc::c_int as size_t) as isize);
            let mut current_block_15: u64;
            match *_th_chlevel_tbl.as_ptr().offset(c as isize) {
                0 => {
                    if is_decomp_am != 0 && c as libc::c_int == TIS_SARA_AM {
                        acell.hilo = c;
                    } else {
                        acell.base = c;
                    }
                    current_block_15 = 2370887241019905314;
                }
                1 => {
                    if acell.hilo as libc::c_int != 0
                        && *_th_chlevel_tbl.as_ptr().offset(acell.hilo as isize) == 3 as libc::c_int
                    {
                        acell.top = acell.hilo;
                    }
                    current_block_15 = 17225958594799427083;
                }
                -1 => {
                    current_block_15 = 17225958594799427083;
                }
                2 => {
                    acell.top = c;
                    current_block_15 = 2370887241019905314;
                }
                3 => {
                    if acell.hilo == 0 {
                        acell.hilo = c;
                    } else {
                        acell.top = c;
                    }
                    current_block_15 = 2370887241019905314;
                }
                _ => {
                    current_block_15 = 2370887241019905314;
                }
            }
            match current_block_15 {
                17225958594799427083 => {
                    acell.hilo = c;
                }
                _ => {}
            }
            n = n.wrapping_add(1);
            n;
            pos = pos.wrapping_sub(1);
            pos;
            if !(pos > 0 as libc::c_int as size_t
                && (TACio_op_[TACchtype_
                    [*s.offset(pos.wrapping_sub(1 as libc::c_int as size_t) as isize) as usize]
                    as usize][TACchtype_[*s.offset(pos as isize) as usize] as usize]
                    as libc::c_int
                    == CP as libc::c_int
                    || acell.hilo as libc::c_int == TIS_SARA_AM
                        && acell.base == 0
                        && (acell.top == 0
                            && *_th_ctype_tbl.as_ptr().offset(
                                *s.offset(pos.wrapping_sub(1 as libc::c_int as size_t) as isize)
                                    as isize,
                            ) as libc::c_int
                                & _th_IStone as libc::c_int
                                != 0
                            || *_th_ctype_tbl.as_ptr().offset(
                                *s.offset(pos.wrapping_sub(1 as libc::c_int as size_t) as isize)
                                    as isize,
                            ) as libc::c_int
                                & _th_IScons as libc::c_int
                                != 0)))
            {
                break;
            }
        }
    }
    if !cell.is_null() {
        *cell = acell;
    }
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn th_make_cells(
    mut s: *const thchar_t,
    mut len: size_t,
    mut cells: *mut thcell_t,
    mut ncells: *mut size_t,
    mut is_decomp_am: libc::c_int,
) -> size_t {
    let mut left: size_t = *ncells;
    let mut nchars: size_t = 0 as libc::c_int as size_t;
    while len > 0 as libc::c_int as size_t && left > 0 as libc::c_int as size_t {
        let mut n: size_t = th_next_cell(s.offset(nchars as isize), len, cells, is_decomp_am);
        nchars = nchars.wrapping_add(n);
        len = len.wrapping_sub(n);
        cells = cells.offset(1);
        cells;
        left = left.wrapping_sub(1);
        left;
    }
    *ncells = (*ncells).wrapping_sub(left);
    return nchars;
}
