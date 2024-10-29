use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn th_render_text_tis(
        s: *const thchar_t,
        res: *mut thglyph_t,
        res_sz: size_t,
        is_decomp_am: libc::c_int,
    ) -> libc::c_int;
    fn th_render_text_win(
        s: *const thchar_t,
        res: *mut thglyph_t,
        res_sz: size_t,
        is_decomp_am: libc::c_int,
    ) -> libc::c_int;
    fn th_render_text_mac(
        s: *const thchar_t,
        res: *mut thglyph_t,
        res_sz: size_t,
        is_decomp_am: libc::c_int,
    ) -> libc::c_int;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type thchar_t = libc::c_uchar;
pub type thglyph_t = libc::c_uchar;
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
static mut test_msg: [thchar_t; 48] = unsafe {
    *::core::mem::transmute::<
        &[u8; 48],
        &[thchar_t; 48],
    >(
        b"\xBE\xE8\xCD\xBB\xD9\xE8\xBE\xD5\xE8\xBB\xD5\xE8\xAE\xD8\xB0\xD8\xAD\xD8\xA1\xD3\xBB\xD3\xA1\xE9\xD3\xBB\xE9\xD3\xBB\xD3\xE9\xB7\xE8\xD5\xA1\xE7\xBB\xE7\xA1\xD4\xED\xE0\xA1\xD7\xE7\xCD\xA1\0",
    )
};
static mut ans_tis: [thchar_t; 55] = unsafe {
    *::core::mem::transmute::<
        &[u8; 55],
        &[thchar_t; 55],
    >(
        b"\xBE\xE8\xCD\xBB\xD9\xE8\xBE\xD5\xE8\xBB\xD5\xE8\xAE\xD8\xB0\xD8\xAD\xD8\xA1\xED\xD2\xBB\xED\xD2\xA1\xED\xE9\xD2\xBB\xED\xE9\xD2\xBB\xED\xD2\xDD\xE9\xB7\xE8\xDD\xD5\xA1\xE7\xBB\xE7\xA1\xD4\xED\xE0\xA1\xD7\xE7\xCD\xA1\0",
    )
};
static mut ans_tis_nd: [thchar_t; 50] = unsafe {
    *::core::mem::transmute::<
        &[u8; 50],
        &[thchar_t; 50],
    >(
        b"\xBE\xE8\xCD\xBB\xD9\xE8\xBE\xD5\xE8\xBB\xD5\xE8\xAE\xD8\xB0\xD8\xAD\xD8\xA1\xD3\xBB\xD3\xA1\xE9\xD3\xBB\xE9\xD3\xBB\xD3\xDD\xE9\xB7\xE8\xDD\xD5\xA1\xE7\xBB\xE7\xA1\xD4\xED\xE0\xA1\xD7\xE7\xCD\xA1\0",
    )
};
static mut ans_win_nd: [thglyph_t; 50] = unsafe {
    *::core::mem::transmute::<
        &[u8; 50],
        &[thglyph_t; 50],
    >(
        b"\xBE\x8B\xCD\xBB\xD9\x86\xBE\xD5\xE8\xBB\x82\x9B\xAE\xFC\x80\xD8\x90\xD8\xA1\xD3\xBB\xD3\xA1\x8C\xD3\xBB\x87\xD3\xBB\xD3\xDD\x8C\xB7\x8B\xDD\xD5\xA1\xE7\xBB\x9A\xA1\xD4\xED\xE0\xA1\xD7\xE7\xCD\xA1\0",
    )
};
static mut ans_win: [thglyph_t; 55] = unsafe {
    *::core::mem::transmute::<
        &[u8; 55],
        &[thglyph_t; 55],
    >(
        b"\xBE\x8B\xCD\xBB\xD9\x86\xBE\xD5\xE8\xBB\x82\x9B\xAE\xFC\x80\xD8\x90\xD8\xA1\xED\xD2\xBB\x99\xD2\xA1\xED\xE9\xD2\xBB\x99\x9C\xD2\xBB\x99\xD2\xDD\x8C\xB7\x8B\xDD\xD5\xA1\xE7\xBB\x9A\xA1\xD4\xED\xE0\xA1\xD7\xE7\xCD\xA1\0",
    )
};
static mut ans_mac_nd: [thglyph_t; 50] = unsafe {
    *::core::mem::transmute::<
        &[u8; 50],
        &[thglyph_t; 50],
    >(
        b"\xBE\x88\xCD\xBB\xD9\x83\xBE\xD5\xE8\xBB\x95\x98\xAE\xFC\x80\xD8\x90\xD8\xA1\xD3\xBB\xD3\xA1\x89\xD3\xBB\x84\xD3\xBB\xD3\xDD\x89\xB7\x88\xDD\xD5\xA1\xE7\xBB\x93\xA1\xD4\xED\xE0\xA1\xD7\xE7\xCD\xA1\0",
    )
};
static mut ans_mac: [thglyph_t; 55] = unsafe {
    *::core::mem::transmute::<
        &[u8; 55],
        &[thglyph_t; 55],
    >(
        b"\xBE\x88\xCD\xBB\xD9\x83\xBE\xD5\xE8\xBB\x95\x98\xAE\xFC\x80\xD8\x90\xD8\xA1\xED\xD2\xBB\x8F\xD2\xA1\xED\xE9\xD2\xBB\x8F\x99\xD2\xBB\x8F\xD2\xDD\x89\xB7\x88\xDD\xD5\xA1\xE7\xBB\x93\xA1\xD4\xED\xE0\xA1\xD7\xE7\xCD\xA1\0",
    )
};
#[no_mangle]
pub unsafe extern "C" fn test_th_render_tis() -> libc::c_int {
    let mut rend_buff: [thglyph_t; 80] = [0; 80];
    let mut err_no: libc::c_int = 0 as libc::c_int;
    fprintf(
        stderr,
        b"Testing th_render_text_tis() w/o decomposing SARA AM\n\0" as *const u8
            as *const libc::c_char,
    );
    th_render_text_tis(
        test_msg.as_ptr(),
        rend_buff.as_mut_ptr(),
        ::core::mem::size_of::<[thglyph_t; 80]>() as libc::c_ulong,
        0 as libc::c_int,
    );
    if strcmp(
        rend_buff.as_mut_ptr() as *const libc::c_char,
        ans_tis_nd.as_ptr() as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"(%s)!=(%s)\n\0" as *const u8 as *const libc::c_char,
            rend_buff.as_mut_ptr(),
            ans_tis_nd.as_ptr(),
        );
        err_no += 1;
        err_no;
    }
    fprintf(
        stderr,
        b"Testing th_render_text_tis() decomposing SARA AM\n\0" as *const u8 as *const libc::c_char,
    );
    th_render_text_tis(
        test_msg.as_ptr(),
        rend_buff.as_mut_ptr(),
        ::core::mem::size_of::<[thglyph_t; 80]>() as libc::c_ulong,
        1 as libc::c_int,
    );
    if strcmp(
        rend_buff.as_mut_ptr() as *const libc::c_char,
        ans_tis.as_ptr() as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"(%s)!=(%s)\n\0" as *const u8 as *const libc::c_char,
            rend_buff.as_mut_ptr(),
            ans_tis.as_ptr(),
        );
        err_no += 1;
        err_no;
    }
    return err_no;
}
#[no_mangle]
pub unsafe extern "C" fn test_th_render_win() -> libc::c_int {
    let mut rend_buff: [thglyph_t; 80] = [0; 80];
    let mut err_no: libc::c_int = 0 as libc::c_int;
    fprintf(
        stderr,
        b"Testing th_render_text_win() w/o decomposing SARA AM\n\0" as *const u8
            as *const libc::c_char,
    );
    th_render_text_win(
        test_msg.as_ptr(),
        rend_buff.as_mut_ptr(),
        ::core::mem::size_of::<[thglyph_t; 80]>() as libc::c_ulong,
        0 as libc::c_int,
    );
    if strcmp(
        rend_buff.as_mut_ptr() as *const libc::c_char,
        ans_win_nd.as_ptr() as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"(%s)!=(%s)\n\0" as *const u8 as *const libc::c_char,
            rend_buff.as_mut_ptr(),
            ans_win_nd.as_ptr(),
        );
        err_no += 1;
        err_no;
    }
    fprintf(
        stderr,
        b"Testing th_render_text_win() decomposing SARA AM\n\0" as *const u8 as *const libc::c_char,
    );
    th_render_text_win(
        test_msg.as_ptr(),
        rend_buff.as_mut_ptr(),
        ::core::mem::size_of::<[thglyph_t; 80]>() as libc::c_ulong,
        1 as libc::c_int,
    );
    if strcmp(
        rend_buff.as_mut_ptr() as *const libc::c_char,
        ans_win.as_ptr() as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"(%s)!=(%s)\n\0" as *const u8 as *const libc::c_char,
            rend_buff.as_mut_ptr(),
            ans_win.as_ptr(),
        );
        err_no += 1;
        err_no;
    }
    return err_no;
}
#[no_mangle]
pub unsafe extern "C" fn test_th_render_mac() -> libc::c_int {
    let mut rend_buff: [thglyph_t; 80] = [0; 80];
    let mut err_no: libc::c_int = 0 as libc::c_int;
    fprintf(
        stderr,
        b"Testing th_render_text_mac() w/o decomposing SARA AM\n\0" as *const u8
            as *const libc::c_char,
    );
    th_render_text_mac(
        test_msg.as_ptr(),
        rend_buff.as_mut_ptr(),
        ::core::mem::size_of::<[thglyph_t; 80]>() as libc::c_ulong,
        0 as libc::c_int,
    );
    if strcmp(
        rend_buff.as_mut_ptr() as *const libc::c_char,
        ans_mac_nd.as_ptr() as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"(%s)!=(%s)\n\0" as *const u8 as *const libc::c_char,
            rend_buff.as_mut_ptr(),
            ans_mac_nd.as_ptr(),
        );
        err_no += 1;
        err_no;
    }
    fprintf(
        stderr,
        b"Testing th_render_text_mac() decomposing SARA AM\n\0" as *const u8 as *const libc::c_char,
    );
    th_render_text_mac(
        test_msg.as_ptr(),
        rend_buff.as_mut_ptr(),
        ::core::mem::size_of::<[thglyph_t; 80]>() as libc::c_ulong,
        1 as libc::c_int,
    );
    if strcmp(
        rend_buff.as_mut_ptr() as *const libc::c_char,
        ans_mac.as_ptr() as *const libc::c_char,
    ) != 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"(%s)!=(%s)\n\0" as *const u8 as *const libc::c_char,
            rend_buff.as_mut_ptr(),
            ans_mac.as_ptr(),
        );
        err_no += 1;
        err_no;
    }
    return err_no;
}
unsafe fn main_0() -> libc::c_int {
    let mut err_no: libc::c_int = 0 as libc::c_int;
    err_no += test_th_render_tis();
    err_no += test_th_render_win();
    err_no += test_th_render_mac();
    return err_no;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
