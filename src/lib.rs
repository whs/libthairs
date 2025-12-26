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
pub use ctype::thchar_t;
pub use ctype::ThaiCharacter;
pub use ctype::THCHAR_ERR;

pub mod wtt;
// pub mod thinp {
//     //! Thai string input sequence filtering
//     pub mod thinp;
// } // mod thinp
// pub mod thrend {
//     //! Thai string rendering
//     pub mod thrend;
// } // mod thrend
pub mod str;
// pub mod thwbrk {
//     //! Thai wide-char word segmentation
//     pub mod thwbrk;
// } // mod thwbrk
pub mod wchar;
pub use wchar::thwchar_t;
pub use wchar::uni2rust;
pub use wchar::THWCHAR_ERR;

pub mod wctype;
// pub mod wstr;
