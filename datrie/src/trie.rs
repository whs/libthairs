use std::io::{BufReader, Read};
use crate::alpha_map::{
    alpha_map_char_to_trie, alpha_map_char_to_trie_str, alpha_map_clone, alpha_map_fread_bin,
    alpha_map_free, alpha_map_fwrite_bin, alpha_map_get_serialized_size, alpha_map_serialize_bin,
    alpha_map_trie_to_char, AlphaMap,
};
use crate::darray::{
    da_first_separate, da_fread, da_free, da_fwrite, da_get_base, da_get_check, da_get_root,
    da_get_serialized_size, da_insert_branch, da_new, da_next_separate, da_output_symbols,
    da_prune, da_prune_upto, da_serialize, da_set_base, da_walk, DArray,
};
use crate::tail::{
    tail_add_suffix, tail_delete, tail_fread, tail_free, tail_fwrite, tail_get_data,
    tail_get_serialized_size, tail_get_suffix, tail_new, tail_serialize, tail_set_data,
    tail_set_suffix, tail_walk_char, Tail,
};
use crate::trie_string::{
    trie_char_strlen, trie_string_free, trie_string_get_val, trie_string_length, trie_string_new,
    TrieString, TRIE_CHAR_TERM,
};
use crate::types::*;
use ::libc;
use std::{io, mem, ptr};
use std::ffi::{CStr, OsStr};
use std::fs::{File, OpenOptions};
use std::os::unix::prelude::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr::NonNull;
use crate::fileutils::wrap_cfile_nonnull;

extern "C" {
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
}
pub type size_t = libc::c_ulong;
pub type uint8 = libc::c_uchar;
pub type FILE = libc::FILE;

pub type TrieChar = u8;

pub type TrieData = i32;
pub const TRIE_DATA_ERROR: TrieData = -1;

#[repr(C)]
pub struct Trie {
    alpha_map: *mut AlphaMap,
    da: *mut DArray,
    tail: *mut Tail,
    is_dirty: Bool,
}

