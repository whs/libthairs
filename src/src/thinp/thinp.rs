use ::libc;
extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    static _th_chlevel_tbl: [libc::c_int; 0];
    static mut TACchtype_: [libc::c_short; 256];
    static mut TACio_op_: [[libc::c_short; 17]; 17];
}
pub type thchar_t = libc::c_uchar;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thcell_t {
    pub base: thchar_t,
    pub hilo: thchar_t,
    pub top: thchar_t,
}
pub type thstrict_t = libc::c_uint;
pub const ISC_STRICT: thstrict_t = 2;
pub const ISC_BASICCHECK: thstrict_t = 1;
pub const ISC_PASSTHROUGH: thstrict_t = 0;
pub const SR: WTTOp = 5;
pub type WTTOp = libc::c_uint;
pub const RJ: WTTOp = 4;
pub const AC: WTTOp = 3;
pub const XC: WTTOp = 2;
pub const CP: WTTOp = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thinpconv_t {
    pub conv: [thchar_t; 4],
    pub offset: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct correction_t {
    pub c1: thchar_t,
    pub c2: thchar_t,
    pub to: [thchar_t; 3],
}
pub const TIS_RU: libc::c_int = 0xc4 as libc::c_int;
pub const TIS_LU: libc::c_int = 0xc6 as libc::c_int;
pub const TIS_SARA_AA: libc::c_int = 0xd2 as libc::c_int;
pub const TIS_SARA_AM: libc::c_int = 0xd3 as libc::c_int;
pub const TIS_LAKKHANGYAO: libc::c_int = 0xe5 as libc::c_int;
pub const TIS_NIKHAHIT: libc::c_int = 0xed as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn th_isaccept(
    mut c1: thchar_t,
    mut c2: thchar_t,
    mut s: thstrict_t,
) -> libc::c_int {
    match s as libc::c_uint {
        0 => return 1 as libc::c_int,
        1 => {
            return (TACio_op_[TACchtype_[c1 as usize] as usize][TACchtype_[c2 as usize] as usize]
                as libc::c_int
                != RJ as libc::c_int) as libc::c_int;
        }
        2 => {
            let mut op: WTTOp = TACio_op_[TACchtype_[c1 as usize] as usize]
                [TACchtype_[c2 as usize] as usize] as WTTOp;
            return (op as libc::c_uint != RJ as libc::c_int as libc::c_uint
                && op as libc::c_uint != SR as libc::c_int as libc::c_uint)
                as libc::c_int;
        }
        _ => return 0 as libc::c_int,
    };
}
static mut corrections: [correction_t; 4] = [
    {
        let mut init = correction_t {
            c1: TIS_RU as thchar_t,
            c2: TIS_SARA_AA as thchar_t,
            to: [
                TIS_RU as thchar_t,
                TIS_LAKKHANGYAO as thchar_t,
                0 as libc::c_int as thchar_t,
            ],
        };
        init
    },
    {
        let mut init = correction_t {
            c1: TIS_LU as thchar_t,
            c2: TIS_SARA_AA as thchar_t,
            to: [
                TIS_LU as thchar_t,
                TIS_LAKKHANGYAO as thchar_t,
                0 as libc::c_int as thchar_t,
            ],
        };
        init
    },
    {
        let mut init = correction_t {
            c1: TIS_NIKHAHIT as thchar_t,
            c2: TIS_SARA_AA as thchar_t,
            to: [
                TIS_SARA_AM as thchar_t,
                0 as libc::c_int as thchar_t,
                0 as libc::c_int as thchar_t,
            ],
        };
        init
    },
    {
        let mut init = correction_t {
            c1: 0 as libc::c_int as thchar_t,
            c2: 0 as libc::c_int as thchar_t,
            to: [
                0 as libc::c_int as thchar_t,
                0 as libc::c_int as thchar_t,
                0 as libc::c_int as thchar_t,
            ],
        };
        init
    },
];
unsafe extern "C" fn correct_(
    mut c_1: thchar_t,
    mut c: thchar_t,
    mut conv: *mut thchar_t,
) -> libc::c_int {
    let mut p: *const correction_t = 0 as *const correction_t;
    p = corrections.as_ptr();
    while (*p).c1 != 0 {
        if c_1 as libc::c_int == (*p).c1 as libc::c_int
            && c as libc::c_int == (*p).c2 as libc::c_int
        {
            strcpy(
                conv as *mut libc::c_char,
                ((*p).to).as_ptr() as *const libc::c_char,
            );
            return 1 as libc::c_int;
        }
        p = p.offset(1);
        p;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_validate(
    mut context: thcell_t,
    mut c: thchar_t,
    mut conv: *mut thinpconv_t,
) -> libc::c_int {
    return th_validate_leveled(context, c, conv, ISC_STRICT);
}
#[no_mangle]
pub unsafe extern "C" fn th_validate_leveled(
    mut context: thcell_t,
    mut c: thchar_t,
    mut conv: *mut thinpconv_t,
    mut s: thstrict_t,
) -> libc::c_int {
    let mut prev_c: thchar_t = (if context.top as libc::c_int != 0 {
        context.top as libc::c_int
    } else if context.hilo as libc::c_int != 0 {
        context.hilo as libc::c_int
    } else {
        context.base as libc::c_int
    }) as thchar_t;
    let mut ret: libc::c_int = 0;
    if context.hilo as libc::c_int == TIS_SARA_AM {
        prev_c = TIS_SARA_AM as thchar_t;
    }
    ret = correct_(prev_c, c, ((*conv).conv).as_mut_ptr());
    if ret != 0 {
        (*conv).offset = -(1 as libc::c_int);
        return 1 as libc::c_int;
    }
    if th_isaccept(prev_c, c, s) != 0 {
        (*conv).conv[0 as libc::c_int as usize] = c;
        (*conv).conv[1 as libc::c_int as usize] = 0 as libc::c_int as thchar_t;
        (*conv).offset = 0 as libc::c_int;
        return 1 as libc::c_int;
    }
    let mut current_block_52: u64;
    match *_th_chlevel_tbl.as_ptr().offset(c as isize) {
        3 | 2 => {
            if context.hilo as libc::c_int != 0 && th_isaccept(context.hilo, c, s) != 0 {
                (*conv).offset = 0 as libc::c_int;
                (*conv).conv[0 as libc::c_int as usize] = c;
                (*conv).conv[1 as libc::c_int as usize] = 0 as libc::c_int as thchar_t;
                if context.top != 0 {
                    (*conv).offset -= 1;
                    (*conv).offset;
                }
                return 1 as libc::c_int;
            }
            if th_isaccept(context.base, c, s) != 0
                && (context.hilo as libc::c_int != TIS_SARA_AM
                    || th_isaccept(c, TIS_SARA_AM as thchar_t, s) != 0)
            {
                let mut i: libc::c_int = 0 as libc::c_int;
                (*conv).offset = 0 as libc::c_int;
                let fresh0 = i;
                i = i + 1;
                (*conv).conv[fresh0 as usize] = c;
                if context.hilo != 0 {
                    (*conv).offset -= 1;
                    (*conv).offset;
                    if context.hilo as libc::c_int == TIS_SARA_AM {
                        let fresh1 = i;
                        i = i + 1;
                        (*conv).conv[fresh1 as usize] = TIS_SARA_AM as thchar_t;
                    }
                }
                if context.top != 0 {
                    (*conv).offset -= 1;
                    (*conv).offset;
                }
                (*conv).conv[i as usize] = 0 as libc::c_int as thchar_t;
                return 1 as libc::c_int;
            }
            if *_th_chlevel_tbl.as_ptr().offset(c as isize) == 2 as libc::c_int {
                current_block_52 = 10758786907990354186;
            } else {
                current_block_52 = 13862081255046397807;
            }
        }
        -1 | 1 => {
            current_block_52 = 13862081255046397807;
        }
        _ => {
            current_block_52 = 10758786907990354186;
        }
    }
    match current_block_52 {
        13862081255046397807 => {
            if th_isaccept(context.base, c, s) != 0 {
                let mut i_0: libc::c_int = 0 as libc::c_int;
                (*conv).offset = 0 as libc::c_int;
                let fresh2 = i_0;
                i_0 = i_0 + 1;
                (*conv).conv[fresh2 as usize] = c;
                if context.hilo != 0 {
                    (*conv).offset -= 1;
                    (*conv).offset;
                }
                if context.top != 0 {
                    (*conv).offset -= 1;
                    (*conv).offset;
                    if th_isaccept(c, context.top, s) != 0 {
                        let fresh3 = i_0;
                        i_0 = i_0 + 1;
                        (*conv).conv[fresh3 as usize] = context.top;
                    }
                }
                (*conv).conv[i_0 as usize] = 0 as libc::c_int as thchar_t;
                return 1 as libc::c_int;
            }
        }
        _ => {}
    }
    return 0 as libc::c_int;
}
