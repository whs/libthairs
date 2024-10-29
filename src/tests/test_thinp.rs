use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn th_isaccept(c1: thchar_t, c2: thchar_t, s: thstrict_t) -> libc::c_int;
    fn th_prev_cell(
        s: *const thchar_t,
        pos: size_t,
        cell: *mut thcell_t,
        is_decomp_am: libc::c_int,
    ) -> size_t;
    fn th_validate(context: thcell_t, c: thchar_t, conv: *mut thinpconv_t) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
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
pub type thstrict_t = libc::c_uint;
pub const ISC_STRICT: thstrict_t = 2;
pub const ISC_BASICCHECK: thstrict_t = 1;
pub const ISC_PASSTHROUGH: thstrict_t = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct thinpconv_t {
    pub conv: [thchar_t; 4],
    pub offset: libc::c_int,
}
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub _prevchain: *mut *mut _IO_FILE,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[no_mangle]
pub static mut test_keys: [thchar_t; 55] = unsafe {
    *::core::mem::transmute::<
        &[u8; 55],
        &[thchar_t; 55],
    >(
        b"\xB9\xE9\xD3\xB9\xD3\xE9\xA1\xD5\xE8\xA1\xE8\xD5\xA1\xD8\xE8\xA1\xE8\xD8\xA1\xD8\xEC\xA1\xEC\xD8\xA1\xD4\xEC\xA1\xEC\xD4\xE0\xD4\xE0\xD3\xE0\xE9\xB9\xD4\xEC\xD1\xB9\xEC\xD1\xB9\xD8\xE8\xD4\xB9\xD2\xEC\xC4\xD2\xC6\xD2\0",
    )
};
#[no_mangle]
pub static mut res_level0: [thchar_t; 55] = unsafe {
    *::core::mem::transmute::<
        &[u8; 55],
        &[thchar_t; 55],
    >(
        b"\xB9\xE9\xD3\xB9\xD3\xE9\xA1\xD5\xE8\xA1\xE8\xD5\xA1\xD8\xE8\xA1\xE8\xD8\xA1\xD8\xEC\xA1\xEC\xD8\xA1\xD4\xEC\xA1\xEC\xD4\xE0\xD4\xE0\xD3\xE0\xE9\xB9\xD4\xEC\xD1\xB9\xEC\xD1\xB9\xD8\xE8\xD4\xB9\xD2\xEC\xC4\xD2\xC6\xD2\0",
    )
};
#[no_mangle]
pub static mut res_level1: [thchar_t; 44] = unsafe {
    *::core::mem::transmute::<
        &[u8; 44],
        &[thchar_t; 44],
    >(
        b"\xB9\xE9\xD3\xB9\xD3\xA1\xD5\xE8\xA1\xE8\xA1\xD8\xE8\xA1\xE8\xA1\xD8\xEC\xA1\xEC\xA1\xD4\xEC\xA1\xEC\xE0\xE0\xD3\xE0\xB9\xD4\xEC\xB9\xEC\xB9\xD8\xE8\xB9\xD2\xC4\xD2\xC6\xD2\0",
    )
};
#[no_mangle]
pub static mut res_level2: [thchar_t; 38] = unsafe {
    *::core::mem::transmute::<
        &[u8; 38],
        &[thchar_t; 38],
    >(
        b"\xB9\xE9\xD3\xB9\xD3\xA1\xD5\xE8\xA1\xE8\xA1\xD8\xE8\xA1\xE8\xA1\xD8\xEC\xA1\xEC\xA1\xD4\xEC\xA1\xEC\xE0\xB9\xD4\xEC\xB9\xEC\xB9\xD8\xE8\xB9\xD2\xC4\0",
    )
};
#[no_mangle]
pub static mut res_validate: [thchar_t; 45] = unsafe {
    *::core::mem::transmute::<
        &[u8; 45],
        &[thchar_t; 45],
    >(
        b"\xB9\xE9\xD3\xB9\xE9\xD3\xA1\xD5\xE8\xA1\xD5\xE8\xA1\xD8\xE8\xA1\xD8\xE8\xA1\xD8\xEC\xA1\xD8\xEC\xA1\xD4\xEC\xA1\xD4\xEC\xE0\xB9\xD1\xB9\xD1\xB9\xD4\xE8\xB9\xD2\xC4\xE5\xC6\xE5\0",
    )
};
unsafe extern "C" fn test_simple_input(
    mut keys: *const thchar_t,
    mut ans: *const thchar_t,
    mut level: libc::c_int,
) -> libc::c_int {
    let mut buffer: [thchar_t; 80] = [0; 80];
    let mut cur_pos: libc::c_int = 0 as libc::c_int;
    let mut err_no: libc::c_int = 0 as libc::c_int;
    while *keys != 0 {
        let mut prev_c: thchar_t = (if cur_pos != 0 {
            buffer[(cur_pos - 1 as libc::c_int) as usize] as libc::c_int
        } else {
            0 as libc::c_int
        }) as thchar_t;
        if th_isaccept(prev_c, *keys, level as thstrict_t) != 0 {
            let fresh0 = cur_pos;
            cur_pos = cur_pos + 1;
            buffer[fresh0 as usize] = *keys;
        }
        keys = keys.offset(1);
        keys;
    }
    buffer[cur_pos as usize] = 0 as libc::c_int as thchar_t;
    err_no = strcmp(
        buffer.as_mut_ptr() as *const libc::c_char,
        ans as *const libc::c_char,
    );
    if err_no != 0 as libc::c_int {
        fprintf(
            stderr,
            b"(%s)!=(%s)\n\0" as *const u8 as *const libc::c_char,
            buffer.as_mut_ptr(),
            ans,
        );
    }
    return err_no;
}
unsafe extern "C" fn test_th_isaccept() -> libc::c_int {
    let mut err_no: libc::c_int = 0 as libc::c_int;
    err_no += test_simple_input(test_keys.as_ptr(), res_level0.as_ptr(), 0 as libc::c_int);
    err_no += test_simple_input(test_keys.as_ptr(), res_level1.as_ptr(), 1 as libc::c_int);
    err_no += test_simple_input(test_keys.as_ptr(), res_level2.as_ptr(), 2 as libc::c_int);
    return err_no;
}
unsafe extern "C" fn test_th_validate() -> libc::c_int {
    let mut err_no: libc::c_int = 0 as libc::c_int;
    let mut buffer: [thchar_t; 80] = [0; 80];
    let mut cur_pos: libc::c_int = 0 as libc::c_int;
    let mut keys: *const thchar_t = test_keys.as_ptr();
    while *keys != 0 {
        let mut prev_cell: thcell_t = thcell_t {
            base: 0,
            hilo: 0,
            top: 0,
        };
        let mut conv: thinpconv_t = thinpconv_t {
            conv: [0; 4],
            offset: 0,
        };
        th_prev_cell(
            buffer.as_mut_ptr(),
            cur_pos as size_t,
            &mut prev_cell,
            1 as libc::c_int,
        );
        if th_validate(prev_cell, *keys, &mut conv) != 0 {
            strcpy(
                &mut *buffer.as_mut_ptr().offset((cur_pos + conv.offset) as isize) as *mut thchar_t
                    as *mut libc::c_char,
                (conv.conv).as_mut_ptr() as *const libc::c_char,
            );
            cur_pos = (cur_pos as libc::c_ulong).wrapping_add(
                (conv.offset as libc::c_ulong)
                    .wrapping_add(strlen((conv.conv).as_mut_ptr() as *const libc::c_char)),
            ) as libc::c_int as libc::c_int;
        }
        keys = keys.offset(1);
        keys;
    }
    buffer[cur_pos as usize] = 0 as libc::c_int as thchar_t;
    err_no = strcmp(
        buffer.as_mut_ptr() as *const libc::c_char,
        res_validate.as_ptr() as *const libc::c_char,
    );
    if err_no != 0 as libc::c_int {
        fprintf(
            stderr,
            b"(%s)!=(%s)\n\0" as *const u8 as *const libc::c_char,
            buffer.as_mut_ptr(),
            res_validate.as_ptr(),
        );
    }
    return err_no;
}
unsafe fn main_0() -> libc::c_int {
    let mut err_no: libc::c_int = 0 as libc::c_int;
    err_no += test_th_isaccept();
    err_no += test_th_validate();
    return if err_no != 0 {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    };
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
