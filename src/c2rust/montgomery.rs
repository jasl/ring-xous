#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(unused_mut)]
extern crate std;

use std::arch::asm;
extern "C" {
    fn __assert_fail(
        __assertion: *const std::os::raw::c_char,
        __file: *const std::os::raw::c_char,
        __line: std::os::raw::c_uint,
        __function: *const std::os::raw::c_char,
    ) -> !;
    fn GFp_limbs_mul_add_limb(r: *mut Limb, a: *const Limb, b: Limb, num_limbs: size_t) -> Limb;
    fn limbs_mul_add_limb(r: *mut Limb, a: *const Limb, b: Limb, num_limbs: size_t) -> Limb;
}
pub type size_t = std::os::raw::c_uint;
pub type __uint32_t = std::os::raw::c_uint;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type crypto_word = uint32_t;
pub type BN_ULONG = crypto_word;
pub type Limb = crypto_word;
pub type Carry = Limb;
pub type DoubleLimb = uint64_t;
#[inline]
unsafe extern "C" fn value_barrier_w(mut a: crypto_word) -> crypto_word {
    core::sync::atomic::compiler_fence(core::sync::atomic::Ordering::SeqCst);
    return a;
}
#[inline]
unsafe extern "C" fn constant_time_select_w(
    mut mask: crypto_word,
    mut a: crypto_word,
    mut b: crypto_word,
) -> crypto_word {
    return value_barrier_w(mask) & a | value_barrier_w(!mask) & b;
}
#[inline]
unsafe extern "C" fn limb_sbb(
    mut r: *mut Limb,
    mut a: Limb,
    mut b: Limb,
    mut borrow_in: Carry,
) -> Carry {
    let mut ret: Carry = 0;
    let mut x: DoubleLimb = (a as DoubleLimb)
        .wrapping_sub(b as u64)
        .wrapping_sub(borrow_in as u64);
    *r = x as Limb;
    ret = (x >> 32 as std::os::raw::c_uint & 1 as std::os::raw::c_int as u64) as Carry;
    return ret;
}
#[inline]
unsafe extern "C" fn limb_sub(mut r: *mut Limb, mut a: Limb, mut b: Limb) -> Carry {
    let mut ret: Carry = 0;
    let mut x: DoubleLimb = (a as DoubleLimb).wrapping_sub(b as u64);
    *r = x as Limb;
    ret = (x >> 32 as std::os::raw::c_uint & 1 as std::os::raw::c_int as u64) as Carry;
    return ret;
}
#[inline]
unsafe extern "C" fn limbs_sub(
    mut r: *mut Limb,
    mut a: *const Limb,
    mut b: *const Limb,
    mut num_limbs: size_t,
) -> Carry {
    if num_limbs >= 1 as std::os::raw::c_int as std::os::raw::c_uint {
    } else {
        __assert_fail(
            b"num_limbs >= 1\0" as *const u8 as *const std::os::raw::c_char,
            b"crypto/fipsmodule/bn/../../limbs/limbs.inl\0" as *const u8
                as *const std::os::raw::c_char,
            129 as std::os::raw::c_int as std::os::raw::c_uint,
            (*std::mem::transmute::<&[u8; 60], &[std::os::raw::c_char; 60]>(
                b"Carry limbs_sub(Limb *, const Limb *, const Limb *, size_t)\0",
            ))
            .as_ptr(),
        );
    }
    let mut borrow: Carry = limb_sub(
        &mut *r.offset(0 as std::os::raw::c_int as isize),
        *a.offset(0 as std::os::raw::c_int as isize),
        *b.offset(0 as std::os::raw::c_int as isize),
    );
    let mut i: size_t = 1 as std::os::raw::c_int as size_t;
    while i < num_limbs {
        borrow = limb_sbb(
            &mut *r.offset(i as isize),
            *a.offset(i as isize),
            *b.offset(i as isize),
            borrow,
        );
        i = i.wrapping_add(1);
    }
    return borrow;
}
#[no_mangle]
pub unsafe extern "C" fn bn_from_montgomery_in_place(
    mut r: *mut BN_ULONG,
    mut num_r: size_t,
    mut a: *mut BN_ULONG,
    mut num_a: size_t,
    mut n: *const BN_ULONG,
    mut num_n: size_t,
    mut n0_: *const BN_ULONG,
) -> std::os::raw::c_int {
    if num_n == 0 as std::os::raw::c_int as std::os::raw::c_uint
        || num_r != num_n
        || num_a != (2 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_mul(num_n)
    {
        return 0 as std::os::raw::c_int;
    }
    let mut n0: BN_ULONG = *n0_.offset(0 as std::os::raw::c_int as isize);
    let mut carry: BN_ULONG = 0 as std::os::raw::c_int as BN_ULONG;
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < num_n {
        let mut v: BN_ULONG = limbs_mul_add_limb(
            a.offset(i as isize),
            n,
            (*a.offset(i as isize)).wrapping_mul(n0),
            num_n,
        );
        v = (v as std::os::raw::c_uint)
            .wrapping_add(carry.wrapping_add(*a.offset(i.wrapping_add(num_n) as isize)))
            as BN_ULONG as BN_ULONG;
        carry |= (v != *a.offset(i.wrapping_add(num_n) as isize)) as std::os::raw::c_int
            as std::os::raw::c_uint;
        carry &= (v <= *a.offset(i.wrapping_add(num_n) as isize)) as std::os::raw::c_int
            as std::os::raw::c_uint;
        *a.offset(i.wrapping_add(num_n) as isize) = v;
        i = i.wrapping_add(1);
    }
    a = a.offset(num_n as isize);
    let mut v_0: BN_ULONG = (limbs_sub(r, a as *const Limb, n, num_n)).wrapping_sub(carry);
    v_0 = (0 as std::os::raw::c_uint).wrapping_sub(v_0);
    let mut i_0: size_t = 0 as std::os::raw::c_int as size_t;
    while i_0 < num_n {
        *r.offset(i_0 as isize) =
            constant_time_select_w(v_0, *a.offset(i_0 as isize), *r.offset(i_0 as isize));
        *a.offset(i_0 as isize) = 0 as std::os::raw::c_int as BN_ULONG;
        i_0 = i_0.wrapping_add(1);
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn GFp_bn_from_montgomery_in_place(
    mut r: *mut BN_ULONG,
    mut num_r: size_t,
    mut a: *mut BN_ULONG,
    mut num_a: size_t,
    mut n: *const BN_ULONG,
    mut num_n: size_t,
    mut n0_: *const BN_ULONG,
) -> std::os::raw::c_int {
    if num_n == 0 as std::os::raw::c_int as std::os::raw::c_uint
        || num_r != num_n
        || num_a != (2 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_mul(num_n)
    {
        return 0 as std::os::raw::c_int;
    }
    let mut n0: BN_ULONG = *n0_.offset(0 as std::os::raw::c_int as isize);
    let mut carry: BN_ULONG = 0 as std::os::raw::c_int as BN_ULONG;
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < num_n {
        let mut v: BN_ULONG = GFp_limbs_mul_add_limb(
            a.offset(i as isize),
            n,
            (*a.offset(i as isize)).wrapping_mul(n0),
            num_n,
        );
        v = (v as std::os::raw::c_uint)
            .wrapping_add(carry.wrapping_add(*a.offset(i.wrapping_add(num_n) as isize)))
            as BN_ULONG as BN_ULONG;
        carry |= (v != *a.offset(i.wrapping_add(num_n) as isize)) as std::os::raw::c_int
            as std::os::raw::c_uint;
        carry &= (v <= *a.offset(i.wrapping_add(num_n) as isize)) as std::os::raw::c_int
            as std::os::raw::c_uint;
        *a.offset(i.wrapping_add(num_n) as isize) = v;
        i = i.wrapping_add(1);
    }
    a = a.offset(num_n as isize);
    let mut v_0: BN_ULONG = (limbs_sub(r, a as *const Limb, n, num_n)).wrapping_sub(carry);
    v_0 = (0 as std::os::raw::c_uint).wrapping_sub(v_0);
    let mut i_0: size_t = 0 as std::os::raw::c_int as size_t;
    while i_0 < num_n {
        *r.offset(i_0 as isize) =
            constant_time_select_w(v_0, *a.offset(i_0 as isize), *r.offset(i_0 as isize));
        *a.offset(i_0 as isize) = 0 as std::os::raw::c_int as BN_ULONG;
        i_0 = i_0.wrapping_add(1);
    }
    return 1 as std::os::raw::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn bn_mul_mont(
    mut rp: *mut BN_ULONG,
    mut ap: *const BN_ULONG,
    mut bp: *const BN_ULONG,
    mut np: *const BN_ULONG,
    mut n0: *const BN_ULONG,
    mut num: size_t,
) {
    let vla = (2 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_mul(num) as usize;
    let mut tmp: std::vec::Vec<Limb> = std::vec::from_elem(0, vla);
    let mut i: size_t = 0 as std::os::raw::c_int as size_t;
    while i < num {
        *tmp.as_mut_ptr().offset(i as isize) = 0 as std::os::raw::c_int as Limb;
        i = i.wrapping_add(1);
    }
    let mut i_0: size_t = 0 as std::os::raw::c_int as size_t;
    while i_0 < num {
        *tmp.as_mut_ptr().offset(num.wrapping_add(i_0) as isize) = limbs_mul_add_limb(
            tmp.as_mut_ptr().offset(i_0 as isize),
            ap,
            *bp.offset(i_0 as isize),
            num,
        );
        i_0 = i_0.wrapping_add(1);
    }
    bn_from_montgomery_in_place(
        rp,
        num,
        tmp.as_mut_ptr(),
        (2 as std::os::raw::c_int as std::os::raw::c_uint).wrapping_mul(num),
        np,
        num,
        n0,
    );
}
