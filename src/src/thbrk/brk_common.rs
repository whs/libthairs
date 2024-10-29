use ::libc;
use datrie::{CTrieData, ROTrie};
use std::path::Path;
use std::{env, io, ptr};

const DICT_DIR: &'static str = "share/libthai";
const DICT_NAME: &'static str = "thbrk";

pub type ThTrie = ROTrie<Option<CTrieData>>;

extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static _th_ctype_tbl: [libc::c_ushort; 0];
}
pub type thchar_t = u8;
pub type C2RustUnnamed = libc::c_uint;
pub const _th_ISpunct: C2RustUnnamed = 1024;
pub const _th_ISdigit: C2RustUnnamed = 512;
pub const _th_ISdiac: C2RustUnnamed = 256;
pub const _th_IStone: C2RustUnnamed = 128;
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
pub const _th_IScons: C2RustUnnamed = 2;
pub const _th_IStis: C2RustUnnamed = 1;
pub const TIS_KO_KAI: libc::c_int = 0xa1 as libc::c_int;
pub const TIS_RU: libc::c_int = 0xc4 as libc::c_int;
pub const TIS_LU: libc::c_int = 0xc6 as libc::c_int;
pub const TIS_WO_WAEN: libc::c_int = 0xc7 as libc::c_int;
pub const TIS_O_ANG: libc::c_int = 0xcd as libc::c_int;
pub const TIS_SARA_A: libc::c_int = 0xd0 as libc::c_int;
pub const TIS_MAI_HAN_AKAT: libc::c_int = 0xd1 as libc::c_int;
pub const TIS_SARA_AA: libc::c_int = 0xd2 as libc::c_int;
pub const TIS_SARA_UEE: libc::c_int = 0xd7 as libc::c_int;
pub const TIS_SARA_E: libc::c_int = 0xe0 as libc::c_int;
pub const TIS_SARA_AE: libc::c_int = 0xe1 as libc::c_int;
pub const TIS_MAITAIKHU: libc::c_int = 0xe7 as libc::c_int;
pub const TIS_THANTHAKHAT: libc::c_int = 0xec as libc::c_int;

pub fn brk_load_default_dict_rs() -> io::Result<ThTrie> {
    let env = env::var("LIBTHAI_DICTDIR");
    let dict_file = match env {
        Ok(v) => Path::new(&v).join(format!("{}.tri", DICT_NAME)),
        Err(_) => Path::new(DICT_DIR).join(format!("{}.tri", DICT_NAME)),
    };

    ThTrie::from_file(dict_file)
}

#[no_mangle]
#[deprecated(note = "Use brk_load_default_dict_rs")]
pub extern "C" fn brk_load_default_dict() -> *mut ThTrie {
    match brk_load_default_dict_rs() {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(_) => ptr::null_mut(),
    }
}

