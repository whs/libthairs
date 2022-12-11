#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]

/// Package binding contains bindgen-generated code
/// This package is public but is not guaranteed to have any API stability

type Bool = bool;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
