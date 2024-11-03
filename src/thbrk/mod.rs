mod common;
mod ctype;
mod maximal;

use crate::thbrk::common::{brk_load_default_dict, ThTrie};
use crate::thbrk::ctype::{brk_class, brk_op, BrkClass, BrkOp};
use crate::thbrk::maximal::{brk_maximal_do, BrkEnv};
use crate::thctype::thchar_t;
use ::libc;
use std::ffi::{CStr, OsStr};
use std::os::unix::ffi::OsStrExt;
use std::ptr::NonNull;
use std::sync::LazyLock;
use std::{ptr, slice};

extern "C" {
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::size_t;
    fn malloc(_: libc::size_t) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
}

pub static SHARED: LazyLock<ThBrk> = LazyLock::new(|| ThBrk::new_default());

#[derive(Clone)]
pub struct ThBrk {
    dict_trie: Option<ThTrie>,
}

impl ThBrk {
    pub fn new(dict: Option<ThTrie>) -> Self {
        ThBrk { dict_trie: dict }
    }

    pub fn new_default() -> ThBrk {
        ThBrk::new(brk_load_default_dict().ok())
    }

    /// Find word break positions in TIS-620 string
    pub fn find_breaks(&self, input: &[thchar_t]) -> Vec<i32> {
        self.find_breaks_limited(input, input.len()).0
    }

    /// Find word break positions in TIS-620 string, with limit
    pub fn find_breaks_limited(&self, s: &[thchar_t], limit: usize) -> (Vec<i32>, usize) {
        const MAX_ACRONYM_FRAG_LEN: usize = 3;

        let mut pos = vec![0; limit];

        if s.len() == 0 {
            return (pos, 0);
        }

        let mut prev_class = brk_class(s[0]);
        let mut effective_class = brk_class(s[0]);
        let mut chunk_ind = 0;
        let mut acronym_end = 0;
        let mut p = 0;
        let mut cur_pos = 0;

        let env = BrkEnv::new(self);
        loop {
            p += 1;
            if p >= s.len() || cur_pos >= limit {
                break;
            }

            let mut new_class = brk_class(s[p]);

            if prev_class == BrkClass::Thai || prev_class == BrkClass::Alpha {
                // handle acronyms
                if s[p] == '.' as u8 && p - acronym_end <= MAX_ACRONYM_FRAG_LEN {
                    // the period after Thai/Alpha is part of the acronym
                    new_class = prev_class;
                    acronym_end = p + 1;
                } else if acronym_end > chunk_ind {
                    // an acronym was marked
                    if new_class != prev_class || p - acronym_end > MAX_ACRONYM_FRAG_LEN {
                        // end of Thai/Alpha chunk or entered non-acronym word,
                        // jump back to the acronym end
                        effective_class = brk_class('.' as thchar_t);
                        prev_class = effective_class;
                        chunk_ind = acronym_end;
                        p = acronym_end;
                        new_class = brk_class(s[p]);
                    }
                }

                // break chunk if leaving Thai chunk
                if prev_class == BrkClass::Thai && new_class != BrkClass::Thai && p > chunk_ind {
                    let chunk = &s[chunk_ind..p];
                    let n_brk = brk_maximal_do(
                        chunk.as_ptr(),
                        chunk.len() as i32,
                        (&mut pos[cur_pos..]) as *mut [i32] as *mut i32,
                        limit - cur_pos,
                        &env,
                    );

                    for i in 0..(n_brk as usize) {
                        pos[cur_pos + i] += chunk_ind as i32;
                    }
                    cur_pos += n_brk as usize;

                    // remove last break if at string end
                    // note that even if it's allowed, the table-lookup
                    // operation below will take care of it anyway
                    if cur_pos > 0 && pos[cur_pos - 1] == p as i32 {
                        cur_pos -= 1;
                    }

                    if cur_pos >= limit {
                        break;
                    }
                }
            }

            // reset chunk on switching
            if new_class != prev_class {
                acronym_end = p;
                chunk_ind = p;
            }

            let op = brk_op(effective_class, new_class);

            match op {
                BrkOp::Allowed => {
                    if !(s[p] == '\n' as thchar_t && s[p - 1] == '\r' as thchar_t) {
                        pos[cur_pos] = p as i32;
                        cur_pos += 1;
                    }
                }
                BrkOp::Indirect => {
                    if prev_class == BrkClass::Space {
                        pos[cur_pos] = p as i32;
                        cur_pos += 1;
                    }
                }
                _ => {}
            }

            prev_class = new_class;
            if op == BrkOp::Allowed || new_class != BrkClass::Space {
                effective_class = new_class;
            }
        }

        // break last Thai non-acronym chunk
        if prev_class == BrkClass::Thai && acronym_end <= chunk_ind && cur_pos < limit {
            let chunk = &s[chunk_ind..p];
            let n_brk = brk_maximal_do(
                chunk.as_ptr(),
                chunk.len() as i32,
                (&mut pos[cur_pos..]) as *mut [i32] as *mut i32,
                limit - cur_pos,
                &env,
            );

            for i in 0..(n_brk as usize) {
                pos[cur_pos + i] += chunk_ind as i32;
            }
            cur_pos += n_brk as usize;

            // remove last break if at string end
            if cur_pos > 0 && pos[cur_pos - 1] == p as i32 {
                cur_pos -= 1;
            }
        }

        (pos, cur_pos)
    }
}

