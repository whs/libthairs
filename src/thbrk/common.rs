use crate::thctype::{thchar_t, ThaiCharacter};
use crate::tis::{
    TIS_KO_KAI, TIS_LU, TIS_MAITAIKHU, TIS_MAI_HAN_AKAT, TIS_O_ANG, TIS_RU, TIS_SARA_A,
    TIS_SARA_AA, TIS_SARA_AE, TIS_SARA_E, TIS_SARA_UEE, TIS_THANTHAKHAT, TIS_WO_WAEN,
};
use ::libc;
use datrie::{CTrieData, ROTrie};
use std::ffi::CStr;
use std::path::Path;
use std::ptr::NonNull;
use std::{env, io, ptr, slice};

const DICT_DIR: &'static str = "share/libthai";
const DICT_NAME: &'static str = "thbrk";

pub type ThTrie = ROTrie<Option<CTrieData>>;

extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static _th_ctype_tbl: [libc::c_ushort; 0];
}

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
pub(crate) extern "C" fn brk_load_default_dict() -> *mut ThTrie {
    match brk_load_default_dict_rs() {
        Ok(v) => Box::into_raw(Box::new(v)),
        Err(_) => ptr::null_mut(),
    }
}

pub fn brk_brkpos_hints_rs(str: &[u8]) -> Vec<bool> {
    let mut hints = vec![false; str.len()];

    let mut i = 0;

    while i < str.len() {
        match str[i] {
            v if v.is_thcons() => {
                if str.get(i + 1) == Some(&TIS_THANTHAKHAT) {
                    i += 2; /* the cons + THANTHAKHAT */
                } else if str.get(i + 2) == Some(&TIS_THANTHAKHAT) {
                    i += 3; /* the cons + intermediate char + THANTHAKHAT */
                } else if i + 2 < str.len()
                    && str[i] != TIS_KO_KAI
                    && str[i + 1] == TIS_MAITAIKHU
                    && (str[i + 2] == TIS_O_ANG || str[i + 2] == TIS_WO_WAEN)
                {
                    hints[i] = true;
                    i += 4; /* the cons + MAITAIKHU + OANG/WOWAEN + cons */
                } else if (i > 0 && (str[i - 1] == TIS_MAI_HAN_AKAT || str[i - 1] == TIS_SARA_UEE))
                    || (i > 1
                        && str[i - 1].is_thtone()
                        && (str[i - 2] == TIS_MAI_HAN_AKAT || str[i - 2] == TIS_SARA_UEE))
                {
                    i += 1;
                } else {
                    hints[i] = true;
                    i += 1;
                }
            }
            TIS_SARA_E | TIS_SARA_AE => {
                hints[i] = true; /* sara e/ae */
                i += 2; /* sara e/ae + the supposedly cons */
                if i >= str.len() {
                    break;
                }

                if str[i] == TIS_MAITAIKHU {
                    i += 2; /* MAITAIKHU + the supposedly cons */
                } else if str[i].is_upvowel() {
                    i += 1; /* the upper vowel, as part of composite vowel */
                    if i < str.len() && str[i].is_thtone() {
                        i += 1;
                    }
                    i += 1; /* the supposedly cons */
                } else if i + 2 < str.len()
                    && ((str[i + 1] == TIS_SARA_AA && str[i + 2] == TIS_SARA_A)
                        || (str[i] != TIS_KO_KAI
                            && str[i + 1] == TIS_MAITAIKHU
                            && str[i + 2] != TIS_O_ANG
                            && str[i + 2] != TIS_WO_WAEN))
                {
                    i += 3; /* 2nd cons + SARA_AA + SARA_A, or
                             * 2nd cons + MAITAIKHU + final cons
                             */
                }
            }
            v if v.is_ldvowel() => {
                hints[i] = true; /* the ldvowel */
                i += 2; /* the ldvowel + the supposedly cons */
            }
            TIS_RU | TIS_LU => {
                hints[i] = true;
                i += 1;
            }
            _ => i += 1,
        }
    }

    hints
}

// TODO: Remove once nobody use it. Rename the _rs version to no _rs
#[no_mangle]
#[deprecated(note = "Use brk_brkpos_hints_rs")]
pub(crate) extern "C" fn brk_brkpos_hints(
    str: *const thchar_t,
    len: i32,
    mut hints: NonNull<libc::c_char>,
) {
    let str = unsafe {
        match len {
            v if v < 0 => CStr::from_ptr(str as *const i8).to_bytes(),
            _ => slice::from_raw_parts(str, len as usize),
        }
    };
    let hints = unsafe { slice::from_raw_parts_mut(hints.as_mut(), str.len()) };

    let out = brk_brkpos_hints_rs(str);
    for (index, hint) in out.iter().enumerate() {
        hints[index] = if *hint { 1 } else { 0 };
    }
}
