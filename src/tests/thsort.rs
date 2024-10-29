use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn th_strcoll(s1: *const thchar_t, s2: *const thchar_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    fn perror(__s: *const libc::c_char);
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn qsort(__base: *mut libc::c_void, __nmemb: size_t, __size: size_t, __compar: __compar_fn_t);
}
pub type size_t = libc::c_ulong;
pub type thchar_t = libc::c_uchar;
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
pub type __compar_fn_t =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
pub type CMPFUNC =
    Option<unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int>;
pub const NULL: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn readData(
    mut dataFile: *mut FILE,
    mut data_0: *mut *mut thchar_t,
    mut maxData: size_t,
) -> size_t {
    let mut nData: size_t = 0 as libc::c_int as size_t;
    static mut wordBuf: [libc::c_char; 128] = [0; 128];
    while nData < maxData
        && !(fgets(
            wordBuf.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong as libc::c_int,
            dataFile,
        ))
        .is_null()
    {
        let mut len: libc::c_int = strlen(wordBuf.as_mut_ptr()) as libc::c_int;
        if len == 0 as libc::c_int {
            return nData;
        }
        len -= 1;
        wordBuf[len as usize] = 0 as libc::c_int as libc::c_char;
        let ref mut fresh0 = *data_0.offset(nData as isize);
        *fresh0 = malloc((len + 1 as libc::c_int) as libc::c_ulong) as *mut thchar_t;
        if (*data_0.offset(nData as isize)).is_null() {
            printf(
                b"Warning: Only %ld items were read\n\0" as *const u8 as *const libc::c_char,
                nData as libc::c_long,
            );
            return nData;
        }
        strcpy(
            *data_0.offset(nData as isize) as *mut libc::c_char,
            wordBuf.as_mut_ptr(),
        );
        nData = nData.wrapping_add(1);
        nData;
    }
    return nData;
}
unsafe extern "C" fn freeData(mut data_0: *mut *mut thchar_t, mut nItems: size_t) {
    let mut i: size_t = 0;
    i = 0 as libc::c_int as size_t;
    while i < nItems {
        free(*data_0.offset(i as isize) as *mut libc::c_void);
        i = i.wrapping_add(1);
        i;
    }
}
unsafe extern "C" fn dataCmp(
    mut pStr1: *mut *const thchar_t,
    mut pStr2: *mut *const thchar_t,
) -> libc::c_int {
    return th_strcoll(*pStr1, *pStr2);
}
unsafe extern "C" fn sortData(mut data_0: *mut *mut thchar_t, mut nItems: size_t) {
    qsort(
        data_0 as *mut libc::c_void,
        nItems,
        ::core::mem::size_of::<*mut thchar_t>() as libc::c_ulong,
        ::core::mem::transmute::<
            Option<unsafe extern "C" fn(*mut *const thchar_t, *mut *const thchar_t) -> libc::c_int>,
            CMPFUNC,
        >(Some(
            dataCmp
                as unsafe extern "C" fn(*mut *const thchar_t, *mut *const thchar_t) -> libc::c_int,
        )),
    );
}
unsafe extern "C" fn writeData(
    mut outFile: *mut FILE,
    mut data_0: *mut *mut thchar_t,
    mut nItems: size_t,
) {
    let mut i: size_t = 0;
    i = nItems;
    while i > 0 as libc::c_int as size_t {
        fprintf(
            outFile,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            *data_0,
        );
        data_0 = data_0.offset(1);
        data_0;
        i = i.wrapping_sub(1);
        i;
    }
}
pub const MAX_DATA: libc::c_int = 40000 as libc::c_int;
static mut data: [*mut thchar_t; 40000] = [0 as *const thchar_t as *mut thchar_t; 40000];
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut dataFile: *mut FILE = 0 as *mut FILE;
    let mut outFile: *mut FILE = 0 as *mut FILE;
    let mut dataRead: size_t = 0;
    let mut DataFileName: [libc::c_char; 512] = [0; 512];
    let mut OutFileName: [libc::c_char; 512] = [0; 512];
    if argc == 3 as libc::c_int {
        strcpy(
            DataFileName.as_mut_ptr(),
            *argv.offset(1 as libc::c_int as isize),
        );
        strcpy(
            OutFileName.as_mut_ptr(),
            *argv.offset(2 as libc::c_int as isize),
        );
    } else {
        fprintf(
            stderr,
            b"Usage: thsort <input file> <output file>\n\0" as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    dataFile = fopen(
        DataFileName.as_mut_ptr(),
        b"rt\0" as *const u8 as *const libc::c_char,
    );
    if dataFile.is_null() {
        fprintf(
            stderr,
            b"Can't open file %s\n\0" as *const u8 as *const libc::c_char,
            DataFileName.as_mut_ptr(),
        );
        perror(b"fopen\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    outFile = fopen(
        OutFileName.as_mut_ptr(),
        b"wt\0" as *const u8 as *const libc::c_char,
    );
    if outFile.is_null() {
        fprintf(
            stderr,
            b"Can't open file %s for write\n\0" as *const u8 as *const libc::c_char,
            OutFileName.as_mut_ptr(),
        );
        perror(b"fopen\0" as *const u8 as *const libc::c_char);
        return 1 as libc::c_int;
    }
    dataRead = readData(dataFile, data.as_mut_ptr(), MAX_DATA as size_t);
    sortData(data.as_mut_ptr(), dataRead);
    writeData(outFile, data.as_mut_ptr(), dataRead);
    freeData(data.as_mut_ptr(), dataRead);
    fclose(outFile);
    fclose(dataFile);
    return 0 as libc::c_int;
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
