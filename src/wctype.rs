//! Thai wide-char character classifications

use crate::wchar::uni2tis;
use crate::{thwchar_t, ThaiCharacter};

impl ThaiCharacter for thwchar_t {
    fn is_tis(&self) -> bool {
        uni2tis(*self).is_tis()
    }

    fn is_thai(&self) -> bool {
        uni2tis(*self).is_thai()
    }

    fn is_eng(&self) -> bool {
        uni2tis(*self).is_eng()
    }

    fn is_th_cons(&self) -> bool {
        uni2tis(*self).is_th_cons()
    }

    fn is_th_vowel(&self) -> bool {
        uni2tis(*self).is_th_vowel()
    }

    fn is_th_tone(&self) -> bool {
        uni2tis(*self).is_th_tone()
    }

    fn is_th_diac(&self) -> bool {
        uni2tis(*self).is_th_diac()
    }

    fn is_th_digit(&self) -> bool {
        uni2tis(*self).is_th_digit()
    }

    fn is_th_punct(&self) -> bool {
        uni2tis(*self).is_th_punct()
    }

    fn is_tailless_cons(&self) -> bool {
        uni2tis(*self).is_tailless_cons()
    }

    fn is_overshoot_cons(&self) -> bool {
        uni2tis(*self).is_overshoot_cons()
    }

    fn is_undershoot_cons(&self) -> bool {
        uni2tis(*self).is_undershoot_cons()
    }

    fn is_undersplit_cons(&self) -> bool {
        uni2tis(*self).is_undersplit_cons()
    }

    fn is_leading_vowel(&self) -> bool {
        uni2tis(*self).is_leading_vowel()
    }

    fn is_following_vowel(&self) -> bool {
        uni2tis(*self).is_following_vowel()
    }

    fn is_upper_vowel(&self) -> bool {
        uni2tis(*self).is_upper_vowel()
    }

    fn is_below_vowel(&self) -> bool {
        uni2tis(*self).is_below_vowel()
    }

    fn chlevel(&self) -> i8 {
        uni2tis(*self).chlevel()
    }

    fn is_combining_char(&self) -> bool {
        uni2tis(*self).is_combining_char()
    }
}

/// Is the wide character convertible to a valid TIS-620 code?
pub const fn is_tis(wc: thwchar_t) -> bool {
    crate::ctype::is_tis(uni2tis(wc))
}

/// Is the wide character a Thai character?
pub const fn is_thai(wc: thwchar_t) -> bool {
    crate::ctype::is_thai(uni2tis(wc))
}

/// Is the wide character an English character?
pub const fn is_eng(wc: thwchar_t) -> bool {
    crate::ctype::is_eng(uni2tis(wc))
}

/// Is the wide character a Thai consonant?
pub const fn is_th_cons(wc: thwchar_t) -> bool {
    crate::ctype::is_th_cons(uni2tis(wc))
}

/// Is the wide character a Thai vowel?
pub const fn is_th_vowel(wc: thwchar_t) -> bool {
    crate::ctype::is_th_vowel(uni2tis(wc))
}

/// Is the wide character a Thai tone mark?
pub const fn is_th_tone(wc: thwchar_t) -> bool {
    crate::ctype::is_th_tone(uni2tis(wc))
}

/// Is the wide character a Thai diacritic?
pub const fn is_th_diac(wc: thwchar_t) -> bool {
    crate::ctype::is_th_diac(uni2tis(wc))
}

/// Is the character a Thai digit?
pub const fn is_th_digit(wc: thwchar_t) -> bool {
    crate::ctype::is_th_digit(uni2tis(wc))
}

/// Is the character a Thai punctuation?
pub const fn is_th_punct(wc: thwchar_t) -> bool {
    crate::ctype::is_th_punct(uni2tis(wc))
}

/// Is the wide character a Thai consonant that fits the x-height?
pub const fn is_tailless_cons(wc: thwchar_t) -> bool {
    crate::ctype::is_tailless_cons(uni2tis(wc))
}

/// Is the wide character a Thai consonant with stem above ascender?
pub const fn is_overshoot_cons(wc: thwchar_t) -> bool {
    crate::ctype::is_overshoot_cons(uni2tis(wc))
}

/// Is the wide character a Thai consonant with stem below baseline?
pub const fn is_undershoot_cons(wc: thwchar_t) -> bool {
    crate::ctype::is_undershoot_cons(uni2tis(wc))
}

/// Is the wide character a Thai consonant with split part below baseline?
pub const fn is_undersplit_cons(wc: thwchar_t) -> bool {
    crate::ctype::is_undersplit_cons(uni2tis(wc))
}

