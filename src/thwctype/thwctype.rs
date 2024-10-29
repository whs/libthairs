use ::libc;
extern "C" {
    static _th_ctype_tbl: [libc::c_ushort; 0];
    fn th_uni2tis(wc: thwchar_t) -> thchar_t;
    static _th_chlevel_tbl: [libc::c_int; 0];
}
pub type wchar_t = libc::c_int;
pub type thchar_t = libc::c_uchar;
pub type thwchar_t = wchar_t;
pub const _th_IStis: C2RustUnnamed = 1;
pub const _th_IScons: C2RustUnnamed = 2;
pub const _th_ISvowel: C2RustUnnamed = 16;
pub const _th_IStone: C2RustUnnamed = 128;
pub const _th_ISdiac: C2RustUnnamed = 256;
pub const _th_ISdigit: C2RustUnnamed = 512;
pub const _th_ISpunct: C2RustUnnamed = 1024;
pub const _th_CCtailless: C2RustUnnamed = 2;
pub const _th_CClassMsk: C2RustUnnamed = 14;
pub const _th_CCovershoot: C2RustUnnamed = 6;
pub const _th_CCundershoot: C2RustUnnamed = 10;
pub const _th_CCundersplit: C2RustUnnamed = 14;
pub const _th_VCldvowel: C2RustUnnamed = 48;
pub const _th_VClassMsk: C2RustUnnamed = 112;
pub const _th_VCflvowel: C2RustUnnamed = 16;
pub const _th_VCupvowel: C2RustUnnamed = 80;
pub const _th_VCblvowel: C2RustUnnamed = 112;
pub type C2RustUnnamed = libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn th_wcistis(mut wc: thwchar_t) -> libc::c_int {
    return *_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_IStis as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcisthai(mut wc: thwchar_t) -> libc::c_int {
    return (*_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_IStis as libc::c_int
        != 0
        && th_uni2tis(wc) as libc::c_int & 0x80 as libc::c_int != 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wciseng(mut wc: thwchar_t) -> libc::c_int {
    return (th_uni2tis(wc) as libc::c_int & 0x80 as libc::c_int == 0) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcisthcons(mut wc: thwchar_t) -> libc::c_int {
    return *_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_IScons as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcisthvowel(mut wc: thwchar_t) -> libc::c_int {
    return *_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_ISvowel as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcisthtone(mut wc: thwchar_t) -> libc::c_int {
    return *_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_IStone as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcisthdiac(mut wc: thwchar_t) -> libc::c_int {
    return *_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_ISdiac as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcisthdigit(mut wc: thwchar_t) -> libc::c_int {
    return *_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_ISdigit as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcisthpunct(mut wc: thwchar_t) -> libc::c_int {
    return *_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_ISpunct as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcistaillesscons(mut wc: thwchar_t) -> libc::c_int {
    return (*_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_CClassMsk as libc::c_int
        == _th_CCtailless as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcisovershootcons(mut wc: thwchar_t) -> libc::c_int {
    return (*_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_CClassMsk as libc::c_int
        == _th_CCovershoot as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcisundershootcons(mut wc: thwchar_t) -> libc::c_int {
    return (*_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_CClassMsk as libc::c_int
        == _th_CCundershoot as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcisundersplitcons(mut wc: thwchar_t) -> libc::c_int {
    return (*_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_CClassMsk as libc::c_int
        == _th_CCundersplit as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcisldvowel(mut wc: thwchar_t) -> libc::c_int {
    return (*_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_VClassMsk as libc::c_int
        == _th_VCldvowel as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcisflvowel(mut wc: thwchar_t) -> libc::c_int {
    return (*_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_VClassMsk as libc::c_int
        == _th_VCflvowel as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcisupvowel(mut wc: thwchar_t) -> libc::c_int {
    return (*_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_VClassMsk as libc::c_int
        == _th_VCupvowel as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcisblvowel(mut wc: thwchar_t) -> libc::c_int {
    return (*_th_ctype_tbl.as_ptr().offset(th_uni2tis(wc) as isize) as libc::c_int
        & _th_VClassMsk as libc::c_int
        == _th_VCblvowel as libc::c_int) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn th_wcchlevel(mut wc: thwchar_t) -> libc::c_int {
    return *_th_chlevel_tbl.as_ptr().offset(th_uni2tis(wc) as isize);
}
