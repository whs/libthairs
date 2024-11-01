mod common;
mod ctype;
mod maximal;

use crate::thbrk::common::{brk_load_default_dict_rs, ThTrie};
use crate::thbrk::ctype::{brk_class, brk_op, BrkClass, BrkOp};
use crate::thctype::thchar_t;
use ::libc;
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::ptr::NonNull;
use std::sync::LazyLock;
use std::{io, ptr};

extern "C" {
    pub type BrkEnv;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::size_t;
    fn malloc(_: libc::size_t) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn brk_env_new(brk: *mut ThBrk) -> *mut BrkEnv;
    fn brk_env_free(env: *mut BrkEnv);
    fn brk_maximal_do(
        s: *const thchar_t,
        len: libc::c_int,
        pos: *mut libc::c_int,
        n: libc::size_t,
        env: *mut BrkEnv,
    ) -> libc::c_int;
}

#[repr(C)]
pub struct ThBrk {
    // TODO: Remove Box when the Rust port is complete
    dict_trie: Box<ThTrie>,
}

pub const MAX_ACRONYM_FRAG_LEN: libc::c_int = 3 as libc::c_int;

impl ThBrk {
    pub fn new(dict: ThTrie) -> Self {
        ThBrk {
            dict_trie: Box::new(dict),
        }
    }

    pub fn new_default() -> io::Result<ThBrk> {
        Ok(ThBrk::new(brk_load_default_dict_rs()?))
    }

    /// Find word break positions in TIS-620 string
    pub fn find_breaks(&self, input: &[u8]) -> Vec<usize> {
        self.find_breaks_limited(input, input.len())
    }

    /// Find word break positions in TIS-620 string, with limit
    pub fn find_breaks_limited(&self, input: &[u8], limit: usize) -> Vec<usize> {
        if input.len() == 0 {
            return Vec::default();
        }

        todo!()
    }
}

impl Default for ThBrk {
    fn default() -> Self {
        ThBrk::new(brk_load_default_dict_rs().expect("unable to load default dict"))
    }
}

pub static SHARED: LazyLock<Option<ThBrk>> = LazyLock::new(|| ThBrk::new_default().ok());

#[no_mangle]
#[deprecated(note = "Use ThBrk::new() or ThBrk::new_default()")]
pub extern "C" fn th_brk_new(dictpath: *const libc::c_char) -> *mut ThBrk {
    println!("th_brk_new rust");

    match unsafe { dictpath.as_ref() } {
        Some(path) => {
            let path_str = unsafe { CStr::from_ptr(path) };
            let path_os = OsStr::from_bytes(path_str.to_bytes());
            match ThTrie::from_file(path_os) {
                Ok(trie) => Box::into_raw(Box::new(ThBrk::new(trie))),
                Err(_) => ptr::null_mut(),
            }
        }
        None => match ThBrk::new_default() {
            Ok(v) => Box::into_raw(Box::new(v)),
            Err(_) => ptr::null_mut(),
        },
    }
}

#[no_mangle]
pub unsafe extern "C" fn th_brk_delete(mut brk: NonNull<ThBrk>) {
    drop(Box::from_raw(brk.as_mut()))
}

