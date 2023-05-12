#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub type size_t = core::ffi::c_uint;
pub type __uint8_t = core::ffi::c_uchar;
pub type uint8_t = __uint8_t;
#[no_mangle]
pub unsafe extern "C" fn GFp_memcmp(
    a: *const uint8_t,
    b: *const uint8_t,
    len: size_t,
) -> core::ffi::c_int {
    let mut x: uint8_t = 0 as core::ffi::c_int as uint8_t;
    let mut i: size_t = 0 as core::ffi::c_int as size_t;
    while i < len {
        x = (x as core::ffi::c_int
            | *a.offset(i as isize) as core::ffi::c_int ^ *b.offset(i as isize) as core::ffi::c_int)
            as uint8_t;
        i = i.wrapping_add(1);
    }
    x as core::ffi::c_int
}
