// this file originally from:
//   https://rust-lang.github.io/rust-bindgen/tutorial-4.html

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
