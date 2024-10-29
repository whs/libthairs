#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]

extern crate libc;
pub mod thbrk;
pub mod tis;
pub mod thcell {
    pub mod thcell;
} // mod thcell
pub mod thcoll {
    pub mod cweight;
    pub mod thcoll;
} // mod thcoll
pub mod thctype;
pub mod thinp {
    pub mod thinp;
} // mod thinp
pub mod thrend {
    pub mod thrend;
} // mod thrend
pub mod thstr {
    pub mod thstr;
} // mod thstr
pub mod thwbrk {
    pub mod thwbrk;
} // mod thwbrk
pub mod thwchar {
    pub mod thwchar;
} // mod thwchar
pub mod thwctype {
    pub mod thwctype;
} // mod thwctype
pub mod thwstr {
    pub mod thwstr;
} // mod thwstr
pub mod tests {
    pub mod test_thbrk;
    pub mod test_thcell;
    pub mod test_thctype;
    pub mod test_thinp;
    pub mod test_thrend;
    pub mod test_thstr;
    pub mod test_thwbrk;
    pub mod test_thwchar;
    pub mod thsort;
} // mod tests
