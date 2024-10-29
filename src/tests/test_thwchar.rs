use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    fn th_uni2macthai(wc: thwchar_t) -> thchar_t;
    fn th_uni2winthai(wc: thwchar_t) -> thchar_t;
    fn th_uni2tis_line(s: *const thwchar_t, result: *mut thchar_t, n: size_t) -> libc::c_int;
    fn th_macthai2uni(c: thchar_t) -> thwchar_t;
    fn th_winthai2uni(c: thchar_t) -> thwchar_t;
    fn th_tis2uni_line(s: *const thchar_t, result: *mut thwchar_t, n: size_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
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
pub type wchar_t = libc::c_int;
pub type thchar_t = libc::c_uchar;
pub type thwchar_t = wchar_t;
pub const MAXLINELENGTH: libc::c_int = 1000 as libc::c_int;
static mut win_sample: [thchar_t; 46] = unsafe {
    *::core::mem::transmute::<
        &[u8; 46],
        &[thchar_t; 46],
    >(
        b"\xBE\x8B\xCD\xBB\xD9\x86\xBE\xD5\xE8\xBB\x82\x9B\xAE\xFC\x80\xD8\x90\xD8\xA1\xED\xD2\xBB\x99\xD2\xA1\xED\xE9\xD2\xBB\x99\x9C\xD2\xBB\x99\xD2 \x8C\xB7\x8B \xD5\xA1\xE7\xBB\x9A\0",
    )
};
static mut mac_sample: [thchar_t; 46] = unsafe {
    *::core::mem::transmute::<
        &[u8; 46],
        &[thchar_t; 46],
    >(
        b"\xBE\x88\xCD\xBB\xD9\x83\xBE\xD5\xE8\xBB\x95\x98\xAE\xD8\xB0\xD8\xAD\xD8\xA1\xED\xD2\xBB\x8F\xD2\xA1\xED\xE9\xD2\xBB\x8F\x99\xD2\xBB\x8F\xD2 \x89\xB7\x88 \xD5\xA1\xE7\xBB\x93\0",
    )
};
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut tis_620_0: [thchar_t; 1000] = [0; 1000];
    let mut newtis_620_0: [thchar_t; 1000] = [0; 1000];
    let mut uni: [thwchar_t; 1000] = [0; 1000];
    let mut tisLength: libc::c_int = 0;
    let mut uniLength: libc::c_int = 0;
    let mut pc: *const thchar_t = 0 as *const thchar_t;
    let mut err: libc::c_int = 0 as libc::c_int;
    strcpy(
        tis_620_0.as_mut_ptr() as *mut libc::c_char,
        b"\xCA\xC7\xD1\xCA\xB4\xD5\xA4\xC3\xD1\xBA \xB9\xD5\xE8\xE0\xBB\xE7\xB9\xA1\xD2\xC3\xB7\xB4\xCA\xCD\xBA\xB5\xD1\xC7\xE0\xCD\xA7\0"
            as *const u8 as *const libc::c_char,
    );
    fprintf(
        stderr,
        b"Testing thwchar...\n\0" as *const u8 as *const libc::c_char,
    );
    tisLength = strlen(tis_620_0.as_mut_ptr() as *const libc::c_char) as libc::c_int;
    fprintf(
        stderr,
        b"Input:  tis-620-0 string of length %d: %s\n\0" as *const u8 as *const libc::c_char,
        tisLength,
        tis_620_0.as_mut_ptr(),
    );
    uniLength = th_tis2uni_line(
        tis_620_0.as_mut_ptr(),
        uni.as_mut_ptr(),
        MAXLINELENGTH as size_t,
    );
    fprintf(
        stderr,
        b"Output: Unicode string of length %d, wcslen = %ld\n\0" as *const u8
            as *const libc::c_char,
        uniLength,
        wcslen(uni.as_mut_ptr()) as libc::c_long,
    );
    if uniLength != tisLength {
        fprintf(
            stderr,
            b"th_tis2uni_line() returns different length %d from original %d\n\0" as *const u8
                as *const libc::c_char,
            uniLength,
            tisLength,
        );
        err = 1 as libc::c_int;
    }
    fprintf(
        stderr,
        b"\nConvert back to tis-620-0 string...\n\0" as *const u8 as *const libc::c_char,
    );
    tisLength = th_uni2tis_line(
        uni.as_mut_ptr(),
        newtis_620_0.as_mut_ptr(),
        MAXLINELENGTH as size_t,
    );
    fprintf(
        stderr,
        b"Output: tis-620-0 string of length %d, strlen = %ld: %s\n\0" as *const u8
            as *const libc::c_char,
        tisLength,
        strlen(newtis_620_0.as_mut_ptr() as *const libc::c_char) as libc::c_long,
        newtis_620_0.as_mut_ptr(),
    );
    if tisLength != uniLength {
        fprintf(
            stderr,
            b"th_uni2tis_line() returns different length %d from original %d\n\0" as *const u8
                as *const libc::c_char,
            tisLength,
            uniLength,
        );
        err = 1 as libc::c_int;
    }
    if strcmp(
        newtis_620_0.as_mut_ptr() as *const libc::c_char,
        tis_620_0.as_mut_ptr() as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        fprintf(
            stderr,
            b" Input = output, correct! Test thwchar OK.\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        fprintf(
            stderr,
            b" Input != output, incorrect!!\n\0" as *const u8 as *const libc::c_char,
        );
        err = 1 as libc::c_int;
    }
    pc = win_sample.as_ptr();
    while *pc != 0 {
        if th_uni2winthai(th_winthai2uni(*pc)) as libc::c_int != *pc as libc::c_int {
            fprintf(
                stderr,
                b"Inconsistent uni<->winthai conv: %02x -> %04lx, %02x\n\0" as *const u8
                    as *const libc::c_char,
                *pc as libc::c_int,
                th_winthai2uni(*pc) as libc::c_ulong,
                th_uni2winthai(th_winthai2uni(*pc)) as libc::c_int,
            );
            err = 1 as libc::c_int;
        }
        pc = pc.offset(1);
        pc;
    }
    pc = mac_sample.as_ptr();
    while *pc != 0 {
        if th_uni2macthai(th_macthai2uni(*pc)) as libc::c_int != *pc as libc::c_int {
            fprintf(
                stderr,
                b"Inconsistent uni<->macthai conv: %02x -> %04lx, %02x\n\0" as *const u8
                    as *const libc::c_char,
                *pc as libc::c_int,
                th_macthai2uni(*pc) as libc::c_ulong,
                th_uni2macthai(th_macthai2uni(*pc)) as libc::c_int,
            );
            err = 1 as libc::c_int;
        }
        pc = pc.offset(1);
        pc;
    }
    return err;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
