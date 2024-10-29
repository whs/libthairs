use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn th_isblvowel(c: thchar_t) -> libc::c_int;
    fn th_isupvowel(c: thchar_t) -> libc::c_int;
    fn th_isflvowel(c: thchar_t) -> libc::c_int;
    fn th_isldvowel(c: thchar_t) -> libc::c_int;
    fn th_isundersplitcons(c: thchar_t) -> libc::c_int;
    fn th_isundershootcons(c: thchar_t) -> libc::c_int;
    fn th_isovershootcons(c: thchar_t) -> libc::c_int;
    fn th_istaillesscons(c: thchar_t) -> libc::c_int;
    fn th_isthpunct(c: thchar_t) -> libc::c_int;
    fn th_isthdigit(c: thchar_t) -> libc::c_int;
    fn th_isthdiac(c: thchar_t) -> libc::c_int;
    fn th_isthtone(c: thchar_t) -> libc::c_int;
    fn th_isthvowel(c: thchar_t) -> libc::c_int;
    fn th_isthcons(c: thchar_t) -> libc::c_int;
    fn th_iseng(c: thchar_t) -> libc::c_int;
    fn th_isthai(c: thchar_t) -> libc::c_int;
    fn th_istis(c: thchar_t) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
}
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct char_range {
    pub begin: thchar_t,
    pub end: thchar_t,
}
#[no_mangle]
pub static mut tis_ranges: [char_range; 4] = [
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0x7f as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xa1 as libc::c_int as thchar_t,
            end: 0xda as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xdf as libc::c_int as thchar_t,
            end: 0xfb as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut thai_ranges: [char_range; 3] = [
    {
        let mut init = char_range {
            begin: 0xa1 as libc::c_int as thchar_t,
            end: 0xda as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xdf as libc::c_int as thchar_t,
            end: 0xfb as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut eng_ranges: [char_range; 2] = [
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0x7f as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut thcons_ranges: [char_range; 4] = [
    {
        let mut init = char_range {
            begin: 0xa1 as libc::c_int as thchar_t,
            end: 0xc3 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xc5 as libc::c_int as thchar_t,
            end: 0xc5 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xc7 as libc::c_int as thchar_t,
            end: 0xce as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut tlcons_ranges: [char_range; 9] = [
    {
        let mut init = char_range {
            begin: 0xa1 as libc::c_int as thchar_t,
            end: 0xac as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xb1 as libc::c_int as thchar_t,
            end: 0xba as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xbc as libc::c_int as thchar_t,
            end: 0xbc as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xbe as libc::c_int as thchar_t,
            end: 0xbe as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xc0 as libc::c_int as thchar_t,
            end: 0xc3 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xc5 as libc::c_int as thchar_t,
            end: 0xc5 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xc7 as libc::c_int as thchar_t,
            end: 0xcb as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xcd as libc::c_int as thchar_t,
            end: 0xce as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut oscons_ranges: [char_range; 5] = [
    {
        let mut init = char_range {
            begin: 0xbb as libc::c_int as thchar_t,
            end: 0xbb as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xbd as libc::c_int as thchar_t,
            end: 0xbd as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xbf as libc::c_int as thchar_t,
            end: 0xbf as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xcc as libc::c_int as thchar_t,
            end: 0xcc as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut uscons_ranges: [char_range; 2] = [
    {
        let mut init = char_range {
            begin: 0xae as libc::c_int as thchar_t,
            end: 0xaf as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut spcons_ranges: [char_range; 3] = [
    {
        let mut init = char_range {
            begin: 0xad as libc::c_int as thchar_t,
            end: 0xad as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xb0 as libc::c_int as thchar_t,
            end: 0xb0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut thvowel_ranges: [char_range; 5] = [
    {
        let mut init = char_range {
            begin: 0xc4 as libc::c_int as thchar_t,
            end: 0xc4 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xc6 as libc::c_int as thchar_t,
            end: 0xc6 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xd0 as libc::c_int as thchar_t,
            end: 0xd9 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xe0 as libc::c_int as thchar_t,
            end: 0xe5 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut ldvowel_ranges: [char_range; 2] = [
    {
        let mut init = char_range {
            begin: 0xe0 as libc::c_int as thchar_t,
            end: 0xe4 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut flvowel_ranges: [char_range; 6] = [
    {
        let mut init = char_range {
            begin: 0xc4 as libc::c_int as thchar_t,
            end: 0xc4 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xc6 as libc::c_int as thchar_t,
            end: 0xc6 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xd0 as libc::c_int as thchar_t,
            end: 0xd0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xd2 as libc::c_int as thchar_t,
            end: 0xd3 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xe5 as libc::c_int as thchar_t,
            end: 0xe5 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut upvowel_ranges: [char_range; 3] = [
    {
        let mut init = char_range {
            begin: 0xd1 as libc::c_int as thchar_t,
            end: 0xd1 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xd4 as libc::c_int as thchar_t,
            end: 0xd7 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut blvowel_ranges: [char_range; 2] = [
    {
        let mut init = char_range {
            begin: 0xd8 as libc::c_int as thchar_t,
            end: 0xd9 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut thtone_ranges: [char_range; 2] = [
    {
        let mut init = char_range {
            begin: 0xe8 as libc::c_int as thchar_t,
            end: 0xeb as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut thdiac_ranges: [char_range; 4] = [
    {
        let mut init = char_range {
            begin: 0xda as libc::c_int as thchar_t,
            end: 0xda as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xe7 as libc::c_int as thchar_t,
            end: 0xe7 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xec as libc::c_int as thchar_t,
            end: 0xee as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut thdigit_ranges: [char_range; 3] = [
    {
        let mut init = char_range {
            begin: 0x30 as libc::c_int as thchar_t,
            end: 0x39 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xf0 as libc::c_int as thchar_t,
            end: 0xf9 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub static mut thpunct_ranges: [char_range; 10] = [
    {
        let mut init = char_range {
            begin: 0x21 as libc::c_int as thchar_t,
            end: 0x2f as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0x3a as libc::c_int as thchar_t,
            end: 0x40 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0x5b as libc::c_int as thchar_t,
            end: 0x60 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0x7b as libc::c_int as thchar_t,
            end: 0x7e as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xcf as libc::c_int as thchar_t,
            end: 0xcf as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xdf as libc::c_int as thchar_t,
            end: 0xdf as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xe6 as libc::c_int as thchar_t,
            end: 0xe6 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xef as libc::c_int as thchar_t,
            end: 0xef as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0xfa as libc::c_int as thchar_t,
            end: 0xfb as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = char_range {
            begin: 0 as libc::c_int as thchar_t,
            end: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn test_bool_funcs(
    mut ranges: *const char_range,
    mut fn_0: Option<unsafe extern "C" fn(thchar_t) -> libc::c_int>,
) -> libc::c_int {
    let mut pRange: *const char_range = ranges;
    let mut err_code: libc::c_int = 0 as libc::c_int;
    let mut c: thchar_t = 0 as libc::c_int as thchar_t;
    while (*pRange).end != 0 {
        while (c as libc::c_int) < (*pRange).begin as libc::c_int {
            if (Some(fn_0.expect("non-null function pointer"))).expect("non-null function pointer")(
                c,
            ) != 0
            {
                fprintf(
                    stderr,
                    b"+%02x \0" as *const u8 as *const libc::c_char,
                    c as libc::c_int,
                );
                err_code = 1 as libc::c_int;
            }
            c = c.wrapping_add(1);
            c;
        }
        while c as libc::c_int <= (*pRange).end as libc::c_int {
            if (Some(fn_0.expect("non-null function pointer"))).expect("non-null function pointer")(
                c,
            ) == 0
            {
                fprintf(
                    stderr,
                    b"-%02x \0" as *const u8 as *const libc::c_char,
                    c as libc::c_int,
                );
                err_code = 1 as libc::c_int;
            }
            c = c.wrapping_add(1);
            c;
        }
        pRange = pRange.offset(1);
        pRange;
    }
    while (c as libc::c_int) < 0xff as libc::c_int {
        if (Some(fn_0.expect("non-null function pointer"))).expect("non-null function pointer")(c)
            != 0
        {
            fprintf(
                stderr,
                b"+%02x \0" as *const u8 as *const libc::c_char,
                c as libc::c_int,
            );
            err_code = 1 as libc::c_int;
        }
        c = c.wrapping_add(1);
        c;
    }
    if (Some(fn_0.expect("non-null function pointer"))).expect("non-null function pointer")(c) != 0
    {
        fprintf(
            stderr,
            b"+%02x \0" as *const u8 as *const libc::c_char,
            c as libc::c_int,
        );
        err_code = 1 as libc::c_int;
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    return err_code;
}
unsafe fn main_0() -> libc::c_int {
    let mut err_code: libc::c_int = 0 as libc::c_int;
    err_code += test_bool_funcs(
        tis_ranges.as_ptr(),
        Some(th_istis as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        thai_ranges.as_ptr(),
        Some(th_isthai as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        eng_ranges.as_ptr(),
        Some(th_iseng as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        thcons_ranges.as_ptr(),
        Some(th_isthcons as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        thvowel_ranges.as_ptr(),
        Some(th_isthvowel as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        thtone_ranges.as_ptr(),
        Some(th_isthtone as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        thdiac_ranges.as_ptr(),
        Some(th_isthdiac as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        thdigit_ranges.as_ptr(),
        Some(th_isthdigit as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        thpunct_ranges.as_ptr(),
        Some(th_isthpunct as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        tlcons_ranges.as_ptr(),
        Some(th_istaillesscons as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        oscons_ranges.as_ptr(),
        Some(th_isovershootcons as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        uscons_ranges.as_ptr(),
        Some(th_isundershootcons as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        spcons_ranges.as_ptr(),
        Some(th_isundersplitcons as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        ldvowel_ranges.as_ptr(),
        Some(th_isldvowel as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        flvowel_ranges.as_ptr(),
        Some(th_isflvowel as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        upvowel_ranges.as_ptr(),
        Some(th_isupvowel as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    err_code += test_bool_funcs(
        blvowel_ranges.as_ptr(),
        Some(th_isblvowel as unsafe extern "C" fn(thchar_t) -> libc::c_int),
    );
    return err_code;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
