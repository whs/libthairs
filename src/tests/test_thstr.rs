use ::libc;
extern "C" {
    fn th_normalize(dest: *mut thchar_t, src: *const thchar_t, n: size_t) -> size_t;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type thchar_t = libc::c_uchar;
#[no_mangle]
pub unsafe extern "C" fn test_th_normalize() -> libc::c_int {
    let mut test_str: *const thchar_t =
        b"\xA4\xD8\xD8\xB3\xBB\xE8\xD9\xAB\xE8\xE8\xD9\xAB\xE8\xE8\xD2\0" as *const u8
            as *const libc::c_char as *const thchar_t;
    let mut ans_str: *const thchar_t = b"\xA4\xD8\xB3\xBB\xD9\xE8\xAB\xD9\xE8\xAB\xE8\xD2\0"
        as *const u8 as *const libc::c_char
        as *const thchar_t;
    let mut resBuff: [thchar_t; 50] = [0; 50];
    th_normalize(
        resBuff.as_mut_ptr(),
        test_str,
        ::core::mem::size_of::<[thchar_t; 50]>() as libc::c_ulong,
    );
    return if strcmp(
        resBuff.as_mut_ptr() as *const libc::c_char,
        ans_str as *const libc::c_char,
    ) == 0 as libc::c_int
    {
        0 as libc::c_int
    } else {
        1 as libc::c_int
    };
}
unsafe fn main_0() -> libc::c_int {
    return test_th_normalize();
}
pub fn main() {
    unsafe { ::std::process::exit(main_0() as i32) }
}