impl Default for ThBrk {
    fn default() -> Self {
        ThBrk::new(brk_load_default_dict().ok())
    }
}

#[no_mangle]
#[deprecated(note = "Use ThBrk::new() or ThBrk::new_default()")]
pub extern "C" fn th_brk_new(dictpath: *const libc::c_char) -> *mut ThBrk {
    // XXX: In the C version, a null ThBrk is equivalent to our ThBrk::default()
    // Hence if data loading fail the result differs
    println!("th_brk_new rust");

    match unsafe { dictpath.as_ref() } {
        Some(path) => {
            let path_str = unsafe { CStr::from_ptr(path) };
            let path_os = OsStr::from_bytes(path_str.to_bytes());
            let trie = ThTrie::from_file(path_os);
            Box::into_raw(Box::new(ThBrk::new(trie.ok())))
        }
        None => Box::into_raw(Box::new(ThBrk::new_default())),
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
    n_brk_pos = th_brk_find_breaks(
        brk.as_ref(),
        in_0,
        NonNull::new_unchecked(brk_pos),
        n_brk_pos,
    ) as libc::size_t;
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
#[deprecated(note = "Use (pos, out) = brk.find_breaks_limited(s, pos_sz). Check for null brk!")]
pub extern "C" fn th_brk_find_breaks(
    brk: Option<&ThBrk>,
    s: *const thchar_t,
    pos: NonNull<i32>,
    pos_sz: usize,
) -> i32 {
    let s = unsafe { CStr::from_ptr(s as *const libc::c_char) };
    let pos = unsafe { slice::from_raw_parts_mut(pos.as_ptr(), pos_sz) };

    let (break_pos, count) = unsafe {
        brk.unwrap_or_else(|| &*SHARED)
            .find_breaks_limited(s.to_bytes(), pos_sz)
    };

    pos[..break_pos.len()].copy_from_slice(&break_pos);

    count as i32
}

#[no_mangle]
pub unsafe extern "C" fn th_brk_line(
    mut in_0: *const thchar_t,
    mut out: *mut thchar_t,
    mut out_sz: libc::size_t,
    mut delim: *const libc::c_char,
) -> libc::c_int {
    th_brk_insert_breaks(ptr::null_mut(), in_0, out, out_sz, delim)
}

#[no_mangle]
#[deprecated(note = "Use (pos, out) = brk.find_breaks_limited(s, pos_sz). Check for null brk!")]
pub extern "C" fn th_brk(s: *const thchar_t, mut pos: NonNull<i32>, pos_sz: usize) -> i32 {
    let s = unsafe { CStr::from_ptr(s as *const libc::c_char) };
    let pos = unsafe { slice::from_raw_parts_mut(pos.as_ptr(), pos_sz) };

    let (break_pos, count) = unsafe { SHARED.find_breaks_limited(s.to_bytes(), pos_sz) };

    pos[..break_pos.len()].copy_from_slice(&break_pos);

    count as i32
}