#[no_mangle]
pub unsafe extern "C" fn th_brk_insert_breaks(
    mut brk: *mut ThBrk,
    mut in_0: *const thchar_t,
    mut out: *mut thchar_t,
    mut out_sz: libc::size_t,
    mut delim: *const libc::c_char,
) -> libc::c_int {
    let mut brk_pos: *mut libc::c_int = 0 as *mut libc::c_int;
    let mut n_brk_pos: libc::size_t = 0;
    let mut i: libc::size_t = 0;
    let mut j: libc::size_t = 0;
    let mut delim_len: libc::c_int = 0;
    let mut p_out: *mut thchar_t = 0 as *mut thchar_t;
    n_brk_pos = strlen(in_0 as *const libc::c_char);
    if n_brk_pos
        > (18446744073709551615 as libc::size_t)
            .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::size_t)
    {
        return 0 as libc::c_int;
    }
    brk_pos = malloc(n_brk_pos.wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::size_t))
        as *mut libc::c_int;
    if brk_pos.is_null() {
        return 0 as libc::c_int;
    }
    n_brk_pos = th_brk_find_breaks(brk, in_0, brk_pos, n_brk_pos) as libc::size_t;
    delim_len = strlen(delim) as libc::c_int;
    j = 0 as libc::size_t;
    i = j;
    p_out = out;
    while out_sz > 1 as libc::c_int as libc::size_t && i < n_brk_pos {
        while out_sz > 1 as libc::c_int as libc::size_t
            && j < *brk_pos.offset(i as isize) as libc::size_t
        {
            let fresh0 = j;
            j = j.wrapping_add(1);
            let fresh1 = p_out;
            p_out = p_out.offset(1);
            *fresh1 = *in_0.offset(fresh0 as isize);
            out_sz = out_sz.wrapping_sub(1);
            out_sz;
        }
        if out_sz > (delim_len + 1 as libc::c_int) as libc::size_t {
            strcpy(p_out as *mut libc::c_char, delim);
            p_out = p_out.offset(delim_len as isize);
            out_sz = out_sz.wrapping_sub(delim_len as libc::size_t);
        }
        i = i.wrapping_add(1);
        i;
    }
    while out_sz > 1 as libc::c_int as libc::size_t && *in_0.offset(j as isize) as libc::c_int != 0
    {
        let fresh2 = j;
        j = j.wrapping_add(1);
        let fresh3 = p_out;
        p_out = p_out.offset(1);
        *fresh3 = *in_0.offset(fresh2 as isize);
        out_sz = out_sz.wrapping_sub(1);
        out_sz;
    }
    *p_out = '\0' as i32 as thchar_t;
    free(brk_pos as *mut libc::c_void);
    return p_out.offset_from(out) as libc::c_long as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_brk_find_breaks(
    mut brk: *mut ThBrk,
    mut s: *const thchar_t,
    mut pos: *mut libc::c_int,
    mut pos_sz: libc::size_t,
) -> libc::c_int {
    let mut env: *mut BrkEnv = 0 as *mut BrkEnv;
    let mut prev_class = BrkClass::Thai;
    let mut effective_class = BrkClass::Thai;
    let mut chunk: *const thchar_t = 0 as *const thchar_t;
    let mut acronym_end: *const thchar_t = 0 as *const thchar_t;
    let mut p: *const thchar_t = 0 as *const thchar_t;
    let mut cur_pos: libc::c_int = 0;
    if *s == 0 {
        return 0 as libc::c_int;
    }
    acronym_end = s;
    chunk = acronym_end;
    p = chunk;
    effective_class = brk_class(*p);
    prev_class = effective_class;
    cur_pos = 0 as libc::c_int;
    env = brk_env_new(if !brk.is_null() {
        brk
    } else {
        brk_get_shared_brk().cast_mut()
    });
    loop {
        p = p.offset(1);
        if !(*p as libc::c_int != 0 && (cur_pos as libc::size_t) < pos_sz) {
            break;
        }
        let mut new_class = BrkClass::Thai;
        let mut op = BrkOp::Prohibited;
        new_class = brk_class(*p);
        if BrkClass::Thai == prev_class || BrkClass::Alpha == prev_class {
            if '.' as i32 == *p as libc::c_int
                && p.offset_from(acronym_end) as libc::c_long
                    <= MAX_ACRONYM_FRAG_LEN as libc::c_long
            {
                new_class = prev_class;
                acronym_end = p.offset(1 as libc::c_int as isize);
            } else if acronym_end > chunk {
                if new_class as libc::c_uint != prev_class as libc::c_uint
                    || p.offset_from(acronym_end) as libc::c_long
                        > MAX_ACRONYM_FRAG_LEN as libc::c_long
                {
                    effective_class = brk_class('.' as i32 as thchar_t);
                    prev_class = effective_class;
                    chunk = acronym_end;
                    p = chunk;
                    new_class = brk_class(*p);
                }
            }
            if BrkClass::Thai == prev_class && BrkClass::Thai != new_class && p > chunk {
                let mut n_brk: libc::c_int = 0;
                let mut i: libc::c_int = 0;
                n_brk = brk_maximal_do(
                    chunk,
                    p.offset_from(chunk) as libc::c_long as libc::c_int,
                    pos.offset(cur_pos as isize),
                    pos_sz.wrapping_sub(cur_pos as libc::size_t),
                    env,
                );
                i = 0 as libc::c_int;
                while i < n_brk {
                    let ref mut fresh4 = *pos.offset((cur_pos + i) as isize);
                    *fresh4 = (*fresh4 as libc::c_long + chunk.offset_from(s) as libc::c_long)
                        as libc::c_int;
                    i += 1;
                    i;
                }
                cur_pos += n_brk;
                if cur_pos > 0 as libc::c_int
                    && *pos.offset((cur_pos - 1 as libc::c_int) as isize) as libc::c_long
                        == p.offset_from(s) as libc::c_long
                {
                    cur_pos -= 1;
                    cur_pos;
                }
                if cur_pos as libc::size_t >= pos_sz {
                    break;
                }
            }
        }
        if new_class as libc::c_uint != prev_class as libc::c_uint {
            acronym_end = p;
            chunk = acronym_end;
        }
        op = brk_op(effective_class, new_class);
        match op as libc::c_uint {
            1 => {
                if !('\n' as i32 == *p as libc::c_int
                    && '\r' as i32 == *p.offset(-(1 as libc::c_int as isize)) as libc::c_int)
                {
                    let fresh5 = cur_pos;
                    cur_pos = cur_pos + 1;
                    *pos.offset(fresh5 as isize) = p.offset_from(s) as libc::c_long as libc::c_int;
                }
            }
            2 => {
                if BrkClass::Space == prev_class {
                    let fresh6 = cur_pos;
                    cur_pos = cur_pos + 1;
                    *pos.offset(fresh6 as isize) = p.offset_from(s) as libc::c_long as libc::c_int;
                }
            }
            _ => {}
        }
        prev_class = new_class;
        if BrkOp::Allowed == op || BrkClass::Space != new_class {
            effective_class = new_class;
        }
    }
    if BrkClass::Thai == prev_class && acronym_end <= chunk && (cur_pos as libc::size_t) < pos_sz {
        let mut n_brk_0: libc::c_int = 0;
        let mut i_0: libc::c_int = 0;
        n_brk_0 = brk_maximal_do(
            chunk,
            p.offset_from(chunk) as libc::c_long as libc::c_int,
            pos.offset(cur_pos as isize),
            pos_sz.wrapping_sub(cur_pos as libc::size_t),
            env,
        );
        i_0 = 0 as libc::c_int;
        while i_0 < n_brk_0 {
            let ref mut fresh7 = *pos.offset((cur_pos + i_0) as isize);
            *fresh7 =
                (*fresh7 as libc::c_long + chunk.offset_from(s) as libc::c_long) as libc::c_int;
            i_0 += 1;
            i_0;
        }
        cur_pos += n_brk_0;
        if cur_pos > 0 as libc::c_int
            && *pos.offset((cur_pos - 1 as libc::c_int) as isize) as libc::c_long
                == p.offset_from(s) as libc::c_long
        {
            cur_pos -= 1;
            cur_pos;
        }
    }
    brk_env_free(env);
    return cur_pos;
}
#[no_mangle]
pub unsafe extern "C" fn th_brk_line(
    mut in_0: *const thchar_t,
    mut out: *mut thchar_t,
    mut out_sz: libc::size_t,
    mut delim: *const libc::c_char,
) -> libc::c_int {
    return th_brk_insert_breaks(ptr::null_mut(), in_0, out, out_sz, delim);
}
#[no_mangle]
pub unsafe extern "C" fn th_brk(
    mut s: *const thchar_t,
    mut pos: *mut libc::c_int,
    mut pos_sz: libc::size_t,
) -> libc::c_int {
    return th_brk_find_breaks(ptr::null_mut(), s, pos, pos_sz);
}

/// Get the global, shared instance of ThBrk
///
/// The Rust version of this is thread safe
#[no_mangle]
#[deprecated(note = "Use SHARED")]
pub(crate) extern "C" fn brk_get_shared_brk() -> *const ThBrk {
    match &*SHARED {
        Some(brk) => brk,
        None => ptr::null(),
    }
}

/// Does nothing in the Rust version
#[no_mangle]
#[deprecated(note = "Use SHARED")]
pub(crate) unsafe extern "C" fn brk_free_shared_brk() {}
