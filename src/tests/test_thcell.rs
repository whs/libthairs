use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn th_next_cell(
        s: *const thchar_t,
        len: size_t,
        cell: *mut thcell_t,
        is_decomp_am: libc::c_int,
    ) -> size_t;
    fn th_prev_cell(
        s: *const thchar_t,
        pos: size_t,
        cell: *mut thcell_t,
        is_decomp_am: libc::c_int,
    ) -> size_t;
    fn th_make_cells(
        s: *const thchar_t,
        len: size_t,
        cells: *mut thcell_t,
        ncells: *mut size_t,
        is_decomp_am: libc::c_int,
    ) -> size_t;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
pub const TIS_KO_KAI: libc::c_int = 0xa1 as libc::c_int;
pub const TIS_NO_NU: libc::c_int = 0xb9 as libc::c_int;
pub const TIS_SARA_AM: libc::c_int = 0xd3 as libc::c_int;
pub const TIS_SARA_I: libc::c_int = 0xd4 as libc::c_int;
pub const TIS_SARA_II: libc::c_int = 0xd5 as libc::c_int;
pub const TIS_SARA_UEE: libc::c_int = 0xd7 as libc::c_int;
pub const TIS_SARA_U: libc::c_int = 0xd8 as libc::c_int;
pub const TIS_SARA_E: libc::c_int = 0xe0 as libc::c_int;
pub const TIS_MAITAIKHU: libc::c_int = 0xe7 as libc::c_int;
pub const TIS_MAI_EK: libc::c_int = 0xe8 as libc::c_int;
pub const TIS_MAI_THO: libc::c_int = 0xe9 as libc::c_int;
pub const TIS_THANTHAKHAT: libc::c_int = 0xec as libc::c_int;
pub const TIS_NIKHAHIT: libc::c_int = 0xed as libc::c_int;
static mut test_msg: [thchar_t; 44] = unsafe {
    *::core::mem::transmute::<
        &[u8; 44],
        &[thchar_t; 44],
    >(
        b"\xB9\xE9\xD3\xB9\xD3\xD3\xA1\xD5\xE8\xA1\xE8\xD5\xA1\xD8\xE8\xA1\xE8\xD8\xA1\xD8\xEC\xA1\xEC\xD8\xA1\xD4\xEC\xA1\xEC\xD4\xE0\xD4\xE0\xD3\xE0\xE9\xB9\xA1\xD4\xED\xA1\xD7\xE7\0",
    )
};
static mut test_ans_nodecomp_am: [thcell_t; 27] = [
    {
        let mut init = thcell_t {
            base: TIS_NO_NU as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: TIS_MAI_THO as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_SARA_AM as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_NO_NU as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_SARA_AM as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_SARA_AM as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: TIS_SARA_II as thchar_t,
            top: TIS_MAI_EK as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: TIS_MAI_EK as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: TIS_SARA_II as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: TIS_SARA_U as thchar_t,
            top: TIS_MAI_EK as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: TIS_MAI_EK as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: TIS_SARA_U as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: TIS_SARA_U as thchar_t,
            top: TIS_THANTHAKHAT as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: TIS_THANTHAKHAT as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: TIS_SARA_U as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: TIS_SARA_I as thchar_t,
            top: TIS_THANTHAKHAT as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: TIS_THANTHAKHAT as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: TIS_SARA_I as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_SARA_E as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: TIS_SARA_I as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_SARA_E as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_SARA_AM as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_SARA_E as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: TIS_MAI_THO as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_NO_NU as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: TIS_SARA_I as thchar_t,
            top: TIS_NIKHAHIT as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: TIS_SARA_UEE as thchar_t,
            top: TIS_MAITAIKHU as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
static mut test_ans_decomp_am: [thcell_t; 25] = [
    {
        let mut init = thcell_t {
            base: TIS_NO_NU as thchar_t,
            hilo: TIS_SARA_AM as thchar_t,
            top: TIS_MAI_THO as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_NO_NU as thchar_t,
            hilo: TIS_SARA_AM as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: TIS_SARA_AM as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: TIS_SARA_II as thchar_t,
            top: TIS_MAI_EK as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: TIS_MAI_EK as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: TIS_SARA_II as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: TIS_SARA_U as thchar_t,
            top: TIS_MAI_EK as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: TIS_MAI_EK as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: TIS_SARA_U as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: TIS_SARA_U as thchar_t,
            top: TIS_THANTHAKHAT as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: TIS_THANTHAKHAT as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: TIS_SARA_U as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: TIS_SARA_I as thchar_t,
            top: TIS_THANTHAKHAT as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: TIS_THANTHAKHAT as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: TIS_SARA_I as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_SARA_E as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: TIS_SARA_I as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_SARA_E as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: TIS_SARA_AM as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_SARA_E as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: TIS_MAI_THO as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_NO_NU as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: TIS_SARA_I as thchar_t,
            top: TIS_NIKHAHIT as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: TIS_KO_KAI as thchar_t,
            hilo: TIS_SARA_UEE as thchar_t,
            top: TIS_MAITAIKHU as thchar_t,
        };
        init
    },
    {
        let mut init = thcell_t {
            base: 0 as libc::c_int as thchar_t,
            hilo: 0 as libc::c_int as thchar_t,
            top: 0 as libc::c_int as thchar_t,
        };
        init
    },
];
#[no_mangle]
pub unsafe extern "C" fn test_th_next_cell() -> libc::c_int {
    let mut err_no: libc::c_int = 0 as libc::c_int;
    let mut s: *const thchar_t = 0 as *const thchar_t;
    let mut len: libc::c_int = 0;
    let mut pCell: *const thcell_t = 0 as *const thcell_t;
    fprintf(
        stderr,
        b"testing th_next_cell() without decomposing SARA AM...\n\0" as *const u8
            as *const libc::c_char,
    );
    pCell = test_ans_nodecomp_am.as_ptr();
    s = test_msg.as_ptr();
    len = strlen(test_msg.as_ptr() as *const libc::c_char) as libc::c_int;
    while *s != 0 {
        let mut aCell: thcell_t = thcell_t {
            base: 0,
            hilo: 0,
            top: 0,
        };
        let mut nChars: size_t = 0;
        nChars = th_next_cell(s, len as size_t, &mut aCell, 0 as libc::c_int);
        s = s.offset(nChars as isize);
        len = (len as size_t).wrapping_sub(nChars) as libc::c_int as libc::c_int;
        if aCell.base as libc::c_int != (*pCell).base as libc::c_int
            || aCell.hilo as libc::c_int != (*pCell).hilo as libc::c_int
            || aCell.top as libc::c_int != (*pCell).top as libc::c_int
        {
            fprintf(
                stderr,
                b"(%c,%c,%c) != (%c,%c,%c)\n\0" as *const u8 as *const libc::c_char,
                aCell.base as libc::c_int,
                aCell.hilo as libc::c_int,
                aCell.top as libc::c_int,
                (*pCell).base as libc::c_int,
                (*pCell).hilo as libc::c_int,
                (*pCell).top as libc::c_int,
            );
            err_no += 1;
            err_no;
        }
        if (*pCell).base as libc::c_int != 0
            || (*pCell).hilo as libc::c_int != 0
            || (*pCell).top as libc::c_int != 0
        {
            pCell = pCell.offset(1);
            pCell;
        }
    }
    fprintf(
        stderr,
        b"testing th_next_cell() decomposing SARA AM...\n\0" as *const u8 as *const libc::c_char,
    );
    s = test_msg.as_ptr();
    len = strlen(test_msg.as_ptr() as *const libc::c_char) as libc::c_int;
    pCell = test_ans_decomp_am.as_ptr();
    while *s != 0 {
        let mut aCell_0: thcell_t = thcell_t {
            base: 0,
            hilo: 0,
            top: 0,
        };
        let mut nChars_0: size_t = 0;
        nChars_0 = th_next_cell(s, len as size_t, &mut aCell_0, 1 as libc::c_int);
        s = s.offset(nChars_0 as isize);
        len = (len as size_t).wrapping_sub(nChars_0) as libc::c_int as libc::c_int;
        if aCell_0.base as libc::c_int != (*pCell).base as libc::c_int
            || aCell_0.hilo as libc::c_int != (*pCell).hilo as libc::c_int
            || aCell_0.top as libc::c_int != (*pCell).top as libc::c_int
        {
            fprintf(
                stderr,
                b"(%c,%c,%c) != (%c,%c,%c)\n\0" as *const u8 as *const libc::c_char,
                aCell_0.base as libc::c_int,
                aCell_0.hilo as libc::c_int,
                aCell_0.top as libc::c_int,
                (*pCell).base as libc::c_int,
                (*pCell).hilo as libc::c_int,
                (*pCell).top as libc::c_int,
            );
            err_no += 1;
            err_no;
        }
        if (*pCell).base as libc::c_int != 0
            || (*pCell).hilo as libc::c_int != 0
            || (*pCell).top as libc::c_int != 0
        {
            pCell = pCell.offset(1);
            pCell;
        }
    }
    return err_no;
}
#[no_mangle]
pub unsafe extern "C" fn test_th_prev_cell() -> libc::c_int {
    let mut err_no: libc::c_int = 0 as libc::c_int;
    let mut s: *const thchar_t = 0 as *const thchar_t;
    let mut pos: libc::c_int = 0;
    let mut pCell: *const thcell_t = 0 as *const thcell_t;
    fprintf(
        stderr,
        b"testing th_prev_cell() without decomposing SARA AM...\n\0" as *const u8
            as *const libc::c_char,
    );
    pCell = test_ans_nodecomp_am
        .as_ptr()
        .offset(
            (::core::mem::size_of::<[thcell_t; 27]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<thcell_t>() as libc::c_ulong)
                as isize,
        )
        .offset(-(2 as libc::c_int as isize));
    s = test_msg.as_ptr();
    pos = strlen(test_msg.as_ptr() as *const libc::c_char) as libc::c_int;
    while pos > 0 as libc::c_int {
        let mut aCell: thcell_t = thcell_t {
            base: 0,
            hilo: 0,
            top: 0,
        };
        pos = (pos as size_t).wrapping_sub(th_prev_cell(
            s,
            pos as size_t,
            &mut aCell,
            0 as libc::c_int,
        )) as libc::c_int as libc::c_int;
        if aCell.base as libc::c_int != (*pCell).base as libc::c_int
            || aCell.hilo as libc::c_int != (*pCell).hilo as libc::c_int
            || aCell.top as libc::c_int != (*pCell).top as libc::c_int
        {
            fprintf(
                stderr,
                b"(%c,%c,%c) != (%c,%c,%c)\n\0" as *const u8 as *const libc::c_char,
                aCell.base as libc::c_int,
                aCell.hilo as libc::c_int,
                aCell.top as libc::c_int,
                (*pCell).base as libc::c_int,
                (*pCell).hilo as libc::c_int,
                (*pCell).top as libc::c_int,
            );
            err_no += 1;
            err_no;
        }
        if pCell > test_ans_nodecomp_am.as_ptr() {
            pCell = pCell.offset(-1);
            pCell;
        }
    }
    fprintf(
        stderr,
        b"testing th_prev_cell() decomposing SARA AM...\n\0" as *const u8 as *const libc::c_char,
    );
    pCell = test_ans_decomp_am
        .as_ptr()
        .offset(
            (::core::mem::size_of::<[thcell_t; 25]>() as libc::c_ulong)
                .wrapping_div(::core::mem::size_of::<thcell_t>() as libc::c_ulong)
                as isize,
        )
        .offset(-(2 as libc::c_int as isize));
    s = test_msg.as_ptr();
    pos = strlen(test_msg.as_ptr() as *const libc::c_char) as libc::c_int;
    while pos > 0 as libc::c_int {
        let mut aCell_0: thcell_t = thcell_t {
            base: 0,
            hilo: 0,
            top: 0,
        };
        pos = (pos as size_t).wrapping_sub(th_prev_cell(
            s,
            pos as size_t,
            &mut aCell_0,
            1 as libc::c_int,
        )) as libc::c_int as libc::c_int;
        if aCell_0.base as libc::c_int != (*pCell).base as libc::c_int
            || aCell_0.hilo as libc::c_int != (*pCell).hilo as libc::c_int
            || aCell_0.top as libc::c_int != (*pCell).top as libc::c_int
        {
            fprintf(
                stderr,
                b"(%c,%c,%c) != (%c,%c,%c)\n\0" as *const u8 as *const libc::c_char,
                aCell_0.base as libc::c_int,
                aCell_0.hilo as libc::c_int,
                aCell_0.top as libc::c_int,
                (*pCell).base as libc::c_int,
                (*pCell).hilo as libc::c_int,
                (*pCell).top as libc::c_int,
            );
            err_no += 1;
            err_no;
        }
        if pCell > test_ans_decomp_am.as_ptr() {
            pCell = pCell.offset(-1);
            pCell;
        }
    }
    return err_no;
}
pub const TESTCELLS: libc::c_int = 10 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn test_th_make_cells() -> libc::c_int {
    let mut err_no: libc::c_int = 0 as libc::c_int;
    let mut cells: *mut thcell_t = 0 as *mut thcell_t;
    let mut s: *const thchar_t = 0 as *const thchar_t;
    let mut len: libc::c_int = 0;
    let mut pCell: *const thcell_t = 0 as *const thcell_t;
    cells = malloc(
        (TESTCELLS as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<thcell_t>() as libc::c_ulong),
    ) as *mut thcell_t;
    fprintf(
        stderr,
        b"testing th_make_cells() without decomposing SARA AM...\n\0" as *const u8
            as *const libc::c_char,
    );
    pCell = test_ans_nodecomp_am.as_ptr();
    s = test_msg.as_ptr();
    len = strlen(test_msg.as_ptr() as *const libc::c_char) as libc::c_int;
    while *s != 0 {
        let mut nCells: size_t = TESTCELLS as size_t;
        let mut i: size_t = 0;
        let mut nChars: size_t =
            th_make_cells(s, len as size_t, cells, &mut nCells, 0 as libc::c_int);
        s = s.offset(nChars as isize);
        len = (len as size_t).wrapping_sub(nChars) as libc::c_int as libc::c_int;
        i = 0 as libc::c_int as size_t;
        while i < nCells {
            if (*cells.offset(i as isize)).base as libc::c_int != (*pCell).base as libc::c_int
                || (*cells.offset(i as isize)).hilo as libc::c_int != (*pCell).hilo as libc::c_int
                || (*cells.offset(i as isize)).top as libc::c_int != (*pCell).top as libc::c_int
            {
                fprintf(
                    stderr,
                    b"(%c,%c,%c) != (%c,%c,%c)\n\0" as *const u8 as *const libc::c_char,
                    (*cells.offset(i as isize)).base as libc::c_int,
                    (*cells.offset(i as isize)).hilo as libc::c_int,
                    (*cells.offset(i as isize)).top as libc::c_int,
                    (*pCell).base as libc::c_int,
                    (*pCell).hilo as libc::c_int,
                    (*pCell).top as libc::c_int,
                );
                err_no += 1;
                err_no;
            }
            if (*pCell).base as libc::c_int != 0
                || (*pCell).hilo as libc::c_int != 0
                || (*pCell).top as libc::c_int != 0
            {
                pCell = pCell.offset(1);
                pCell;
            }
            i = i.wrapping_add(1);
            i;
        }
    }
    fprintf(
        stderr,
        b"testing th_make_cells() decomposing SARA AM...\n\0" as *const u8 as *const libc::c_char,
    );
    pCell = test_ans_decomp_am.as_ptr();
    s = test_msg.as_ptr();
    len = strlen(test_msg.as_ptr() as *const libc::c_char) as libc::c_int;
    while *s != 0 {
        let mut nCells_0: size_t = TESTCELLS as size_t;
        let mut i_0: size_t = 0;
        let mut nChars_0: size_t =
            th_make_cells(s, len as size_t, cells, &mut nCells_0, 1 as libc::c_int);
        s = s.offset(nChars_0 as isize);
        len = (len as size_t).wrapping_sub(nChars_0) as libc::c_int as libc::c_int;
        i_0 = 0 as libc::c_int as size_t;
        while i_0 < nCells_0 {
            if (*cells.offset(i_0 as isize)).base as libc::c_int != (*pCell).base as libc::c_int
                || (*cells.offset(i_0 as isize)).hilo as libc::c_int != (*pCell).hilo as libc::c_int
                || (*cells.offset(i_0 as isize)).top as libc::c_int != (*pCell).top as libc::c_int
            {
                fprintf(
                    stderr,
                    b"(%c,%c,%c) != (%c,%c,%c)\n\0" as *const u8 as *const libc::c_char,
                    (*cells.offset(i_0 as isize)).base as libc::c_int,
                    (*cells.offset(i_0 as isize)).hilo as libc::c_int,
                    (*cells.offset(i_0 as isize)).top as libc::c_int,
                    (*pCell).base as libc::c_int,
                    (*pCell).hilo as libc::c_int,
                    (*pCell).top as libc::c_int,
                );
                err_no += 1;
                err_no;
            }
            if (*pCell).base as libc::c_int != 0
                || (*pCell).hilo as libc::c_int != 0
                || (*pCell).top as libc::c_int != 0
            {
                pCell = pCell.offset(1);
                pCell;
            }
            i_0 = i_0.wrapping_add(1);
            i_0;
        }
    }
    free(cells as *mut libc::c_void);
    return err_no;
}
unsafe fn main_0() -> libc::c_int {
    let mut err_no: libc::c_int = 0 as libc::c_int;
    err_no += test_th_next_cell();
    err_no += test_th_prev_cell();
    err_no += test_th_make_cells();
    return err_no;
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
