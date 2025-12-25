#![allow(dead_code)]
#![allow(mutable_transmutes)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_assignments)]
#![allow(unused_mut)]
#![feature(extern_types)]

//! libthairs is a port of [libthai](https://linux.thai.net/projects/libthai) to Rust.
//!
//! LibThai is a set of Thai language support routines aimed to ease developers’ tasks to incorporate Thai language support in their applications.

extern crate libc;

// pub mod thbrk;
pub mod tis;
// pub mod thcell {
//     //! Thai string cell custering
//     pub mod thcell;
// } // mod thcell
// pub mod thcoll {
//     //! Thai string collation
//     pub mod cweight;
//     pub mod thcoll;
// } // mod thcoll
pub mod ctype;
pub mod wtt;
// pub mod thinp {
//     //! Thai string input sequence filtering
//     pub mod thinp;
// } // mod thinp
// pub mod thrend {
//     //! Thai string rendering
//     pub mod thrend;
// } // mod thrend
// pub mod thstr {
//     //! Thai string manipulators
//     pub mod thstr;
// } // mod thstr
// pub mod thwbrk {
//     //! Thai wide-char word segmentation
//     pub mod thwbrk;
// } // mod thwbrk
// pub mod thwchar {
//     //! Wide char support for Thai
//     pub mod thwchar;
// } // mod thwchar
// pub mod thwctype {
//     //! Thai wide-char character classifications
//     pub mod thwctype;
// } // mod thwctype
// pub mod thwstr {
//     //! Thai wide-char string manipulators
//     pub mod thwstr;
// } // mod thwstr
