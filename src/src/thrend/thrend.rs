use ::libc;
extern "C" {
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn th_next_cell(
        s: *const thchar_t,
        len: size_t,
        cell: *mut thcell_t,
        is_decomp_am: libc::c_int,
    ) -> size_t;
    static _th_chlevel_tbl: [libc::c_int; 0];
    static _th_ctype_tbl: [libc::c_ushort; 0];
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
pub type thglyph_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ThaiShapeTable {
    pub ShiftDown_TONE_AD: [thglyph_t; 8],
    pub ShiftDownLeft_TONE_AD: [thglyph_t; 8],
    pub ShiftLeft_TONE_AD: [thglyph_t; 8],
    pub ShiftLeft_AV: [thglyph_t; 7],
    pub ShiftDown_BV_BD: [thglyph_t; 3],
    pub TailCutCons: [thglyph_t; 4],
}
pub const _th_CCovershoot: C2RustUnnamed = 6;
pub const _th_CClassMsk: C2RustUnnamed = 14;
pub const _th_VCupvowel: C2RustUnnamed = 80;
pub const _th_VClassMsk: C2RustUnnamed = 112;
pub const _th_CCundershoot: C2RustUnnamed = 10;
pub const _th_CCundersplit: C2RustUnnamed = 14;
pub type C2RustUnnamed = libc::c_uint;
pub const _th_ISpunct: C2RustUnnamed = 1024;
pub const _th_ISdigit: C2RustUnnamed = 512;
pub const _th_ISdiac: C2RustUnnamed = 256;
pub const _th_IStone: C2RustUnnamed = 128;
pub const _th_VCblvowel: C2RustUnnamed = 112;
pub const _th_VCldvowel: C2RustUnnamed = 48;
pub const _th_VCflvowel: C2RustUnnamed = 16;
pub const _th_ISvowel: C2RustUnnamed = 16;
pub const _th_CCtailless: C2RustUnnamed = 2;
pub const _th_IScons: C2RustUnnamed = 2;
pub const _th_IStis: C2RustUnnamed = 1;
pub const TH_BLANK_BASE_GLYPH: libc::c_int = 0xdd as libc::c_int;
pub const TIS_YO_YING: libc::c_int = 0xad as libc::c_int;
pub const TIS_MAI_HAN_AKAT: libc::c_int = 0xd1 as libc::c_int;
pub const TIS_SARA_AA: libc::c_int = 0xd2 as libc::c_int;
pub const TIS_SARA_AM: libc::c_int = 0xd3 as libc::c_int;
pub const TIS_SARA_U: libc::c_int = 0xd8 as libc::c_int;
pub const TIS_MAITAIKHU: libc::c_int = 0xe7 as libc::c_int;
pub const TIS_NIKHAHIT: libc::c_int = 0xed as libc::c_int;
static mut Mac_shape_table_: ThaiShapeTable = {
    let mut init = ThaiShapeTable {
        ShiftDown_TONE_AD: [
            0xe7 as libc::c_int as thglyph_t,
            0x88 as libc::c_int as thglyph_t,
            0x89 as libc::c_int as thglyph_t,
            0x8a as libc::c_int as thglyph_t,
            0x8b as libc::c_int as thglyph_t,
            0x8c as libc::c_int as thglyph_t,
            0xed as libc::c_int as thglyph_t,
            0xee as libc::c_int as thglyph_t,
        ],
        ShiftDownLeft_TONE_AD: [
            0x93 as libc::c_int as thglyph_t,
            0x83 as libc::c_int as thglyph_t,
            0x84 as libc::c_int as thglyph_t,
            0x85 as libc::c_int as thglyph_t,
            0x86 as libc::c_int as thglyph_t,
            0x87 as libc::c_int as thglyph_t,
            0x8f as libc::c_int as thglyph_t,
            0xee as libc::c_int as thglyph_t,
        ],
        ShiftLeft_TONE_AD: [
            0x93 as libc::c_int as thglyph_t,
            0x98 as libc::c_int as thglyph_t,
            0x99 as libc::c_int as thglyph_t,
            0x9a as libc::c_int as thglyph_t,
            0x9b as libc::c_int as thglyph_t,
            0x9c as libc::c_int as thglyph_t,
            0x8f as libc::c_int as thglyph_t,
            0xee as libc::c_int as thglyph_t,
        ],
        ShiftLeft_AV: [
            0x92 as libc::c_int as thglyph_t,
            0 as libc::c_int as thglyph_t,
            0 as libc::c_int as thglyph_t,
            0x94 as libc::c_int as thglyph_t,
            0x95 as libc::c_int as thglyph_t,
            0x96 as libc::c_int as thglyph_t,
            0x97 as libc::c_int as thglyph_t,
        ],
        ShiftDown_BV_BD: [
            0xfc as libc::c_int as thglyph_t,
            0xfd as libc::c_int as thglyph_t,
            0xfe as libc::c_int as thglyph_t,
        ],
        TailCutCons: [
            0x90 as libc::c_int as thglyph_t,
            0 as libc::c_int as thglyph_t,
            0 as libc::c_int as thglyph_t,
            0x80 as libc::c_int as thglyph_t,
        ],
    };
    init
};
static mut Win_shape_table_: ThaiShapeTable = {
    let mut init = ThaiShapeTable {
        ShiftDown_TONE_AD: [
            0xe7 as libc::c_int as thglyph_t,
            0x8b as libc::c_int as thglyph_t,
            0x8c as libc::c_int as thglyph_t,
            0x8d as libc::c_int as thglyph_t,
            0x8e as libc::c_int as thglyph_t,
            0x8f as libc::c_int as thglyph_t,
            0xed as libc::c_int as thglyph_t,
            0xee as libc::c_int as thglyph_t,
        ],
        ShiftDownLeft_TONE_AD: [
            0x9a as libc::c_int as thglyph_t,
            0x86 as libc::c_int as thglyph_t,
            0x87 as libc::c_int as thglyph_t,
            0x88 as libc::c_int as thglyph_t,
            0x89 as libc::c_int as thglyph_t,
            0x8a as libc::c_int as thglyph_t,
            0x99 as libc::c_int as thglyph_t,
            0xee as libc::c_int as thglyph_t,
        ],
        ShiftLeft_TONE_AD: [
            0x9a as libc::c_int as thglyph_t,
            0x9b as libc::c_int as thglyph_t,
            0x9c as libc::c_int as thglyph_t,
            0x9d as libc::c_int as thglyph_t,
            0x9e as libc::c_int as thglyph_t,
            0x9f as libc::c_int as thglyph_t,
            0x99 as libc::c_int as thglyph_t,
            0xee as libc::c_int as thglyph_t,
        ],
        ShiftLeft_AV: [
            0x98 as libc::c_int as thglyph_t,
            0 as libc::c_int as thglyph_t,
            0 as libc::c_int as thglyph_t,
            0x81 as libc::c_int as thglyph_t,
            0x82 as libc::c_int as thglyph_t,
            0x83 as libc::c_int as thglyph_t,
            0x84 as libc::c_int as thglyph_t,
        ],
        ShiftDown_BV_BD: [
            0xfc as libc::c_int as thglyph_t,
            0xfd as libc::c_int as thglyph_t,
            0xfe as libc::c_int as thglyph_t,
        ],
        TailCutCons: [
            0x90 as libc::c_int as thglyph_t,
            0 as libc::c_int as thglyph_t,
            0 as libc::c_int as thglyph_t,
            0x80 as libc::c_int as thglyph_t,
        ],
    };
    init
};
unsafe extern "C" fn th_render_cell_(
    mut cell: thcell_t,
    mut res: *mut thglyph_t,
    mut res_sz: size_t,
    mut is_decomp_am: libc::c_int,
    mut tbl: *const ThaiShapeTable,
) -> libc::c_int {
    let mut left: size_t = res_sz;
    if left > 0 as libc::c_int as size_t {
        let mut c: thchar_t = cell.base;
        if *_th_ctype_tbl.as_ptr().offset(c as isize) as libc::c_int & _th_CClassMsk as libc::c_int
            == _th_CCundersplit as libc::c_int
            && *_th_chlevel_tbl.as_ptr().offset(cell.hilo as isize) < 0 as libc::c_int
        {
            c = (*tbl).TailCutCons[(c as libc::c_int - TIS_YO_YING) as usize];
        }
        let fresh0 = res;
        res = res.offset(1);
        *fresh0 = (if c as libc::c_int != 0 {
            c as libc::c_int
        } else {
            TH_BLANK_BASE_GLYPH
        }) as thglyph_t;
        left = left.wrapping_sub(1);
        left;
    }
    if left > 0 as libc::c_int as size_t && cell.hilo as libc::c_int != 0 {
        let mut c_0: thchar_t = 0 as libc::c_int as thchar_t;
        if cell.hilo as libc::c_int != TIS_SARA_AM {
            c_0 = cell.hilo;
        } else if is_decomp_am != 0 {
            c_0 = TIS_NIKHAHIT as thchar_t;
        }
        if c_0 != 0 {
            if *_th_ctype_tbl.as_ptr().offset(cell.base as isize) as libc::c_int
                & _th_CClassMsk as libc::c_int
                == _th_CCovershoot as libc::c_int
                && *_th_chlevel_tbl.as_ptr().offset(c_0 as isize) > 0 as libc::c_int
            {
                c_0 = (if *_th_ctype_tbl.as_ptr().offset(c_0 as isize) as libc::c_int
                    & _th_VClassMsk as libc::c_int
                    == _th_VCupvowel as libc::c_int
                {
                    (*tbl).ShiftLeft_AV[(c_0 as libc::c_int - TIS_MAI_HAN_AKAT) as usize]
                        as libc::c_int
                } else {
                    (*tbl).ShiftLeft_TONE_AD[(c_0 as libc::c_int - TIS_MAITAIKHU) as usize]
                        as libc::c_int
                }) as thchar_t;
            } else if *_th_ctype_tbl.as_ptr().offset(cell.base as isize) as libc::c_int
                & _th_CClassMsk as libc::c_int
                == _th_CCundershoot as libc::c_int
                && *_th_chlevel_tbl.as_ptr().offset(c_0 as isize) < 0 as libc::c_int
            {
                c_0 = (*tbl).ShiftDown_BV_BD[(c_0 as libc::c_int - TIS_SARA_U) as usize];
            }
            let fresh1 = res;
            res = res.offset(1);
            *fresh1 = c_0;
            left = left.wrapping_sub(1);
            left;
        }
    }
    if left > 0 as libc::c_int as size_t && cell.top as libc::c_int != 0 {
        let mut c_1: thchar_t = cell.top;
        if *_th_ctype_tbl.as_ptr().offset(cell.hilo as isize) as libc::c_int
            & _th_VClassMsk as libc::c_int
            == _th_VCupvowel as libc::c_int
            || is_decomp_am != 0 && cell.hilo as libc::c_int == TIS_SARA_AM
        {
            c_1 = (if *_th_ctype_tbl.as_ptr().offset(cell.base as isize) as libc::c_int
                & _th_CClassMsk as libc::c_int
                == _th_CCovershoot as libc::c_int
            {
                (*tbl).ShiftLeft_TONE_AD[(c_1 as libc::c_int - TIS_MAITAIKHU) as usize]
                    as libc::c_int
            } else {
                c_1 as libc::c_int
            }) as thchar_t;
        } else {
            c_1 = (if *_th_ctype_tbl.as_ptr().offset(cell.base as isize) as libc::c_int
                & _th_CClassMsk as libc::c_int
                == _th_CCovershoot as libc::c_int
            {
                (*tbl).ShiftDownLeft_TONE_AD[(c_1 as libc::c_int - TIS_MAITAIKHU) as usize]
                    as libc::c_int
            } else {
                (*tbl).ShiftDown_TONE_AD[(c_1 as libc::c_int - TIS_MAITAIKHU) as usize]
                    as libc::c_int
            }) as thchar_t;
        }
        let fresh2 = res;
        res = res.offset(1);
        *fresh2 = c_1;
        left = left.wrapping_sub(1);
        left;
    }
    if left > 0 as libc::c_int as size_t && cell.hilo as libc::c_int == TIS_SARA_AM {
        let fresh3 = res;
        res = res.offset(1);
        *fresh3 = (if is_decomp_am != 0 {
            TIS_SARA_AA
        } else {
            TIS_SARA_AM
        }) as thglyph_t;
        left = left.wrapping_sub(1);
        left;
    }
    if left > 0 as libc::c_int as size_t {
        *res = 0 as libc::c_int as thglyph_t;
    }
    return res_sz.wrapping_sub(left) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_render_cell_tis(
    mut cell: thcell_t,
    mut res: *mut thglyph_t,
    mut res_sz: size_t,
    mut is_decomp_am: libc::c_int,
) -> libc::c_int {
    let mut left: size_t = res_sz;
    if left > 0 as libc::c_int as size_t {
        let fresh4 = res;
        res = res.offset(1);
        *fresh4 = (if cell.base as libc::c_int != 0 {
            cell.base as libc::c_int
        } else {
            TH_BLANK_BASE_GLYPH
        }) as thglyph_t;
        left = left.wrapping_sub(1);
        left;
    }
    if left > 0 as libc::c_int as size_t && cell.hilo as libc::c_int != 0 {
        if cell.hilo as libc::c_int != TIS_SARA_AM {
            let fresh5 = res;
            res = res.offset(1);
            *fresh5 = cell.hilo;
            left = left.wrapping_sub(1);
            left;
        } else if is_decomp_am != 0 {
            let fresh6 = res;
            res = res.offset(1);
            *fresh6 = TIS_NIKHAHIT as thglyph_t;
            left = left.wrapping_sub(1);
            left;
        }
    }
    if left > 0 as libc::c_int as size_t && cell.top as libc::c_int != 0 {
        let fresh7 = res;
        res = res.offset(1);
        *fresh7 = cell.top;
        left = left.wrapping_sub(1);
        left;
    }
    if left > 0 as libc::c_int as size_t && cell.hilo as libc::c_int == TIS_SARA_AM {
        let fresh8 = res;
        res = res.offset(1);
        *fresh8 = (if is_decomp_am != 0 {
            TIS_SARA_AA
        } else {
            TIS_SARA_AM
        }) as thglyph_t;
        left = left.wrapping_sub(1);
        left;
    }
    if left > 0 as libc::c_int as size_t {
        *res = 0 as libc::c_int as thglyph_t;
    }
    return res_sz.wrapping_sub(left) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_render_cell_win(
    mut cell: thcell_t,
    mut res: *mut thglyph_t,
    mut res_sz: size_t,
    mut is_decomp_am: libc::c_int,
) -> libc::c_int {
    return th_render_cell_(cell, res, res_sz, is_decomp_am, &Win_shape_table_);
}
#[no_mangle]
pub unsafe extern "C" fn th_render_cell_mac(
    mut cell: thcell_t,
    mut res: *mut thglyph_t,
    mut res_sz: size_t,
    mut is_decomp_am: libc::c_int,
) -> libc::c_int {
    return th_render_cell_(cell, res, res_sz, is_decomp_am, &Mac_shape_table_);
}
unsafe extern "C" fn th_render_text(
    mut s: *const thchar_t,
    mut res: *mut thglyph_t,
    mut res_sz: size_t,
    mut is_decomp_am: libc::c_int,
    mut cell_rend_fn: Option<
        unsafe extern "C" fn(thcell_t, *mut thglyph_t, size_t, libc::c_int) -> libc::c_int,
    >,
) -> libc::c_int {
    let mut left: size_t = res_sz;
    let mut len: libc::c_int = strlen(s as *const libc::c_char) as libc::c_int;
    while left > 0 as libc::c_int as size_t && len > 0 as libc::c_int {
        let mut cell: thcell_t = thcell_t {
            base: 0,
            hilo: 0,
            top: 0,
        };
        let mut nchars: size_t = 0;
        let mut nglyphs: libc::c_int = 0;
        nchars = th_next_cell(s, len as size_t, &mut cell, is_decomp_am);
        s = s.offset(nchars as isize);
        len = (len as size_t).wrapping_sub(nchars) as libc::c_int as libc::c_int;
        nglyphs = (Some(cell_rend_fn.expect("non-null function pointer")))
            .expect("non-null function pointer")(cell, res, left, is_decomp_am);
        res = res.offset(nglyphs as isize);
        left = left.wrapping_sub(nglyphs as size_t);
    }
    return res_sz.wrapping_sub(left) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_render_text_tis(
    mut s: *const thchar_t,
    mut res: *mut thglyph_t,
    mut res_sz: size_t,
    mut is_decomp_am: libc::c_int,
) -> libc::c_int {
    return th_render_text(
        s,
        res,
        res_sz,
        is_decomp_am,
        Some(
            th_render_cell_tis
                as unsafe extern "C" fn(
                    thcell_t,
                    *mut thglyph_t,
                    size_t,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn th_render_text_win(
    mut s: *const thchar_t,
    mut res: *mut thglyph_t,
    mut res_sz: size_t,
    mut is_decomp_am: libc::c_int,
) -> libc::c_int {
    return th_render_text(
        s,
        res,
        res_sz,
        is_decomp_am,
        Some(
            th_render_cell_win
                as unsafe extern "C" fn(
                    thcell_t,
                    *mut thglyph_t,
                    size_t,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    );
}
#[no_mangle]
pub unsafe extern "C" fn th_render_text_mac(
    mut s: *const thchar_t,
    mut res: *mut thglyph_t,
    mut res_sz: size_t,
    mut is_decomp_am: libc::c_int,
) -> libc::c_int {
    return th_render_text(
        s,
        res,
        res_sz,
        is_decomp_am,
        Some(
            th_render_cell_mac
                as unsafe extern "C" fn(
                    thcell_t,
                    *mut thglyph_t,
                    size_t,
                    libc::c_int,
                ) -> libc::c_int,
        ),
    );
}
