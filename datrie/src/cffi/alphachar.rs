use crate::AlphaChar;
use null_terminated::Nul;
use std::cmp::Ordering;

/// Alphabet string length
#[no_mangle]
pub extern "C" fn alpha_char_strlen(key: *const AlphaChar) -> i32 {
    unsafe { Nul::new_unchecked(key) }.len() as i32
}

/// Compare alphabet strings
///
/// Returns
///
/// * Negative if str1 < str2
/// * 0 if str1 == str2
/// * Positive if str1 > str2
#[no_mangle]
pub extern "C" fn alpha_char_strcmp(str1: *const AlphaChar, str2: *const AlphaChar) -> i32 {
    // Nul::cmp would do a bound check which is slower
    let mut str1pos = str1;
    let mut str2pos = str2;

    unsafe {
        while *str1pos != 0 && *str1pos == *str2pos {
            str1pos = str1pos.offset(1);
            str2pos = str2pos.offset(1);
        }

        match (*str1pos).cmp(&*str2pos) {
            Ordering::Less => -1,
            Ordering::Equal => 0,
            Ordering::Greater => 1,
        }
    }
}

#[cfg(test)]
mod test {
    use crate::cffi::alphachar::*;
    use crate::AlphaChar;

    #[test]
    fn test_alpha_char_strlen() {
        let ch: [AlphaChar; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 0];
        assert_eq!(alpha_char_strlen(ch.as_ptr()), 9);
    }

    #[test]
    fn test_alpha_char_strlen_empty() {
        let ch: [AlphaChar; 1] = [0];
        assert_eq!(alpha_char_strlen(ch.as_ptr()), 0);
    }

    #[test]
    fn test_alpha_char_strcmp() {
        let ch1: [AlphaChar; 3] = [1, 1, 0];
        let ch2: [AlphaChar; 3] = [1, 2, 0];
        assert_eq!(alpha_char_strcmp(ch1.as_ptr(), ch1.as_ptr()), 0);
        assert_eq!(alpha_char_strcmp(ch1.as_ptr(), ch2.as_ptr()), -1);
        assert_eq!(alpha_char_strcmp(ch2.as_ptr(), ch1.as_ptr()), 1);
    }

    #[test]
    fn test_alpha_char_strcmp_longer() {
        let ch1: [AlphaChar; 4] = [1, 1, 2, 0];
        let ch2: [AlphaChar; 3] = [1, 1, 0];
        assert_eq!(alpha_char_strcmp(ch1.as_ptr(), ch2.as_ptr()), 1);
        assert_eq!(alpha_char_strcmp(ch2.as_ptr(), ch1.as_ptr()), -1);
    }
}
