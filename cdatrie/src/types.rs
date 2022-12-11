use crate::binding::*;
use std::slice;

pub trait AlphaCharEx {
    fn len(&self) -> usize;
    fn as_slice(&self) -> &[AlphaChar];
}

impl AlphaCharEx for *const AlphaChar {
    fn len(&self) -> usize {
        unsafe { alpha_char_strlen(*self) as usize }
    }

    fn as_slice(&self) -> &[AlphaChar] {
        unsafe { slice::from_raw_parts(*self, self.len()) }
    }
}
