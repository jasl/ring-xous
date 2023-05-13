#![allow(warnings)]

pub type size_t = core::ffi::c_uint;
pub type __uint8_t = core::ffi::c_uchar;
pub type uint8_t = __uint8_t;
#[no_mangle]
pub unsafe extern "C" fn GFp_memcmp(
    mut a: *const uint8_t,
    mut b: *const uint8_t,
    mut len: size_t,
) -> core::ffi::c_int {
    let mut x: uint8_t = 0 as core::ffi::c_int as uint8_t;
    let mut i: size_t = 0 as core::ffi::c_int as size_t;
    while i < len {
        x = (x as core::ffi::c_int
            | *a.offset(i as isize) as core::ffi::c_int ^ *b.offset(i as isize) as core::ffi::c_int)
            as uint8_t;
        i = i.wrapping_add(1);
    }
    return x as core::ffi::c_int;
}