#[no_mangle]
pub unsafe extern "C" fn brk_brkpos_hints(
    mut str: *const thchar_t,
    mut len: libc::c_int,
    mut hints: *mut libc::c_char,
) {
    let mut i: libc::c_int = 0;
    if len < 0 as libc::c_int {
        len = strlen(str as *const libc::c_char) as libc::c_int;
    }
    memset(
        hints as *mut libc::c_void,
        0 as libc::c_int,
        len as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < len {
        if *_th_ctype_tbl
            .as_ptr()
            .offset(*str.offset(i as isize) as isize) as libc::c_int
            & _th_IScons as libc::c_int
            != 0
        {
            if (i + 1 as libc::c_int) < len
                && *str.offset((i + 1 as libc::c_int) as isize) as libc::c_int == TIS_THANTHAKHAT
            {
                i += 2 as libc::c_int;
            } else if (i + 2 as libc::c_int) < len
                && *str.offset((i + 2 as libc::c_int) as isize) as libc::c_int == TIS_THANTHAKHAT
            {
                i += 3 as libc::c_int;
            } else if (i + 2 as libc::c_int) < len
                && *str.offset(i as isize) as libc::c_int != TIS_KO_KAI
                && *str.offset((i + 1 as libc::c_int) as isize) as libc::c_int == TIS_MAITAIKHU
                && (*str.offset((i + 2 as libc::c_int) as isize) as libc::c_int == TIS_O_ANG
                    || *str.offset((i + 2 as libc::c_int) as isize) as libc::c_int == TIS_WO_WAEN)
            {
                *hints.offset(i as isize) = 1 as libc::c_int as libc::c_char;
                i += 4 as libc::c_int;
            } else if i > 0 as libc::c_int
                && (*str.offset((i - 1 as libc::c_int) as isize) as libc::c_int == TIS_MAI_HAN_AKAT
                    || *str.offset((i - 1 as libc::c_int) as isize) as libc::c_int == TIS_SARA_UEE)
                || i > 1 as libc::c_int
                    && *_th_ctype_tbl
                        .as_ptr()
                        .offset(*str.offset((i - 1 as libc::c_int) as isize) as isize)
                        as libc::c_int
                        & _th_IStone as libc::c_int
                        != 0
                    && (*str.offset((i - 2 as libc::c_int) as isize) as libc::c_int
                        == TIS_MAI_HAN_AKAT
                        || *str.offset((i - 2 as libc::c_int) as isize) as libc::c_int
                            == TIS_SARA_UEE)
            {
                i += 1;
                i;
            } else {
                let fresh0 = i;
                i = i + 1;
                *hints.offset(fresh0 as isize) = 1 as libc::c_int as libc::c_char;
            }
        } else if *str.offset(i as isize) as libc::c_int == TIS_SARA_E
            || *str.offset(i as isize) as libc::c_int == TIS_SARA_AE
        {
            *hints.offset(i as isize) = 1 as libc::c_int as libc::c_char;
            i += 2 as libc::c_int;
            if i >= len {
                break;
            }
            if *str.offset(i as isize) as libc::c_int == TIS_MAITAIKHU {
                i += 2 as libc::c_int;
            } else if *_th_ctype_tbl
                .as_ptr()
                .offset(*str.offset(i as isize) as isize) as libc::c_int
                & _th_VClassMsk as libc::c_int
                == _th_VCupvowel as libc::c_int
            {
                i += 1;
                i;
                if i < len
                    && *_th_ctype_tbl
                        .as_ptr()
                        .offset(*str.offset(i as isize) as isize)
                        as libc::c_int
                        & _th_IStone as libc::c_int
                        != 0
                {
                    i += 1;
                    i;
                }
                i += 1;
                i;
            } else if (i + 2 as libc::c_int) < len
                && (*str.offset((i + 1 as libc::c_int) as isize) as libc::c_int == TIS_SARA_AA
                    && *str.offset((i + 2 as libc::c_int) as isize) as libc::c_int == TIS_SARA_A
                    || *str.offset(i as isize) as libc::c_int != TIS_KO_KAI
                        && *str.offset((i + 1 as libc::c_int) as isize) as libc::c_int
                            == TIS_MAITAIKHU
                        && *str.offset((i + 2 as libc::c_int) as isize) as libc::c_int != TIS_O_ANG
                        && *str.offset((i + 2 as libc::c_int) as isize) as libc::c_int
                            != TIS_WO_WAEN)
            {
                i += 3 as libc::c_int;
            }
        } else if *_th_ctype_tbl
            .as_ptr()
            .offset(*str.offset(i as isize) as isize) as libc::c_int
            & _th_VClassMsk as libc::c_int
            == _th_VCldvowel as libc::c_int
        {
            *hints.offset(i as isize) = 1 as libc::c_int as libc::c_char;
            i += 2 as libc::c_int;
        } else if *str.offset(i as isize) as libc::c_int == TIS_RU
            || *str.offset(i as isize) as libc::c_int == TIS_LU
        {
            let fresh1 = i;
            i = i + 1;
            *hints.offset(fresh1 as isize) = 1 as libc::c_int as libc::c_char;
        } else {
            i += 1;
            i;
        }
    }
}
