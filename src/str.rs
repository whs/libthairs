//! Thai string manipulators

use crate::{thchar_t, ThaiCharacter};

/// Normalize character order and remove excessive characters
pub fn normalize(src: &[thchar_t]) -> Vec<thchar_t> {
    let mut out = Vec::with_capacity(src.len());

    let mut top: thchar_t = 0;
    let mut up: thchar_t = 0;
    let mut middle: thchar_t = 0;
    let mut low: thchar_t = 0;

    for ch in src {
        match ch.chlevel() {
            0 => {
                if middle != 0 {
                    out.push(middle);
                    if low != 0 {
                        out.push(low);
                    } else if up != 0 {
                        out.push(up);
                    }
                    if top != 0 {
                        out.push(top);
                    }
                }
                top = 0;
                up = 0;
                low = 0;
                middle = *ch;
            }
            -1 => {
                low = *ch;
            }
            1 => {
                if up != 0 && up.chlevel() == 3 {
                    top = up;
                }
                up = *ch;
            }
            2 => {
                top = *ch;
            }
            3 => {
                if up == 0 {
                    top = *ch;
                } else {
                    up = *ch;
                }
            }
            _ => unreachable!(),
        }
    }

    if middle != 0 {
        out.push(middle);
        if low != 0 {
            out.push(low);
        } else if up != 0 {
            out.push(up);
        }
        if top != 0 {
            out.push(top);
        }
    }

    out
}

#[cfg(feature = "cffi")]
mod cffi {
    use super::*;
    use crate::thchar_t;
    use null_terminated::Nul;
    use std::ptr::NonNull;
    use std::slice;

    /// Normalize character order and remove excessive characters
    #[no_mangle]
    pub extern "C" fn th_normalize(
        mut dest: NonNull<thchar_t>,
        src: *const thchar_t,
        n: usize,
    ) -> usize {
        let dest = unsafe { slice::from_raw_parts_mut(dest.as_mut(), n) };
        let src_nul = unsafe { Nul::new_unchecked(src) };
        let src_slice = unsafe { slice::from_raw_parts(src, src_nul.len()) };

        let out = normalize(src_slice);
        let copy_len = out.len().min(n - 1);
        dest[..copy_len].copy_from_slice(&out[..copy_len]);
        dest[copy_len] = 0;

        copy_len
    }
}

mod tests {
    use super::*;
    use std::ffi::CString;
    use std::ptr::NonNull;

    const test: [u8; 15] = [
        0xa4, 0xd8, 0xd8, 0xb3, 0xbb, 0xe8, 0xd9, 0xab, 0xe8, 0xe8, 0xd9, 0xab, 0xe8, 0xe8, 0xd2,
    ];
    const expected: [u8; 12] = [
        0xa4, 0xd8, 0xb3, 0xbb, 0xd9, 0xe8, 0xab, 0xd9, 0xe8, 0xab, 0xe8, 0xd2,
    ];

    #[cfg(feature = "cffi")]
    #[test]
    fn test_c_th_normalize() {
        let input = CString::new(test).unwrap();
        let mut buf = vec![0; 50];
        let len = unsafe {
            cffi::th_normalize(
                NonNull::new_unchecked(buf.as_mut_ptr()),
                input.as_ptr().cast(),
                buf.len(),
            )
        };
        assert_eq!(buf[..len], expected);
        assert_eq!(buf[len], 0);
    }

    #[test]
    fn test_th_normalize() {
        assert_eq!(normalize(&test), expected);
    }
}
