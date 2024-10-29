use ::libc;
extern "C" {
    pub type _ThBrk;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn exit(_: libc::c_int) -> !;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn th_brk_new(dictpath: *const libc::c_char) -> *mut ThBrk;
    fn th_brk_delete(brk: *mut ThBrk);
    fn th_brk_insert_breaks(
        brk: *mut ThBrk,
        in_0: *const thchar_t,
        out: *mut thchar_t,
        out_sz: size_t,
        delim: *const libc::c_char,
    ) -> libc::c_int;
    fn wcslen(_: *const libc::c_int) -> libc::c_ulong;
    fn th_uni2tis_line(s: *const thwchar_t, result: *mut thchar_t, n: size_t) -> libc::c_int;
    fn th_tis2uni_line(s: *const thchar_t, result: *mut thwchar_t, n: size_t) -> libc::c_int;
    fn th_brk_wc_find_breaks(
        brk: *mut ThBrk,
        s: *const thwchar_t,
        pos: *mut libc::c_int,
        pos_sz: size_t,
    ) -> libc::c_int;
    fn th_brk_wc_insert_breaks(
        brk: *mut ThBrk,
        in_0: *const thwchar_t,
        out: *mut thwchar_t,
        out_sz: size_t,
        delim: *const thwchar_t,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type wchar_t = libc::c_int;
pub type thchar_t = libc::c_uchar;
pub type ThBrk = _ThBrk;
pub type thwchar_t = wchar_t;
pub const MAXLINELENGTH: libc::c_int = 1000 as libc::c_int;
pub const NULL: libc::c_int = 0 as libc::c_int;
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut str: [thchar_t; 1000] = [0; 1000];
    let mut ustr: [thwchar_t; 1000] = [0; 1000];
    let mut uout: [thwchar_t; 1000] = [0; 1000];
    let mut unicodeCutCode: [thwchar_t; 6] = [0; 6];
    let mut out1: [thchar_t; 2001] = [0; 2001];
    let mut out2: [thchar_t; 1000] = [0; 1000];
    let mut pos: [libc::c_int; 1000] = [0; 1000];
    let mut outputLength: libc::c_int = 0;
    let mut unicodeCutCodeLength: libc::c_int = 0;
    let mut numCut: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut brk: *mut ThBrk = 0 as *mut ThBrk;
    brk = th_brk_new(NULL as *const libc::c_char);
    if brk.is_null() {
        printf(b"Unable to create word breaker!\n\0" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    strcpy(
        str.as_mut_ptr() as *mut libc::c_char,
        b"\xCA\xC7\xD1\xCA\xB4\xD5\xA4\xC3\xD1\xBA \xA1\xCD.\xC3\xC1\xB9. \xB9\xD5\xE8\xE0\xBB\xE7\xB9\xA1\xD2\xC3\xB7\xB4\xCA\xCD\xBA\xB5\xD1\xC7\xE0\xCD\xA7\0"
            as *const u8 as *const libc::c_char,
    );
    printf(
        b"Testing with input string: %s\n\0" as *const u8 as *const libc::c_char,
        str.as_mut_ptr(),
    );
    printf(b"Converting to Unicode...\n\0" as *const u8 as *const libc::c_char);
    th_tis2uni_line(str.as_mut_ptr(), ustr.as_mut_ptr(), MAXLINELENGTH as size_t);
    printf(b"Calling th_brk_wc_find_breaks()...\n\0" as *const u8 as *const libc::c_char);
    numCut = th_brk_wc_find_breaks(
        brk,
        ustr.as_mut_ptr(),
        pos.as_mut_ptr(),
        MAXLINELENGTH as size_t,
    );
    printf(
        b"Total %d cut points.\0" as *const u8 as *const libc::c_char,
        numCut,
    );
    if numCut > 0 as libc::c_int {
        printf(
            b"Cut points list: %d\0" as *const u8 as *const libc::c_char,
            pos[0 as libc::c_int as usize],
        );
        i = 1 as libc::c_int;
        while i < numCut {
            printf(
                b", %d\0" as *const u8 as *const libc::c_char,
                pos[i as usize],
            );
            i += 1;
            i;
        }
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    if numCut != 7 as libc::c_int {
        printf(
            b"Error! Should have 7 cut points.\nTest th_brk_wc_find_breaks() failed...\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    unicodeCutCodeLength = th_tis2uni_line(
        b"<WBR>\0" as *const u8 as *const libc::c_char as *const thchar_t,
        unicodeCutCode.as_mut_ptr(),
        6 as libc::c_int as size_t,
    );
    if unicodeCutCodeLength as libc::c_ulong
        != strlen(b"<WBR>\0" as *const u8 as *const libc::c_char)
    {
        printf(
            b"Warning! Expect th_tis2uni_line() returned length %ld, got %d\n\0" as *const u8
                as *const libc::c_char,
            strlen(b"<WBR>\0" as *const u8 as *const libc::c_char) as libc::c_long,
            unicodeCutCodeLength,
        );
    }
    printf(b"Calling th_brk_wc_insert_breaks() ....\n\0" as *const u8 as *const libc::c_char);
    outputLength = th_brk_wc_insert_breaks(
        brk,
        ustr.as_mut_ptr(),
        uout.as_mut_ptr(),
        MAXLINELENGTH as size_t,
        unicodeCutCode.as_mut_ptr(),
    );
    printf(
        b"Return value from th_brk_wc_insert_breaks is %d\n\0" as *const u8 as *const libc::c_char,
        outputLength,
    );
    printf(
        b"Output string length is %ld\n\0" as *const u8 as *const libc::c_char,
        wcslen(uout.as_mut_ptr()) as libc::c_long,
    );
    if outputLength != 75 as libc::c_int {
        printf(
            b"Error! Output string length != 75. Test th_brk_wc_insert_breaks() failed...\n\0"
                as *const u8 as *const libc::c_char,
        );
        exit(-(1 as libc::c_int));
    }
    printf(
        b"Compare with result from th_brk_insert_breaks()..\n\0" as *const u8
            as *const libc::c_char,
    );
    th_brk_insert_breaks(
        brk,
        str.as_mut_ptr(),
        out1.as_mut_ptr(),
        (MAXLINELENGTH * 2 as libc::c_int + 1 as libc::c_int) as size_t,
        b"<WBR>\0" as *const u8 as *const libc::c_char,
    );
    th_uni2tis_line(
        uout.as_mut_ptr(),
        out2.as_mut_ptr(),
        MAXLINELENGTH as size_t,
    );
    if strcmp(
        out1.as_mut_ptr() as *const libc::c_char,
        out2.as_mut_ptr() as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        printf(
            b"Correct! .. test th_brk_wc_insert_breaks() passed...\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        printf(
            b"Error! Comparison of results from th_brk_insert_breaks() and th_brk_wc_insert_breaks() failed.\n\0"
                as *const u8 as *const libc::c_char,
        );
        printf(
            b"th_brk_insert_breaks :\"%s\"\n\0" as *const u8 as *const libc::c_char,
            out1.as_mut_ptr(),
        );
        printf(
            b"th_brk_wc_insert_breaks:\"%s\"\n\0" as *const u8 as *const libc::c_char,
            out2.as_mut_ptr(),
        );
        printf(b"Test th_brk_wc_insert_breaks() failed...\n\0" as *const u8 as *const libc::c_char);
        exit(-(1 as libc::c_int));
    }
    th_brk_delete(brk);
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
