#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/Jolt.rs"));

#[link(name = "jolt-wrapper")]
extern "C" {
    pub fn register_types();
}