impl Trie {
    /// Create a new empty trie object based on the given `alpha_map` alphabet
    /// set. The trie contents can then be added and deleted with trie.store() and
    /// trie.delete() respectively.
    pub fn new(alpha_map: &AlphaMap) -> Self {
        Self {
            alpha_map: Box::into_raw(Box::new(alpha_map.clone())),
            da: Box::into_raw(Box::new(DArray::default())),
            tail: Box::into_raw(Box::new(Tail::default())),
            is_dirty: TRUE,
        }
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> io::Result<Self> {
        let mut fp = BufReader::new(File::open(path)?);
        Self::from_reader(&mut fp)
    }

    /// Create a new trie and initialize its contents by reading from a reader.
    /// This function guaranteed that only the trie has been read from the reader.
    /// This can be useful for embedding trie index as part of file data.
    pub fn from_reader<T: Read>(reader: &mut T) -> io::Result<Self> {
        let alpha_map = AlphaMap::read(reader)?;
        let da = DArray::read(reader)?;
        let tail = Tail::read(reader)?;

        Ok(Self {
            alpha_map: Box::into_raw(Box::new(alpha_map)),
            da: Box::into_raw(Box::new(da)),
            tail: Box::into_raw(Box::new(tail)),
            is_dirty: FALSE,
        })
    }

    /// Returns size that would be occupied by a trie if it was
    /// serialized into a binary blob or file.
    pub fn serialized_size(&self) -> usize {
        let am = unsafe { &*self.alpha_map };
        let da = unsafe { &*self.da };
        let tail = unsafe { &*self.tail };

        am.serialized_size() + da.serialized_size() + tail.serialized_size()
    }

    /// Check if the trie is dirty with some pending changes and needs saving
    /// to keep the file synchronized.
    pub fn is_dirty(&self) -> bool {
        self.is_dirty.into()
    }
}

impl Drop for Trie {
    fn drop(&mut self) {
        unsafe {
            drop(Box::from_raw(self.alpha_map));
            drop(Box::from_raw(self.da));
            drop(Box::from_raw(self.tail));
        }
    }
}

pub type TrieEnumFunc =
    Option<unsafe extern "C" fn(*const AlphaChar, TrieData, *mut libc::c_void) -> Bool>;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct TrieState {
    trie: *const Trie,
    index: TrieIndex,
    suffix_idx: libc::c_short,
    is_suffix: libc::c_short,
}

#[repr(C)]
pub struct TrieIterator {
    root: *const TrieState,
    state: *mut TrieState,
    key: *mut TrieString,
}

pub const NULL: libc::c_int = 0 as libc::c_int;

#[deprecated(note = "Use Trie::new()")]
#[no_mangle]
pub extern "C" fn trie_new(alpha_map: *const AlphaMap) -> *mut Trie {
    println!("trie_new: Rust!");
    let trie = Trie::new(unsafe { &*alpha_map });
    Box::into_raw(Box::new(trie))
}

#[deprecated(note="Use Trie::from_file()")]
#[no_mangle]
pub extern "C" fn trie_new_from_file(path: *const libc::c_char) -> *mut Trie {
    println!("trie_new_from_file: Rust!");
    let str = unsafe { CStr::from_ptr(path) };
    let osstr = OsStr::from_bytes(str.to_bytes());
    let Ok(trie) = Trie::from_file(osstr) else { return ptr::null_mut() };
    Box::into_raw(Box::new(trie))
}

#[deprecated(note="Use Trie::from_reader()")]
#[no_mangle]
pub extern "C" fn trie_fread(mut file: NonNull<libc::FILE>) -> *mut Trie {
    let mut file = wrap_cfile_nonnull(file);
    let Ok(trie) = Trie::from_reader(&mut file) else { return ptr::null_mut() };
    Box::into_raw(Box::new(trie))
}

#[no_mangle]
pub unsafe extern "C" fn trie_free(trie: *mut Trie) {
    drop(Box::from_raw(trie))
}

#[no_mangle]
pub unsafe extern "C" fn trie_save(
    mut trie: *mut Trie,
    mut path: *const libc::c_char,
) -> libc::c_int {
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut res: libc::c_int = 0 as libc::c_int;
    file = fopen(path, b"wb+\0" as *const u8 as *const libc::c_char);
    if file.is_null() {
        return -(1 as libc::c_int);
    }
    res = trie_fwrite(trie, NonNull::new_unchecked(file));
    fclose(file);
    return res;
}

#[deprecated(note = "Use trie.serialized_size()")]
#[no_mangle]
pub unsafe extern "C" fn trie_get_serialized_size(trie: *const Trie) -> usize {
    let trie = unsafe { &*trie };
    trie.serialized_size()
}

#[no_mangle]
pub unsafe extern "C" fn trie_serialize(mut trie: *mut Trie, mut ptr: *mut u8) {
    let mut ptr1: *mut uint8 = ptr;
    alpha_map_serialize_bin((*trie).alpha_map, mem::transmute(&mut ptr1));
    da_serialize((*trie).da, mem::transmute(&mut ptr1));
    tail_serialize((*trie).tail, mem::transmute(&mut ptr1));
    (*trie).is_dirty = FALSE as Bool;
}

#[no_mangle]
pub unsafe extern "C" fn trie_fwrite(mut trie: *mut Trie, mut file: NonNull<FILE>) -> libc::c_int {
    if alpha_map_fwrite_bin((*trie).alpha_map, file) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if da_fwrite((*trie).da, file) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if tail_fwrite((*trie).tail, file) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    (*trie).is_dirty = FALSE as Bool;
    return 0 as libc::c_int;
}

#[deprecated(note = "Use trie.is_dirty()")]
#[no_mangle]
pub extern "C" fn trie_is_dirty(trie: *const Trie) -> Bool {
    let trie = unsafe { &*trie };
    trie.is_dirty().into()
}

#[no_mangle]
pub unsafe extern "C" fn trie_retrieve(
    mut trie: *const Trie,
    mut key: *const AlphaChar,
    mut o_data: *mut TrieData,
) -> Bool {
    let mut s: TrieIndex = 0;
    let mut suffix_idx: libc::c_short = 0;
    let mut p: *const AlphaChar = 0 as *const AlphaChar;
    s = da_get_root((*trie).da);
    p = key;
    while !(da_get_base((*trie).da, s) < 0 as libc::c_int) {
        let mut tc: TrieIndex = alpha_map_char_to_trie((*trie).alpha_map, *p);
        if TRIE_INDEX_MAX == tc {
            return FALSE as Bool;
        }
        if !da_walk((*trie).da, &mut s, tc as TrieChar) {
            return FALSE as Bool;
        }
        if 0 as libc::c_int as AlphaChar == *p {
            break;
        }
        p = p.offset(1);
        p;
    }
    s = -da_get_base((*trie).da, s);
    suffix_idx = 0 as libc::c_int as libc::c_short;
    loop {
        let mut tc_0: TrieIndex = alpha_map_char_to_trie((*trie).alpha_map, *p);
        if TRIE_INDEX_MAX == tc_0 {
            return FALSE as Bool;
        }
        if !tail_walk_char((*trie).tail, s, &mut suffix_idx, tc_0 as TrieChar) {
            return FALSE as Bool;
        }
        if 0 as libc::c_int as AlphaChar == *p {
            break;
        }
        p = p.offset(1);
        p;
    }
    if !o_data.is_null() {
        *o_data = tail_get_data((*trie).tail, s);
    }
    return TRUE as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn trie_store(
    mut trie: *mut Trie,
    mut key: *const AlphaChar,
    mut data: TrieData,
) -> Bool {
    return trie_store_conditionally(trie, key, data, TRUE as Bool);
}
#[no_mangle]
pub unsafe extern "C" fn trie_store_if_absent(
    mut trie: *mut Trie,
    mut key: *const AlphaChar,
    mut data: TrieData,
) -> Bool {
    return trie_store_conditionally(trie, key, data, FALSE as Bool);
}
unsafe extern "C" fn trie_store_conditionally(
    mut trie: *mut Trie,
    mut key: *const AlphaChar,
    mut data: TrieData,
    mut is_overwrite: Bool,
) -> Bool {
    let mut s: TrieIndex = 0;
    let mut t: TrieIndex = 0;
    let mut suffix_idx: libc::c_short = 0;
    let mut p: *const AlphaChar = 0 as *const AlphaChar;
    let mut sep: *const AlphaChar = 0 as *const AlphaChar;
    s = da_get_root((*trie).da);
    p = key;
    while !(da_get_base((*trie).da, s) < 0 as libc::c_int) {
        let mut tc: TrieIndex = alpha_map_char_to_trie((*trie).alpha_map, *p);
        if TRIE_INDEX_MAX == tc {
            return FALSE as Bool;
        }
        if !da_walk((*trie).da, &mut s, tc as TrieChar) {
            let mut key_str: *mut TrieChar = 0 as *mut TrieChar;
            let mut res: Bool = DA_FALSE;
            key_str = alpha_map_char_to_trie_str((*trie).alpha_map, p);
            if key_str.is_null() {
                return FALSE as Bool;
            }
            res = trie_branch_in_branch(trie, s, key_str, data);
            free(key_str as *mut libc::c_void);
            return res;
        }
        if 0 as libc::c_int as AlphaChar == *p {
            break;
        }
        p = p.offset(1);
        p;
    }
    sep = p;
    t = -da_get_base((*trie).da, s);
    suffix_idx = 0 as libc::c_int as libc::c_short;
    loop {
        let mut tc_0: TrieIndex = alpha_map_char_to_trie((*trie).alpha_map, *p);
        if TRIE_INDEX_MAX == tc_0 {
            return FALSE as Bool;
        }
        if !tail_walk_char((*trie).tail, t, &mut suffix_idx, tc_0 as TrieChar) {
            let mut tail_str: *mut TrieChar = 0 as *mut TrieChar;
            let mut res_0: Bool = DA_FALSE;
            tail_str = alpha_map_char_to_trie_str((*trie).alpha_map, sep);
            if tail_str.is_null() {
                return FALSE as Bool;
            }
            res_0 = trie_branch_in_tail(trie, s, tail_str, data);
            free(tail_str as *mut libc::c_void);
            return res_0;
        }
        if 0 as libc::c_int as AlphaChar == *p {
            break;
        }
        p = p.offset(1);
        p;
    }
    if !is_overwrite {
        return FALSE as Bool;
    }
    tail_set_data(NonNull::new_unchecked((*trie).tail), t, data);
    (*trie).is_dirty = TRUE as Bool;
    return TRUE as Bool;
}
unsafe extern "C" fn trie_branch_in_branch(
    mut trie: *mut Trie,
    mut sep_node: TrieIndex,
    mut suffix: *const TrieChar,
    mut data: TrieData,
) -> Bool {
    let mut new_da: TrieIndex = 0;
    let mut new_tail: TrieIndex = 0;
    new_da = da_insert_branch(NonNull::new_unchecked((*trie).da), sep_node, *suffix);
    if TRIE_INDEX_ERROR == new_da {
        return FALSE as Bool;
    }
    if TRIE_CHAR_TERM != *suffix {
        suffix = suffix.offset(1);
        suffix;
    }
    new_tail = tail_add_suffix(NonNull::new_unchecked((*trie).tail), suffix);
    tail_set_data(NonNull::new_unchecked((*trie).tail), new_tail, data);
    da_set_base(NonNull::new_unchecked((*trie).da), new_da, -new_tail);
    (*trie).is_dirty = TRUE as Bool;
    return TRUE as Bool;
}
unsafe extern "C" fn trie_branch_in_tail(
    mut trie: *mut Trie,
    mut sep_node: TrieIndex,
    mut suffix: *const TrieChar,
    mut data: TrieData,
) -> Bool {
    let mut current_block: u64;
    let mut old_tail: TrieIndex = 0;
    let mut old_da: TrieIndex = 0;
    let mut s: TrieIndex = 0;
    let mut old_suffix: *const TrieChar = 0 as *const TrieChar;
    let mut p: *const TrieChar = 0 as *const TrieChar;
    old_tail = -da_get_base((*trie).da, sep_node);
    old_suffix = tail_get_suffix((*trie).tail, old_tail);
    if old_suffix.is_null() {
        return FALSE as Bool;
    }
    p = old_suffix;
    s = sep_node;
    loop {
        if !(*p as libc::c_int == *suffix as libc::c_int) {
            current_block = 6937071982253665452;
            break;
        }
        let mut t: TrieIndex = da_insert_branch(NonNull::new_unchecked((*trie).da), s, *p);
        if TRIE_INDEX_ERROR == t {
            current_block = 3141307649429454656;
            break;
        }
        s = t;
        p = p.offset(1);
        p;
        suffix = suffix.offset(1);
        suffix;
    }
    match current_block {
        6937071982253665452 => {
            old_da = da_insert_branch(NonNull::new_unchecked((*trie).da), s, *p);
            if !(TRIE_INDEX_ERROR == old_da) {
                if TRIE_CHAR_TERM != *p {
                    p = p.offset(1);
                    p;
                }
                tail_set_suffix(NonNull::new_unchecked((*trie).tail), old_tail, p);
                da_set_base(NonNull::new_unchecked((*trie).da), old_da, -old_tail);
                return trie_branch_in_branch(trie, s, suffix, data);
            }
        }
        _ => {}
    }
    da_prune_upto(NonNull::new_unchecked((*trie).da), sep_node, s);
    da_set_base(NonNull::new_unchecked((*trie).da), sep_node, -old_tail);
    return FALSE as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn trie_delete(mut trie: *mut Trie, mut key: *const AlphaChar) -> Bool {
    let mut s: TrieIndex = 0;
    let mut t: TrieIndex = 0;
    let mut suffix_idx: libc::c_short = 0;
    let mut p: *const AlphaChar = 0 as *const AlphaChar;
    s = da_get_root((*trie).da);
    p = key;
    while !(da_get_base((*trie).da, s) < 0 as libc::c_int) {
        let mut tc: TrieIndex = alpha_map_char_to_trie((*trie).alpha_map, *p);
        if TRIE_INDEX_MAX == tc {
            return FALSE as Bool;
        }
        if !da_walk((*trie).da, &mut s, tc as TrieChar) {
            return FALSE as Bool;
        }
        if 0 as libc::c_int as AlphaChar == *p {
            break;
        }
        p = p.offset(1);
        p;
    }
    t = -da_get_base((*trie).da, s);
    suffix_idx = 0 as libc::c_int as libc::c_short;
    loop {
        let mut tc_0: TrieIndex = alpha_map_char_to_trie((*trie).alpha_map, *p);
        if TRIE_INDEX_MAX == tc_0 {
            return FALSE as Bool;
        }
        if !tail_walk_char((*trie).tail, t, &mut suffix_idx, tc_0 as TrieChar) {
            return FALSE as Bool;
        }
        if 0 as libc::c_int as AlphaChar == *p {
            break;
        }
        p = p.offset(1);
        p;
    }
    tail_delete(NonNull::new_unchecked((*trie).tail), t);
    da_set_base(NonNull::new_unchecked((*trie).da), s, TRIE_INDEX_ERROR);
    da_prune(NonNull::new_unchecked((*trie).da), s);
    (*trie).is_dirty = TRUE as Bool;
    return TRUE as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn trie_enumerate(
    mut trie: *const Trie,
    mut enum_func: TrieEnumFunc,
    mut user_data: *mut libc::c_void,
) -> Bool {
    let mut root: *mut TrieState = 0 as *mut TrieState;
    let mut iter: *mut TrieIterator = 0 as *mut TrieIterator;
    let mut cont: Bool = TRUE as Bool;
    root = trie_root(trie);
    if root.is_null() {
        return FALSE as Bool;
    }
    iter = trie_iterator_new(root);
    if iter.is_null() {
        trie_state_free(root);
        return FALSE as Bool;
    } else {
        while cont.into() && trie_iterator_next(iter).into() {
            let mut key: *mut AlphaChar = trie_iterator_get_key(iter);
            let mut data: TrieData = trie_iterator_get_data(iter);
            cont = (Some(enum_func.expect("non-null function pointer")))
                .expect("non-null function pointer")(key, data, user_data);
            free(key as *mut libc::c_void);
        }
        trie_iterator_free(iter);
        trie_state_free(root);
        return cont;
    };
}
#[no_mangle]
pub unsafe extern "C" fn trie_root(mut trie: *const Trie) -> *mut TrieState {
    return trie_state_new(
        trie,
        da_get_root((*trie).da),
        0 as libc::c_int as libc::c_short,
        Into::<u32>::into(FALSE) as libc::c_short,
    );
}
unsafe extern "C" fn trie_state_new(
    mut trie: *const Trie,
    mut index: TrieIndex,
    mut suffix_idx: libc::c_short,
    mut is_suffix: libc::c_short,
) -> *mut TrieState {
    let mut s: *mut TrieState = 0 as *mut TrieState;
    s = malloc(::core::mem::size_of::<TrieState>() as libc::c_ulong) as *mut TrieState;
    if s.is_null() {
        return NULL as *mut TrieState;
    }
    (*s).trie = trie;
    (*s).index = index;
    (*s).suffix_idx = suffix_idx;
    (*s).is_suffix = is_suffix;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_copy(mut dst: *mut TrieState, mut src: *const TrieState) {
    *dst = *src;
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_clone(mut s: *const TrieState) -> *mut TrieState {
    return trie_state_new((*s).trie, (*s).index, (*s).suffix_idx, (*s).is_suffix);
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_free(mut s: *mut TrieState) {
    free(s as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_rewind(mut s: *mut TrieState) {
    (*s).index = da_get_root((*(*s).trie).da);
    (*s).is_suffix = Into::<u32>::into(FALSE) as libc::c_short;
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_walk(mut s: *mut TrieState, mut c: AlphaChar) -> Bool {
    let mut tc: TrieIndex = alpha_map_char_to_trie((*(*s).trie).alpha_map, c);
    if 0x7fffffff as libc::c_int == tc {
        return FALSE as Bool;
    }
    if (*s).is_suffix == 0 {
        let mut ret: Bool = DA_FALSE;
        ret = da_walk((*(*s).trie).da, &mut (*s).index, tc as TrieChar);
        if ret.into() && da_get_base((*(*s).trie).da, (*s).index) < 0 as libc::c_int {
            (*s).index = -da_get_base((*(*s).trie).da, (*s).index);
            (*s).suffix_idx = 0 as libc::c_int as libc::c_short;
            (*s).is_suffix = Into::<u32>::into(TRUE) as libc::c_short;
        }
        return ret;
    } else {
        return tail_walk_char(
            (*(*s).trie).tail,
            (*s).index,
            &mut (*s).suffix_idx,
            tc as TrieChar,
        );
    };
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_is_walkable(mut s: *const TrieState, mut c: AlphaChar) -> Bool {
    let mut tc: TrieIndex = alpha_map_char_to_trie((*(*s).trie).alpha_map, c);
    if 0x7fffffff as libc::c_int == tc {
        return FALSE as Bool;
    }
    if (*s).is_suffix == 0 {
        return (da_get_check(
            (*(*s).trie).da,
            da_get_base((*(*s).trie).da, (*s).index) + tc as TrieChar as libc::c_int,
        ) == (*s).index)
            .into();
    } else {
        return (*(tail_get_suffix((*(*s).trie).tail, (*s).index)).offset((*s).suffix_idx as isize)
            as libc::c_int
            == tc as TrieChar as libc::c_int)
            .into();
    };
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_walkable_chars(
    mut s: *const TrieState,
    mut chars: *mut AlphaChar,
    mut chars_nelm: libc::c_int,
) -> libc::c_int {
    let mut syms_num: libc::c_int = 0 as libc::c_int;
    if (*s).is_suffix == 0 {
        let mut syms = da_output_symbols((*(*s).trie).da.cast(), (*s).index);
        let mut i: libc::c_int = 0;
        syms_num = syms.num() as libc::c_int;
        i = 0 as libc::c_int;
        while i < syms_num && i < chars_nelm {
            let mut tc: TrieChar = syms.get(i as usize).unwrap();
            *chars.offset(i as isize) = alpha_map_trie_to_char((*(*s).trie).alpha_map, tc);
            i += 1;
        }
    } else {
        let mut suffix: *const TrieChar = tail_get_suffix((*(*s).trie).tail, (*s).index);
        *chars.offset(0 as libc::c_int as isize) = alpha_map_trie_to_char(
            (*(*s).trie).alpha_map,
            *suffix.offset((*s).suffix_idx as isize),
        );
        syms_num = 1 as libc::c_int;
    }
    return syms_num;
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_is_single(mut s: *const TrieState) -> Bool {
    return (*s).is_suffix.into();
}
#[no_mangle]
pub unsafe extern "C" fn trie_state_get_data(mut s: *const TrieState) -> TrieData {
    if s.is_null() {
        return TRIE_DATA_ERROR;
    }
    if (*s).is_suffix == 0 {
        let mut index: TrieIndex = (*s).index;
        if da_walk((*(*s).trie).da, &mut index, TRIE_CHAR_TERM as TrieChar).into() {
            if da_get_base((*(*s).trie).da, index) < 0 as libc::c_int {
                index = -da_get_base((*(*s).trie).da, index);
                return tail_get_data((*(*s).trie).tail, index);
            }
        }
    } else if *(tail_get_suffix((*(*s).trie).tail, (*s).index)).offset((*s).suffix_idx as isize)
        as libc::c_int
        == '\0' as i32
    {
        return tail_get_data((*(*s).trie).tail, (*s).index);
    }
    return TRIE_DATA_ERROR;
}
#[no_mangle]
pub unsafe extern "C" fn trie_iterator_new(mut s: *mut TrieState) -> *mut TrieIterator {
    let mut iter: *mut TrieIterator = 0 as *mut TrieIterator;
    iter = malloc(::core::mem::size_of::<TrieIterator>() as libc::c_ulong) as *mut TrieIterator;
    if iter.is_null() {
        return NULL as *mut TrieIterator;
    }
    (*iter).root = s;
    (*iter).state = NULL as *mut TrieState;
    (*iter).key = NULL as *mut TrieString;
    return iter;
}
#[no_mangle]
pub unsafe extern "C" fn trie_iterator_free(mut iter: *mut TrieIterator) {
    if !((*iter).state).is_null() {
        trie_state_free((*iter).state);
    }
    if !((*iter).key).is_null() {
        trie_string_free((*iter).key);
    }
    free(iter as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn trie_iterator_next(mut iter: *mut TrieIterator) -> Bool {
    let mut s: *mut TrieState = (*iter).state;
    let mut sep: TrieIndex = 0;
    if s.is_null() {
        (*iter).state = trie_state_clone((*iter).root);
        s = (*iter).state;
        if (*s).is_suffix != 0 {
            return TRUE as Bool;
        }
        (*iter).key = trie_string_new(20 as libc::c_int);
        sep = da_first_separate(
            NonNull::new_unchecked((*(*s).trie).da),
            (*s).index,
            NonNull::new_unchecked((*iter).key),
        );
        if TRIE_INDEX_ERROR == sep {
            return FALSE as Bool;
        }
        (*s).index = sep;
        return TRUE as Bool;
    }
    if (*s).is_suffix != 0 {
        return FALSE as Bool;
    }
    sep = da_next_separate(
        NonNull::new_unchecked((*(*s).trie).da),
        (*(*iter).root).index,
        (*s).index,
        NonNull::new_unchecked((*iter).key),
    );
    if TRIE_INDEX_ERROR == sep {
        return FALSE as Bool;
    }
    (*s).index = sep;
    return TRUE as Bool;
}
#[no_mangle]
pub unsafe extern "C" fn trie_iterator_get_key(mut iter: *const TrieIterator) -> *mut AlphaChar {
    let mut s: *const TrieState = 0 as *const TrieState;
    let mut tail_str: *const TrieChar = 0 as *const TrieChar;
    let mut alpha_key: *mut AlphaChar = 0 as *mut AlphaChar;
    let mut alpha_p: *mut AlphaChar = 0 as *mut AlphaChar;
    s = (*iter).state;
    if s.is_null() {
        return NULL as *mut AlphaChar;
    }
    if (*s).is_suffix != 0 {
        tail_str = tail_get_suffix((*(*s).trie).tail, (*s).index);
        if tail_str.is_null() {
            return NULL as *mut AlphaChar;
        }
        tail_str = tail_str.offset((*s).suffix_idx as libc::c_int as isize);
        alpha_key = malloc(
            (::core::mem::size_of::<AlphaChar>() as libc::c_ulong).wrapping_mul(
                (trie_char_strlen(tail_str) as size_t).wrapping_add(1 as libc::c_int as size_t),
            ),
        ) as *mut AlphaChar;
        alpha_p = alpha_key;
    } else {
        let mut tail_idx: TrieIndex = 0;
        let mut i: libc::c_int = 0;
        let mut key_len: libc::c_int = 0;
        let mut key_p: *const TrieChar = 0 as *const TrieChar;
        tail_idx = -da_get_base((*(*s).trie).da, (*s).index);
        tail_str = tail_get_suffix((*(*s).trie).tail, tail_idx);
        if tail_str.is_null() {
            return NULL as *mut AlphaChar;
        }
        key_len = trie_string_length((*iter).key);
        key_p = trie_string_get_val((*iter).key) as *const TrieChar;
        alpha_key = malloc(
            (::core::mem::size_of::<AlphaChar>() as libc::c_ulong).wrapping_mul(
                (key_len as size_t)
                    .wrapping_add(trie_char_strlen(tail_str) as size_t)
                    .wrapping_add(1 as libc::c_int as size_t),
            ),
        ) as *mut AlphaChar;
        alpha_p = alpha_key;
        i = key_len;
        while i > 0 as libc::c_int {
            let fresh0 = key_p;
            key_p = key_p.offset(1);
            let fresh1 = alpha_p;
            alpha_p = alpha_p.offset(1);
            *fresh1 = alpha_map_trie_to_char((*(*s).trie).alpha_map, *fresh0);
            i -= 1;
            i;
        }
    }
    while TRIE_CHAR_TERM != *tail_str {
        let fresh2 = tail_str;
        tail_str = tail_str.offset(1);
        let fresh3 = alpha_p;
        alpha_p = alpha_p.offset(1);
        *fresh3 = alpha_map_trie_to_char((*(*s).trie).alpha_map, *fresh2);
    }
    *alpha_p = 0 as libc::c_int as AlphaChar;
    return alpha_key;
}
#[no_mangle]
pub unsafe extern "C" fn trie_iterator_get_data(mut iter: *const TrieIterator) -> TrieData {
    let mut s: *const TrieState = (*iter).state;
    let mut tail_index: TrieIndex = 0;
    if s.is_null() {
        return TRIE_DATA_ERROR;
    }
    if (*s).is_suffix == 0 {
        if !(da_get_base((*(*s).trie).da, (*s).index) < 0 as libc::c_int) {
            return TRIE_DATA_ERROR;
        }
        tail_index = -da_get_base((*(*s).trie).da, (*s).index);
    } else {
        tail_index = (*s).index;
    }
    return tail_get_data((*(*s).trie).tail, tail_index);
}