/// Is the wide character a Thai leading vowel?
pub const fn is_leading_vowel(wc: thwchar_t) -> bool {
    crate::ctype::is_leading_vowel(uni2tis(wc))
}

/// Is the wide character a Thai following vowel?
pub const fn is_following_vowel(wc: thwchar_t) -> bool {
    crate::ctype::is_following_vowel(uni2tis(wc))
}

/// Is the wide character a Thai upper vowel?
pub const fn is_upper_vowel(wc: thwchar_t) -> bool {
    crate::ctype::is_upper_vowel(uni2tis(wc))
}

/// Is the wide character a Thai below vowel?
pub const fn is_below_vowel(wc: thwchar_t) -> bool {
    crate::ctype::is_below_vowel(uni2tis(wc))
}

/// Position for rendering
pub const fn chlevel(wc: thwchar_t) -> i8 {
    crate::ctype::chlevel(uni2tis(wc))
}

#[cfg(feature = "cffi")]
mod cffi {
    use super::*;

    /// Is the wide character convertible to a valid TIS-620 code?
    #[no_mangle]
    pub extern "C" fn th_wcistis(wc: thwchar_t) -> bool {
        is_tis(wc)
    }

    /// Is the wide character a Thai character?
    #[no_mangle]
    pub extern "C" fn th_wcisthai(wc: thwchar_t) -> bool {
        is_thai(wc)
    }

    /// Is the wide character an English character?
    #[no_mangle]
    pub extern "C" fn th_wciseng(wc: thwchar_t) -> bool {
        is_eng(wc)
    }

    /// Is the wide character a Thai consonant?
    #[no_mangle]
    pub extern "C" fn th_wcisthcons(wc: thwchar_t) -> bool {
        is_th_cons(wc)
    }

    /// Is the wide character a Thai vowel?
    #[no_mangle]
    pub extern "C" fn th_wcisthvowel(wc: thwchar_t) -> bool {
        is_th_vowel(wc)
    }

    /// Is the wide character a Thai tone mark?
    #[no_mangle]
    pub extern "C" fn th_wcisthtone(wc: thwchar_t) -> bool {
        is_th_tone(wc)
    }

    /// Is the wide character a Thai diacritic?
    #[no_mangle]
    pub extern "C" fn th_wcisthdiac(wc: thwchar_t) -> bool {
        is_th_diac(wc)
    }

    /// Is the character a Thai digit?
    #[no_mangle]
    pub extern "C" fn th_wcisthdigit(wc: thwchar_t) -> bool {
        is_th_digit(wc)
    }

    /// Is the character a Thai punctuation?
    #[no_mangle]
    pub extern "C" fn th_wcisthpunct(wc: thwchar_t) -> bool {
        is_th_punct(wc)
    }

    /// Is the wide character a Thai consonant that fits the x-height?
    #[no_mangle]
    pub extern "C" fn th_wcistaillesscons(wc: thwchar_t) -> bool {
        is_tailless_cons(wc)
    }

    /// Is the wide character a Thai consonant with stem above ascender?
    #[no_mangle]
    pub extern "C" fn th_wcisovershootcons(wc: thwchar_t) -> bool {
        is_overshoot_cons(wc)
    }

    /// Is the wide character a Thai consonant with stem below baseline?
    #[no_mangle]
    pub extern "C" fn th_wcisundershootcons(wc: thwchar_t) -> bool {
        is_undershoot_cons(wc)
    }

    /// Is the wide character a Thai consonant with split part below baseline?
    #[no_mangle]
    pub extern "C" fn th_wcisundersplitcons(wc: thwchar_t) -> bool {
        is_undersplit_cons(wc)
    }

    /// Is the wide character a Thai leading vowel?
    #[no_mangle]
    pub extern "C" fn th_wcisldvowel(wc: thwchar_t) -> bool {
        is_leading_vowel(wc)
    }

    /// Is the wide character a Thai following vowel?
    #[no_mangle]
    pub extern "C" fn th_wcisflvowel(wc: thwchar_t) -> bool {
        is_following_vowel(wc)
    }

    /// Is the wide character a Thai upper vowel?
    #[no_mangle]
    pub extern "C" fn th_wcisupvowel(wc: thwchar_t) -> bool {
        is_upper_vowel(wc)
    }

    /// Is the wide character a Thai below vowel?
    #[no_mangle]
    pub extern "C" fn th_wcisblvowel(wc: thwchar_t) -> bool {
        is_below_vowel(wc)
    }

    /// Position for rendering
    #[no_mangle]
    pub extern "C" fn th_wcchlevel(wc: thwchar_t) -> libc::c_int {
        chlevel(wc) as libc::c_int
    }
}
