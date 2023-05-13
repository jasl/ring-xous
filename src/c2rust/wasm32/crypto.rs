#![allow(warnings)]

pub type __uint32_t = core::ffi::c_uint;
pub type uint32_t = __uint32_t;
#[no_mangle]
pub static mut GFp_ia32cap_P: [uint32_t; 4] = [0 as core::ffi::c_int as uint32_t, 0, 0, 0];
