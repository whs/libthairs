//! Thai wide-char string manipulators

use crate::wchar::{tis2uni, tis2uni_str, uni2tis, uni2tis_str};
use crate::{rust2uni, thwchar_t, uni2rust, THCHAR_ERR};

/// Normalize character order and remove excessive characters
pub fn normalize(src: &[thwchar_t]) -> Vec<thwchar_t> {
    src.split_inclusive(|&v| uni2tis(v) == THCHAR_ERR)
        .flat_map(|segment| {
            // segment can be
            // - [THCHAR_ERR]
            // - [text, ... THCHAR_ERR]
            // - [text, ...]

            let mut segment = segment;
            let trail = *segment.last().unwrap();
            let has_trailing_err = uni2tis(trail) == THCHAR_ERR;
            if has_trailing_err {
                // pop the trail
                segment = &segment[..segment.len() - 1];
            }

            let mut normalized = tis2uni_str(&crate::str::normalize(&uni2tis_str(segment)));

            if has_trailing_err {
                normalized.push(trail)
            }

            normalized
        })
        .collect()
}

pub fn normalize_str(str: &str) -> String {
    uni2rust(&normalize(&rust2uni(str)))
}

mod cffi {
    use super::*;
    use crate::thwchar_t;
    use core::slice;
    use null_terminated::Nul;
    use std::ptr::NonNull;

    /// Normalize character order and remove excessive characters
    #[no_mangle]
    pub extern "C" fn th_wnormalize(
        mut wdest: NonNull<thwchar_t>,
        wsrc: *const thwchar_t,
        n: usize,
    ) -> usize {
        let wsrc = unsafe { Nul::new_unchecked(wsrc) };
        let wsrc_slice = unsafe { slice::from_raw_parts(wsrc.as_ptr(), wsrc.len()) };
        let mut wdest = unsafe { slice::from_raw_parts_mut(wdest.as_ptr(), n) };

        let out = normalize(wsrc_slice);
        let out_len = out.len().min(n - 1);
        wdest[..out_len].copy_from_slice(&out[..out_len]);
        wdest[out_len] = 0;

        out_len
    }
}

mod tests {
    use super::*;
    use crate::{rust2uni, uni2rust};
    use std::ptr::NonNull;

    #[cfg(feature = "cffi")]
    #[test]
    fn test_c_wnormalize() {
        let input = "คุุณปู่ซู่่ซ่่า en้้";
        let expected = "คุณปู่ซู่ซ่า en้";
        let mut buf = vec![0; input.len() + 1];

        let out_len = unsafe {
            cffi::th_wnormalize(
                NonNull::new_unchecked(buf.as_mut_ptr()),
                rust2uni(input).as_ptr(),
                buf.len(),
            )
        };

        assert_eq!(buf[out_len], 0);
        assert_eq!(uni2rust(&buf[..out_len]), expected);
    }

    #[test]
    fn test_wnormalize() {
        let input = "คุุณปู่ซู่่ซ่่า en้้";
        let expected = "คุณปู่ซู่ซ่า en้";

        assert_eq!(normalize_str(input), expected);
    }
}
