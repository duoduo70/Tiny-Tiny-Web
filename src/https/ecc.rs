/* ecc.rs
 * No Copyright 2024 Plasma
 * CC0 LICENSE
 */
#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut,
    clippy::all
)]
use std::ptr::addr_of_mut;

use crate::drop::random;
use crate::drop::time;

mod libc {
    pub type c_char = i8;
    pub type c_uchar = u8;
    pub type c_int = i32;
    pub type c_uint = u32;
    pub type c_long = i32;
    pub type c_ulong = u32;
    pub type c_ulonglong = u64;
}

pub type __uint8_t = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulonglong;
pub type uint8_t = __uint8_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EccPoint {
    pub x: [uint64_t; 4],
    pub y: [uint64_t; 4],
}
pub type uint = libc::c_uint;
pub type uint128_t = u128;
static mut curve_p: [uint64_t; 4] = [0xffffffffffffffff, 0xffffffff, 0, 0xffffffff00000001];
static mut curve_G: EccPoint = {
    let mut init = EccPoint {
        x: [
            0xf4a13945d898c296,
            0x77037d812deb33a0,
            0xf8bce6e563a440f2,
            0x6b17d1f2e12c4247,
        ],
        y: [
            0xcbb6406837bf51f5,
            0x2bce33576b315ece,
            0x8ee7eb4a7c0f9e16,
            0x4fe342e2fe1a7f9b,
        ],
    };
    init
};
static mut curve_n: [uint64_t; 4] = [
    0xf3b9cac2fc632551,
    0xbce6faada7179e84,
    0xffffffffffffffff,
    0xffffffff00000000,
];
/// FIXME: 更改种子，并且随机数内部状态结构体应该是全局的
/// 这个函数只是临时的
unsafe extern "C" fn getRandomNumber(p_vli: *mut uint64_t) -> libc::c_int {
    let mut bytes = [0; 32];
    random::random_init(time::Time::nsec().unwrap().into()).fill_bytes(&mut bytes);
    p_vli.copy_from(bytes.as_mut_ptr() as *const uint64_t, 4);
    1
}
unsafe extern "C" fn vli_clear(mut p_vli: *mut uint64_t) {
    let mut i: uint = 0;
    i = 0 as libc::c_int as uint;
    while i < (32 as libc::c_int / 8 as libc::c_int) as uint {
        *p_vli.offset(i as isize) = 0 as libc::c_int as uint64_t;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn vli_isZero(mut p_vli: *mut uint64_t) -> libc::c_int {
    let mut i: uint = 0;
    i = 0 as libc::c_int as uint;
    while i < (32 as libc::c_int / 8 as libc::c_int) as uint {
        if *p_vli.offset(i as isize) != 0 {
            return 0 as libc::c_int;
        }
        i = i.wrapping_add(1);
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn vli_testBit(mut p_vli: *mut uint64_t, mut p_bit: uint) -> uint64_t {
    return *p_vli.offset((p_bit / 64 as libc::c_int as uint) as isize)
        & (1 as libc::c_int as uint64_t) << p_bit % 64 as libc::c_int as uint;
}
unsafe extern "C" fn vli_numDigits(mut p_vli: *mut uint64_t) -> uint {
    let mut i: libc::c_int = 0;
    i = 32 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int && *p_vli.offset(i as isize) == 0 as libc::c_int as uint64_t {
        i -= 1;
    }
    return (i + 1 as libc::c_int) as uint;
}
unsafe extern "C" fn vli_numBits(mut p_vli: *mut uint64_t) -> uint {
    let mut i: uint = 0;
    let mut l_digit: uint64_t = 0;
    let mut l_numDigits: uint = vli_numDigits(p_vli);
    if l_numDigits == 0 as libc::c_int as uint {
        return 0 as libc::c_int as uint;
    }
    l_digit = *p_vli.offset(l_numDigits.wrapping_sub(1 as libc::c_int as uint) as isize);
    i = 0 as libc::c_int as uint;
    while l_digit != 0 {
        l_digit >>= 1 as libc::c_int;
        i = i.wrapping_add(1);
    }
    return (l_numDigits.wrapping_sub(1 as libc::c_int as uint) * 64 as libc::c_int as uint)
        .wrapping_add(i);
}
unsafe extern "C" fn vli_set(mut p_dest: *mut uint64_t, mut p_src: *mut uint64_t) {
    let mut i: uint = 0;
    i = 0 as libc::c_int as uint;
    while i < (32 as libc::c_int / 8 as libc::c_int) as uint {
        *p_dest.offset(i as isize) = *p_src.offset(i as isize);
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn vli_cmp(mut p_left: *mut uint64_t, mut p_right: *mut uint64_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 32 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if *p_left.offset(i as isize) > *p_right.offset(i as isize) {
            return 1 as libc::c_int;
        } else if *p_left.offset(i as isize) < *p_right.offset(i as isize) {
            return -(1 as libc::c_int);
        }
        i -= 1;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn vli_lshift(
    mut p_result: *mut uint64_t,
    mut p_in: *mut uint64_t,
    mut p_shift: uint,
) -> uint64_t {
    let mut l_carry: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: uint = 0;
    i = 0 as libc::c_int as uint;
    while i < (32 as libc::c_int / 8 as libc::c_int) as uint {
        let mut l_temp: uint64_t = *p_in.offset(i as isize);
        *p_result.offset(i as isize) = l_temp << p_shift | l_carry;
        l_carry = l_temp >> (64 as libc::c_int as uint).wrapping_sub(p_shift);
        i = i.wrapping_add(1);
    }
    return l_carry;
}
unsafe extern "C" fn vli_rshift1(mut p_vli: *mut uint64_t) {
    let mut l_end: *mut uint64_t = p_vli;
    let mut l_carry: uint64_t = 0 as libc::c_int as uint64_t;
    p_vli = p_vli.offset((32 as libc::c_int / 8 as libc::c_int) as isize);
    loop {
        let fresh0 = p_vli;
        p_vli = p_vli.offset(-1);
        if !(fresh0 > l_end) {
            break;
        }
        let mut l_temp: uint64_t = *p_vli;
        *p_vli = l_temp >> 1 as libc::c_int | l_carry;
        l_carry = l_temp << 63 as libc::c_int;
    }
}
unsafe extern "C" fn vli_add(
    mut p_result: *mut uint64_t,
    mut p_left: *mut uint64_t,
    mut p_right: *mut uint64_t,
) -> uint64_t {
    let mut l_carry: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: uint = 0;
    i = 0 as libc::c_int as uint;
    while i < (32 as libc::c_int / 8 as libc::c_int) as uint {
        let mut l_sum: uint64_t = (*p_left.offset(i as isize))
            .wrapping_add(*p_right.offset(i as isize))
            .wrapping_add(l_carry);
        if l_sum != *p_left.offset(i as isize) {
            l_carry = (l_sum < *p_left.offset(i as isize)) as libc::c_int as uint64_t;
        }
        *p_result.offset(i as isize) = l_sum;
        i = i.wrapping_add(1);
    }
    return l_carry;
}
unsafe extern "C" fn vli_sub(
    mut p_result: *mut uint64_t,
    mut p_left: *mut uint64_t,
    mut p_right: *mut uint64_t,
) -> uint64_t {
    let mut l_borrow: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: uint = 0;
    i = 0 as libc::c_int as uint;
    while i < (32 as libc::c_int / 8 as libc::c_int) as uint {
        let mut l_diff: uint64_t = (*p_left.offset(i as isize))
            .wrapping_sub(*p_right.offset(i as isize))
            .wrapping_sub(l_borrow);
        if l_diff != *p_left.offset(i as isize) {
            l_borrow = (l_diff > *p_left.offset(i as isize)) as libc::c_int as uint64_t;
        }
        *p_result.offset(i as isize) = l_diff;
        i = i.wrapping_add(1);
    }
    return l_borrow;
}
unsafe extern "C" fn vli_mult(
    mut p_result: *mut uint64_t,
    mut p_left: *mut uint64_t,
    mut p_right: *mut uint64_t,
) {
    let mut r01: uint128_t = 0 as libc::c_int as uint128_t;
    let mut r2: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: uint = 0;
    let mut k: uint = 0;
    k = 0 as libc::c_int as uint;
    while k < (32 as libc::c_int / 8 as libc::c_int * 2 as libc::c_int - 1 as libc::c_int) as uint {
        let mut l_min: uint = if k < (32 as libc::c_int / 8 as libc::c_int) as uint {
            0 as libc::c_int as uint
        } else {
            k.wrapping_add(1 as libc::c_int as uint)
                .wrapping_sub((32 as libc::c_int / 8 as libc::c_int) as uint)
        };
        i = l_min;
        while i <= k && i < (32 as libc::c_int / 8 as libc::c_int) as uint {
            let mut l_product: uint128_t = *p_left.offset(i as isize) as uint128_t
                * *p_right.offset(k.wrapping_sub(i) as isize) as uint128_t;
            r01 = r01.wrapping_add(l_product);
            r2 = r2.wrapping_add((r01 < l_product) as libc::c_int as uint64_t);
            i = i.wrapping_add(1);
        }
        *p_result.offset(k as isize) = r01 as uint64_t;
        r01 = r01 >> 64 as libc::c_int | (r2 as uint128_t) << 64 as libc::c_int;
        r2 = 0 as libc::c_int as uint64_t;
        k = k.wrapping_add(1);
    }
    *p_result.offset(
        (32 as libc::c_int / 8 as libc::c_int * 2 as libc::c_int - 1 as libc::c_int) as isize,
    ) = r01 as uint64_t;
}
unsafe extern "C" fn vli_square(mut p_result: *mut uint64_t, mut p_left: *mut uint64_t) {
    let mut r01: uint128_t = 0 as libc::c_int as uint128_t;
    let mut r2: uint64_t = 0 as libc::c_int as uint64_t;
    let mut i: uint = 0;
    let mut k: uint = 0;
    while k < (32 as libc::c_int / 8 as libc::c_int * 2 as libc::c_int - 1 as libc::c_int) as uint {
        let mut l_min: uint = if k < (32 as libc::c_int / 8 as libc::c_int) as uint {
            0 as libc::c_int as uint
        } else {
            k.wrapping_add(1 as libc::c_int as uint)
                .wrapping_sub((32 as libc::c_int / 8 as libc::c_int) as uint)
        };
        i = l_min;
        while i <= k && i <= k.wrapping_sub(i) {
            let mut l_product: uint128_t = *p_left.offset(i as isize) as uint128_t
                * *p_left.offset(k.wrapping_sub(i) as isize) as uint128_t;
            if i < k.wrapping_sub(i) {
                r2 = (r2 as uint128_t).wrapping_add(l_product >> 127 as libc::c_int) as uint64_t
                    as uint64_t;
                l_product = l_product.wrapping_mul(2);
            }
            r01 = r01.wrapping_add(l_product);
            r2 = r2.wrapping_add((r01 < l_product) as libc::c_int as uint64_t);
            i = i.wrapping_add(1);
        }
        *p_result.offset(k as isize) = r01 as uint64_t;
        r01 = r01 >> 64 as libc::c_int | (r2 as uint128_t) << 64 as libc::c_int;
        r2 = 0 as libc::c_int as uint64_t;
        k = k.wrapping_add(1);
    }
    *p_result.offset(
        (32 as libc::c_int / 8 as libc::c_int * 2 as libc::c_int - 1 as libc::c_int) as isize,
    ) = r01 as uint64_t;
}

unsafe extern "C" fn vli_modAdd(
    mut p_result: *mut uint64_t,
    mut p_left: *mut uint64_t,
    mut p_right: *mut uint64_t,
    mut p_mod: *mut uint64_t,
) {
    let mut l_carry: uint64_t = vli_add(p_result, p_left, p_right);
    if l_carry != 0 || vli_cmp(p_result, p_mod) >= 0 as libc::c_int {
        vli_sub(p_result, p_result, p_mod);
    }
}
unsafe extern "C" fn vli_modSub(
    mut p_result: *mut uint64_t,
    mut p_left: *mut uint64_t,
    mut p_right: *mut uint64_t,
    mut p_mod: *mut uint64_t,
) {
    let mut l_borrow: uint64_t = vli_sub(p_result, p_left, p_right);
    if l_borrow != 0 {
        vli_add(p_result, p_result, p_mod);
    }
}
unsafe extern "C" fn vli_mmod_fast(mut p_result: *mut uint64_t, mut p_product: *mut uint64_t) {
    let mut l_tmp: [uint64_t; 4] = [0; 4];
    let mut l_carry: libc::c_int = 0;
    vli_set(p_result, p_product);
    l_tmp[0 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    l_tmp[1 as libc::c_int as usize] =
        (*p_product.offset(5 as libc::c_int as isize) as libc::c_ulonglong
            & 0xffffffff00000000 as libc::c_ulonglong) as uint64_t;
    l_tmp[2 as libc::c_int as usize] = *p_product.offset(6 as libc::c_int as isize);
    l_tmp[3 as libc::c_int as usize] = *p_product.offset(7 as libc::c_int as isize);
    l_carry = vli_lshift(
        l_tmp.as_mut_ptr(),
        l_tmp.as_mut_ptr(),
        1 as libc::c_int as uint,
    ) as libc::c_int;
    l_carry = (l_carry as uint64_t).wrapping_add(vli_add(p_result, p_result, l_tmp.as_mut_ptr()))
        as libc::c_int as libc::c_int;
    l_tmp[1 as libc::c_int as usize] =
        *p_product.offset(6 as libc::c_int as isize) << 32 as libc::c_int;
    l_tmp[2 as libc::c_int as usize] = *p_product.offset(6 as libc::c_int as isize)
        >> 32 as libc::c_int
        | *p_product.offset(7 as libc::c_int as isize) << 32 as libc::c_int;
    l_tmp[3 as libc::c_int as usize] =
        *p_product.offset(7 as libc::c_int as isize) >> 32 as libc::c_int;
    l_carry = (l_carry as uint64_t).wrapping_add(vli_lshift(
        l_tmp.as_mut_ptr(),
        l_tmp.as_mut_ptr(),
        1 as libc::c_int as uint,
    )) as libc::c_int as libc::c_int;
    l_carry = (l_carry as uint64_t).wrapping_add(vli_add(p_result, p_result, l_tmp.as_mut_ptr()))
        as libc::c_int as libc::c_int;
    l_tmp[0 as libc::c_int as usize] = *p_product.offset(4 as libc::c_int as isize);
    l_tmp[1 as libc::c_int as usize] =
        *p_product.offset(5 as libc::c_int as isize) & 0xffffffff as libc::c_uint as uint64_t;
    l_tmp[2 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    l_tmp[3 as libc::c_int as usize] = *p_product.offset(7 as libc::c_int as isize);
    l_carry = (l_carry as uint64_t).wrapping_add(vli_add(p_result, p_result, l_tmp.as_mut_ptr()))
        as libc::c_int as libc::c_int;
    l_tmp[0 as libc::c_int as usize] = *p_product.offset(4 as libc::c_int as isize)
        >> 32 as libc::c_int
        | *p_product.offset(5 as libc::c_int as isize) << 32 as libc::c_int;
    l_tmp[1 as libc::c_int as usize] =
        ((*p_product.offset(5 as libc::c_int as isize) >> 32 as libc::c_int) as libc::c_ulonglong
            | *p_product.offset(6 as libc::c_int as isize) as libc::c_ulonglong
                & 0xffffffff00000000 as libc::c_ulonglong) as uint64_t;
    l_tmp[2 as libc::c_int as usize] = *p_product.offset(7 as libc::c_int as isize);
    l_tmp[3 as libc::c_int as usize] = *p_product.offset(6 as libc::c_int as isize)
        >> 32 as libc::c_int
        | *p_product.offset(4 as libc::c_int as isize) << 32 as libc::c_int;
    l_carry = (l_carry as uint64_t).wrapping_add(vli_add(p_result, p_result, l_tmp.as_mut_ptr()))
        as libc::c_int as libc::c_int;
    l_tmp[0 as libc::c_int as usize] = *p_product.offset(5 as libc::c_int as isize)
        >> 32 as libc::c_int
        | *p_product.offset(6 as libc::c_int as isize) << 32 as libc::c_int;
    l_tmp[1 as libc::c_int as usize] =
        *p_product.offset(6 as libc::c_int as isize) >> 32 as libc::c_int;
    l_tmp[2 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    l_tmp[3 as libc::c_int as usize] = *p_product.offset(4 as libc::c_int as isize)
        & 0xffffffff as libc::c_uint as uint64_t
        | *p_product.offset(5 as libc::c_int as isize) << 32 as libc::c_int;
    l_carry = (l_carry as uint64_t).wrapping_sub(vli_sub(p_result, p_result, l_tmp.as_mut_ptr()))
        as libc::c_int as libc::c_int;
    l_tmp[0 as libc::c_int as usize] = *p_product.offset(6 as libc::c_int as isize);
    l_tmp[1 as libc::c_int as usize] = *p_product.offset(7 as libc::c_int as isize);
    l_tmp[2 as libc::c_int as usize] = 0 as libc::c_int as uint64_t;
    l_tmp[3 as libc::c_int as usize] =
        ((*p_product.offset(4 as libc::c_int as isize) >> 32 as libc::c_int) as libc::c_ulonglong
            | *p_product.offset(5 as libc::c_int as isize) as libc::c_ulonglong
                & 0xffffffff00000000 as libc::c_ulonglong) as uint64_t;
    l_carry = (l_carry as uint64_t).wrapping_sub(vli_sub(p_result, p_result, l_tmp.as_mut_ptr()))
        as libc::c_int as libc::c_int;
    l_tmp[0 as libc::c_int as usize] = *p_product.offset(6 as libc::c_int as isize)
        >> 32 as libc::c_int
        | *p_product.offset(7 as libc::c_int as isize) << 32 as libc::c_int;
    l_tmp[1 as libc::c_int as usize] = *p_product.offset(7 as libc::c_int as isize)
        >> 32 as libc::c_int
        | *p_product.offset(4 as libc::c_int as isize) << 32 as libc::c_int;
    l_tmp[2 as libc::c_int as usize] = *p_product.offset(4 as libc::c_int as isize)
        >> 32 as libc::c_int
        | *p_product.offset(5 as libc::c_int as isize) << 32 as libc::c_int;
    l_tmp[3 as libc::c_int as usize] =
        *p_product.offset(6 as libc::c_int as isize) << 32 as libc::c_int;
    l_carry = (l_carry as uint64_t).wrapping_sub(vli_sub(p_result, p_result, l_tmp.as_mut_ptr()))
        as libc::c_int as libc::c_int;
    l_tmp[0 as libc::c_int as usize] = *p_product.offset(7 as libc::c_int as isize);
    l_tmp[1 as libc::c_int as usize] =
        (*p_product.offset(4 as libc::c_int as isize) as libc::c_ulonglong
            & 0xffffffff00000000 as libc::c_ulonglong) as uint64_t;
    l_tmp[2 as libc::c_int as usize] = *p_product.offset(5 as libc::c_int as isize);
    l_tmp[3 as libc::c_int as usize] =
        (*p_product.offset(6 as libc::c_int as isize) as libc::c_ulonglong
            & 0xffffffff00000000 as libc::c_ulonglong) as uint64_t;
    l_carry = (l_carry as uint64_t).wrapping_sub(vli_sub(p_result, p_result, l_tmp.as_mut_ptr()))
        as libc::c_int as libc::c_int;
    if l_carry < 0 as libc::c_int {
        loop {
            l_carry = (l_carry as uint64_t).wrapping_add(vli_add(
                p_result,
                p_result,
                curve_p.as_mut_ptr(),
            )) as libc::c_int as libc::c_int;
            if !(l_carry < 0 as libc::c_int) {
                break;
            }
        }
    } else {
        while l_carry != 0 || vli_cmp(curve_p.as_mut_ptr(), p_result) != 1 as libc::c_int {
            l_carry = (l_carry as uint64_t).wrapping_sub(vli_sub(
                p_result,
                p_result,
                curve_p.as_mut_ptr(),
            )) as libc::c_int as libc::c_int;
        }
    };
}
unsafe extern "C" fn vli_modMult_fast(
    mut p_result: *mut uint64_t,
    mut p_left: *mut uint64_t,
    mut p_right: *mut uint64_t,
) {
    let mut l_product: [uint64_t; 8] = [0; 8];
    vli_mult(l_product.as_mut_ptr(), p_left, p_right);
    vli_mmod_fast(p_result, l_product.as_mut_ptr());
}
unsafe extern "C" fn vli_modSquare_fast(mut p_result: *mut uint64_t, mut p_left: *mut uint64_t) {
    let mut l_product: [uint64_t; 8] = [0; 8];
    vli_square(l_product.as_mut_ptr(), p_left);
    vli_mmod_fast(p_result, l_product.as_mut_ptr());
}
unsafe extern "C" fn vli_modInv(
    mut p_result: *mut uint64_t,
    mut p_input: *mut uint64_t,
    mut p_mod: *mut uint64_t,
) {
    let mut a: [uint64_t; 4] = [0; 4];
    let mut b: [uint64_t; 4] = [0; 4];
    let mut u: [uint64_t; 4] = [0; 4];
    let mut v: [uint64_t; 4] = [0; 4];
    let mut l_carry: uint64_t = 0;
    let mut l_cmpResult: libc::c_int = 0;
    if vli_isZero(p_input) != 0 {
        vli_clear(p_result);
        return;
    }
    vli_set(a.as_mut_ptr(), p_input);
    vli_set(b.as_mut_ptr(), p_mod);
    vli_clear(u.as_mut_ptr());
    u[0 as libc::c_int as usize] = 1 as libc::c_int as uint64_t;
    vli_clear(v.as_mut_ptr());
    loop {
        l_cmpResult = vli_cmp(a.as_mut_ptr(), b.as_mut_ptr());
        if !(l_cmpResult != 0 as libc::c_int) {
            break;
        }
        l_carry = 0 as libc::c_int as uint64_t;
        if a[0 as libc::c_int as usize] & 1 as libc::c_int as uint64_t == 0 {
            vli_rshift1(a.as_mut_ptr());
            if u[0 as libc::c_int as usize] & 1 as libc::c_int as uint64_t != 0 {
                l_carry = vli_add(u.as_mut_ptr(), u.as_mut_ptr(), p_mod);
            }
            vli_rshift1(u.as_mut_ptr());
            if l_carry != 0 {
                u[(32 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int) as usize] =
                    (u[(32 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int) as usize]
                        as libc::c_ulonglong
                        | 0x8000000000000000 as libc::c_ulonglong) as uint64_t;
            }
        } else if b[0 as libc::c_int as usize] & 1 as libc::c_int as uint64_t == 0 {
            vli_rshift1(b.as_mut_ptr());
            if v[0 as libc::c_int as usize] & 1 as libc::c_int as uint64_t != 0 {
                l_carry = vli_add(v.as_mut_ptr(), v.as_mut_ptr(), p_mod);
            }
            vli_rshift1(v.as_mut_ptr());
            if l_carry != 0 {
                v[(32 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int) as usize] =
                    (v[(32 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int) as usize]
                        as libc::c_ulonglong
                        | 0x8000000000000000 as libc::c_ulonglong) as uint64_t;
            }
        } else if l_cmpResult > 0 as libc::c_int {
            vli_sub(a.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
            vli_rshift1(a.as_mut_ptr());
            if vli_cmp(u.as_mut_ptr(), v.as_mut_ptr()) < 0 as libc::c_int {
                vli_add(u.as_mut_ptr(), u.as_mut_ptr(), p_mod);
            }
            vli_sub(u.as_mut_ptr(), u.as_mut_ptr(), v.as_mut_ptr());
            if u[0 as libc::c_int as usize] & 1 as libc::c_int as uint64_t != 0 {
                l_carry = vli_add(u.as_mut_ptr(), u.as_mut_ptr(), p_mod);
            }
            vli_rshift1(u.as_mut_ptr());
            if l_carry != 0 {
                u[(32 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int) as usize] =
                    (u[(32 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int) as usize]
                        as libc::c_ulonglong
                        | 0x8000000000000000 as libc::c_ulonglong) as uint64_t;
            }
        } else {
            vli_sub(b.as_mut_ptr(), b.as_mut_ptr(), a.as_mut_ptr());
            vli_rshift1(b.as_mut_ptr());
            if vli_cmp(v.as_mut_ptr(), u.as_mut_ptr()) < 0 as libc::c_int {
                vli_add(v.as_mut_ptr(), v.as_mut_ptr(), p_mod);
            }
            vli_sub(v.as_mut_ptr(), v.as_mut_ptr(), u.as_mut_ptr());
            if v[0 as libc::c_int as usize] & 1 as libc::c_int as uint64_t != 0 {
                l_carry = vli_add(v.as_mut_ptr(), v.as_mut_ptr(), p_mod);
            }
            vli_rshift1(v.as_mut_ptr());
            if l_carry != 0 {
                v[(32 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int) as usize] =
                    (v[(32 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int) as usize]
                        as libc::c_ulonglong
                        | 0x8000000000000000 as libc::c_ulonglong) as uint64_t;
            }
        }
    }
    vli_set(p_result, u.as_mut_ptr());
}
unsafe extern "C" fn EccPoint_isZero(mut p_point: *mut EccPoint) -> libc::c_int {
    return (vli_isZero(((*p_point).x).as_mut_ptr()) != 0
        && vli_isZero(((*p_point).y).as_mut_ptr()) != 0) as libc::c_int;
}
unsafe extern "C" fn EccPoint_double_jacobian(
    mut X1: *mut uint64_t,
    mut Y1: *mut uint64_t,
    mut Z1: *mut uint64_t,
) {
    let mut t4: [uint64_t; 4] = [0; 4];
    let mut t5: [uint64_t; 4] = [0; 4];
    if vli_isZero(Z1) != 0 {
        return;
    }
    vli_modSquare_fast(t4.as_mut_ptr(), Y1);
    vli_modMult_fast(t5.as_mut_ptr(), X1, t4.as_mut_ptr());
    vli_modSquare_fast(t4.as_mut_ptr(), t4.as_mut_ptr());
    vli_modMult_fast(Y1, Y1, Z1);
    vli_modSquare_fast(Z1, Z1);
    vli_modAdd(X1, X1, Z1, curve_p.as_mut_ptr());
    vli_modAdd(Z1, Z1, Z1, curve_p.as_mut_ptr());
    vli_modSub(Z1, X1, Z1, curve_p.as_mut_ptr());
    vli_modMult_fast(X1, X1, Z1);
    vli_modAdd(Z1, X1, X1, curve_p.as_mut_ptr());
    vli_modAdd(X1, X1, Z1, curve_p.as_mut_ptr());
    if vli_testBit(X1, 0 as libc::c_int as uint) != 0 {
        let mut l_carry: uint64_t = vli_add(X1, X1, curve_p.as_mut_ptr());
        vli_rshift1(X1);
        *X1.offset((32 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int) as isize) |=
            l_carry << 63 as libc::c_int;
    } else {
        vli_rshift1(X1);
    }
    vli_modSquare_fast(Z1, X1);
    vli_modSub(Z1, Z1, t5.as_mut_ptr(), curve_p.as_mut_ptr());
    vli_modSub(Z1, Z1, t5.as_mut_ptr(), curve_p.as_mut_ptr());
    vli_modSub(t5.as_mut_ptr(), t5.as_mut_ptr(), Z1, curve_p.as_mut_ptr());
    vli_modMult_fast(X1, X1, t5.as_mut_ptr());
    vli_modSub(t4.as_mut_ptr(), X1, t4.as_mut_ptr(), curve_p.as_mut_ptr());
    vli_set(X1, Z1);
    vli_set(Z1, Y1);
    vli_set(Y1, t4.as_mut_ptr());
}
unsafe extern "C" fn apply_z(mut X1: *mut uint64_t, mut Y1: *mut uint64_t, mut Z: *mut uint64_t) {
    let mut t1: [uint64_t; 4] = [0; 4];
    vli_modSquare_fast(t1.as_mut_ptr(), Z);
    vli_modMult_fast(X1, X1, t1.as_mut_ptr());
    vli_modMult_fast(t1.as_mut_ptr(), t1.as_mut_ptr(), Z);
    vli_modMult_fast(Y1, Y1, t1.as_mut_ptr());
}
unsafe extern "C" fn XYcZ_initial_double(
    mut X1: *mut uint64_t,
    mut Y1: *mut uint64_t,
    mut X2: *mut uint64_t,
    mut Y2: *mut uint64_t,
    mut p_initialZ: *mut uint64_t,
) {
    let mut z: [uint64_t; 4] = [0; 4];
    vli_set(X2, X1);
    vli_set(Y2, Y1);
    vli_clear(z.as_mut_ptr());
    z[0 as libc::c_int as usize] = 1 as libc::c_int as uint64_t;
    if !p_initialZ.is_null() {
        vli_set(z.as_mut_ptr(), p_initialZ);
    }
    apply_z(X1, Y1, z.as_mut_ptr());
    EccPoint_double_jacobian(X1, Y1, z.as_mut_ptr());
    apply_z(X2, Y2, z.as_mut_ptr());
}
unsafe extern "C" fn XYcZ_add(
    mut X1: *mut uint64_t,
    mut Y1: *mut uint64_t,
    mut X2: *mut uint64_t,
    mut Y2: *mut uint64_t,
) {
    let mut t5: [uint64_t; 4] = [0; 4];
    vli_modSub(t5.as_mut_ptr(), X2, X1, curve_p.as_mut_ptr());
    vli_modSquare_fast(t5.as_mut_ptr(), t5.as_mut_ptr());
    vli_modMult_fast(X1, X1, t5.as_mut_ptr());
    vli_modMult_fast(X2, X2, t5.as_mut_ptr());
    vli_modSub(Y2, Y2, Y1, curve_p.as_mut_ptr());
    vli_modSquare_fast(t5.as_mut_ptr(), Y2);
    vli_modSub(t5.as_mut_ptr(), t5.as_mut_ptr(), X1, curve_p.as_mut_ptr());
    vli_modSub(t5.as_mut_ptr(), t5.as_mut_ptr(), X2, curve_p.as_mut_ptr());
    vli_modSub(X2, X2, X1, curve_p.as_mut_ptr());
    vli_modMult_fast(Y1, Y1, X2);
    vli_modSub(X2, X1, t5.as_mut_ptr(), curve_p.as_mut_ptr());
    vli_modMult_fast(Y2, Y2, X2);
    vli_modSub(Y2, Y2, Y1, curve_p.as_mut_ptr());
    vli_set(X2, t5.as_mut_ptr());
}
unsafe extern "C" fn XYcZ_addC(
    mut X1: *mut uint64_t,
    mut Y1: *mut uint64_t,
    mut X2: *mut uint64_t,
    mut Y2: *mut uint64_t,
) {
    let mut t5: [uint64_t; 4] = [0; 4];
    let mut t6: [uint64_t; 4] = [0; 4];
    let mut t7: [uint64_t; 4] = [0; 4];
    vli_modSub(t5.as_mut_ptr(), X2, X1, curve_p.as_mut_ptr());
    vli_modSquare_fast(t5.as_mut_ptr(), t5.as_mut_ptr());
    vli_modMult_fast(X1, X1, t5.as_mut_ptr());
    vli_modMult_fast(X2, X2, t5.as_mut_ptr());
    vli_modAdd(t5.as_mut_ptr(), Y2, Y1, curve_p.as_mut_ptr());
    vli_modSub(Y2, Y2, Y1, curve_p.as_mut_ptr());
    vli_modSub(t6.as_mut_ptr(), X2, X1, curve_p.as_mut_ptr());
    vli_modMult_fast(Y1, Y1, t6.as_mut_ptr());
    vli_modAdd(t6.as_mut_ptr(), X1, X2, curve_p.as_mut_ptr());
    vli_modSquare_fast(X2, Y2);
    vli_modSub(X2, X2, t6.as_mut_ptr(), curve_p.as_mut_ptr());
    vli_modSub(t7.as_mut_ptr(), X1, X2, curve_p.as_mut_ptr());
    vli_modMult_fast(Y2, Y2, t7.as_mut_ptr());
    vli_modSub(Y2, Y2, Y1, curve_p.as_mut_ptr());
    vli_modSquare_fast(t7.as_mut_ptr(), t5.as_mut_ptr());
    vli_modSub(
        t7.as_mut_ptr(),
        t7.as_mut_ptr(),
        t6.as_mut_ptr(),
        curve_p.as_mut_ptr(),
    );
    vli_modSub(t6.as_mut_ptr(), t7.as_mut_ptr(), X1, curve_p.as_mut_ptr());
    vli_modMult_fast(t6.as_mut_ptr(), t6.as_mut_ptr(), t5.as_mut_ptr());
    vli_modSub(Y1, t6.as_mut_ptr(), Y1, curve_p.as_mut_ptr());
    vli_set(X1, t7.as_mut_ptr());
}
unsafe extern "C" fn EccPoint_mult(
    mut p_result: *mut EccPoint,
    mut p_point: *mut EccPoint,
    mut p_scalar: *mut uint64_t,
    mut p_initialZ: *mut uint64_t,
) {
    let mut Rx: [[uint64_t; 4]; 2] = [[0; 4]; 2];
    let mut Ry: [[uint64_t; 4]; 2] = [[0; 4]; 2];
    let mut z: [uint64_t; 4] = [0; 4];
    let mut i: libc::c_int = 0;
    let mut nb: libc::c_int = 0;
    vli_set(
        (Rx[1 as libc::c_int as usize]).as_mut_ptr(),
        ((*p_point).x).as_mut_ptr(),
    );
    vli_set(
        (Ry[1 as libc::c_int as usize]).as_mut_ptr(),
        ((*p_point).y).as_mut_ptr(),
    );
    XYcZ_initial_double(
        (Rx[1 as libc::c_int as usize]).as_mut_ptr(),
        (Ry[1 as libc::c_int as usize]).as_mut_ptr(),
        (Rx[0 as libc::c_int as usize]).as_mut_ptr(),
        (Ry[0 as libc::c_int as usize]).as_mut_ptr(),
        p_initialZ,
    );
    i = (vli_numBits(p_scalar)).wrapping_sub(2 as libc::c_int as uint) as libc::c_int;
    while i > 0 as libc::c_int {
        nb = (vli_testBit(p_scalar, i as uint) == 0) as libc::c_int;
        XYcZ_addC(
            (Rx[(1 as libc::c_int - nb) as usize]).as_mut_ptr(),
            (Ry[(1 as libc::c_int - nb) as usize]).as_mut_ptr(),
            (Rx[nb as usize]).as_mut_ptr(),
            (Ry[nb as usize]).as_mut_ptr(),
        );
        XYcZ_add(
            (Rx[nb as usize]).as_mut_ptr(),
            (Ry[nb as usize]).as_mut_ptr(),
            (Rx[(1 as libc::c_int - nb) as usize]).as_mut_ptr(),
            (Ry[(1 as libc::c_int - nb) as usize]).as_mut_ptr(),
        );
        i -= 1;
    }
    nb = (vli_testBit(p_scalar, 0 as libc::c_int as uint) == 0) as libc::c_int;
    XYcZ_addC(
        (Rx[(1 as libc::c_int - nb) as usize]).as_mut_ptr(),
        (Ry[(1 as libc::c_int - nb) as usize]).as_mut_ptr(),
        (Rx[nb as usize]).as_mut_ptr(),
        (Ry[nb as usize]).as_mut_ptr(),
    );
    vli_modSub(
        z.as_mut_ptr(),
        (Rx[1 as libc::c_int as usize]).as_mut_ptr(),
        (Rx[0 as libc::c_int as usize]).as_mut_ptr(),
        curve_p.as_mut_ptr(),
    );
    vli_modMult_fast(
        z.as_mut_ptr(),
        z.as_mut_ptr(),
        (Ry[(1 as libc::c_int - nb) as usize]).as_mut_ptr(),
    );
    vli_modMult_fast(z.as_mut_ptr(), z.as_mut_ptr(), ((*p_point).x).as_mut_ptr());
    vli_modInv(z.as_mut_ptr(), z.as_mut_ptr(), curve_p.as_mut_ptr());
    vli_modMult_fast(z.as_mut_ptr(), z.as_mut_ptr(), ((*p_point).y).as_mut_ptr());
    vli_modMult_fast(
        z.as_mut_ptr(),
        z.as_mut_ptr(),
        (Rx[(1 as libc::c_int - nb) as usize]).as_mut_ptr(),
    );
    XYcZ_add(
        (Rx[nb as usize]).as_mut_ptr(),
        (Ry[nb as usize]).as_mut_ptr(),
        (Rx[(1 as libc::c_int - nb) as usize]).as_mut_ptr(),
        (Ry[(1 as libc::c_int - nb) as usize]).as_mut_ptr(),
    );
    apply_z(
        (Rx[0 as libc::c_int as usize]).as_mut_ptr(),
        (Ry[0 as libc::c_int as usize]).as_mut_ptr(),
        z.as_mut_ptr(),
    );
    vli_set(
        ((*p_result).x).as_mut_ptr(),
        (Rx[0 as libc::c_int as usize]).as_mut_ptr(),
    );
    vli_set(
        ((*p_result).y).as_mut_ptr(),
        (Ry[0 as libc::c_int as usize]).as_mut_ptr(),
    );
}
unsafe extern "C" fn ecc_bytes2native(mut p_native: *mut uint64_t, mut p_bytes: *const uint8_t) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint {
        let mut p_digit: *const uint8_t = p_bytes.offset(
            (8 as libc::c_int as libc::c_uint).wrapping_mul(
                ((32 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                    .wrapping_sub(i),
            ) as isize,
        );
        *p_native.offset(i as isize) = (*p_digit.offset(0 as libc::c_int as isize) as uint64_t)
            << 56 as libc::c_int
            | (*p_digit.offset(1 as libc::c_int as isize) as uint64_t) << 48 as libc::c_int
            | (*p_digit.offset(2 as libc::c_int as isize) as uint64_t) << 40 as libc::c_int
            | (*p_digit.offset(3 as libc::c_int as isize) as uint64_t) << 32 as libc::c_int
            | (*p_digit.offset(4 as libc::c_int as isize) as uint64_t) << 24 as libc::c_int
            | (*p_digit.offset(5 as libc::c_int as isize) as uint64_t) << 16 as libc::c_int
            | (*p_digit.offset(6 as libc::c_int as isize) as uint64_t) << 8 as libc::c_int
            | *p_digit.offset(7 as libc::c_int as isize) as uint64_t;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn ecc_native2bytes(mut p_bytes: *mut uint8_t, mut p_native: *const uint64_t) {
    let mut i: libc::c_uint = 0;
    i = 0 as libc::c_int as libc::c_uint;
    while i < (32 as libc::c_int / 8 as libc::c_int) as libc::c_uint {
        let mut p_digit: *mut uint8_t = p_bytes.offset(
            (8 as libc::c_int as libc::c_uint).wrapping_mul(
                ((32 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int) as libc::c_uint)
                    .wrapping_sub(i),
            ) as isize,
        );
        *p_digit.offset(0 as libc::c_int as isize) =
            (*p_native.offset(i as isize) >> 56 as libc::c_int) as uint8_t;
        *p_digit.offset(1 as libc::c_int as isize) =
            (*p_native.offset(i as isize) >> 48 as libc::c_int) as uint8_t;
        *p_digit.offset(2 as libc::c_int as isize) =
            (*p_native.offset(i as isize) >> 40 as libc::c_int) as uint8_t;
        *p_digit.offset(3 as libc::c_int as isize) =
            (*p_native.offset(i as isize) >> 32 as libc::c_int) as uint8_t;
        *p_digit.offset(4 as libc::c_int as isize) =
            (*p_native.offset(i as isize) >> 24 as libc::c_int) as uint8_t;
        *p_digit.offset(5 as libc::c_int as isize) =
            (*p_native.offset(i as isize) >> 16 as libc::c_int) as uint8_t;
        *p_digit.offset(6 as libc::c_int as isize) =
            (*p_native.offset(i as isize) >> 8 as libc::c_int) as uint8_t;
        *p_digit.offset(7 as libc::c_int as isize) = *p_native.offset(i as isize) as uint8_t;
        i = i.wrapping_add(1);
    }
}
unsafe extern "C" fn ecc_point_decompress(
    mut p_point: *mut EccPoint,
    mut p_compressed: *const uint8_t,
) {
    let mut _3: [uint64_t; 4] = [3 as libc::c_int as uint64_t, 0, 0, 0];
    ecc_bytes2native(((*p_point).x).as_mut_ptr(), p_compressed);
    ecc_bytes2native(
        ((*p_point).y).as_mut_ptr(),
        p_compressed.offset(32 as libc::c_int as isize),
    );
}
#[no_mangle]
pub unsafe extern "C" fn ecc_make_key(
    mut p_publicKey: *mut uint8_t,
    mut p_privateKey: *mut uint8_t,
) -> libc::c_int {
    let mut l_private: [uint64_t; 4] = [0; 4];
    let mut l_public: EccPoint = EccPoint {
        x: [0; 4],
        y: [0; 4],
    };
    let mut l_tries: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    loop {
        if getRandomNumber(l_private.as_mut_ptr()) == 0 || {
            let fresh1 = l_tries;
            l_tries = l_tries.wrapping_add(1);
            fresh1 >= 16 as libc::c_int as libc::c_uint
        } {
            return 0 as libc::c_int;
        }
        if !(vli_isZero(l_private.as_mut_ptr()) != 0) {
            i = 0 as libc::c_int as uint32_t;
            while i < (32 as libc::c_int / 8 as libc::c_int) as uint32_t {
                i = i.wrapping_add(1);
            }
            if vli_cmp(curve_n.as_mut_ptr(), l_private.as_mut_ptr()) != 1 as libc::c_int {
                vli_sub(
                    l_private.as_mut_ptr(),
                    l_private.as_mut_ptr(),
                    curve_n.as_mut_ptr(),
                );
            }
            EccPoint_mult(
                &mut l_public,
                addr_of_mut!(curve_G),
                l_private.as_mut_ptr(),
                0 as *mut uint64_t,
            );
        }
        if !(EccPoint_isZero(&mut l_public) != 0) {
            break;
        }
    }
    ecc_native2bytes(p_privateKey, l_private.as_mut_ptr() as *const uint64_t);
    ecc_native2bytes(p_publicKey, (l_public.x).as_mut_ptr() as *const uint64_t);
    ecc_native2bytes(
        &mut *p_publicKey.offset(32 as libc::c_int as isize),
        (l_public.y).as_mut_ptr() as *const uint64_t,
    );
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ecdh_shared_secret(
    mut p_publicKey: *const uint8_t,
    mut p_privateKey: *const uint8_t,
    mut p_secret: *mut uint8_t,
) -> libc::c_int {
    let mut l_public: EccPoint = EccPoint {
        x: [0; 4],
        y: [0; 4],
    };
    let mut l_private: [uint64_t; 4] = [0; 4];
    let mut l_random: [uint64_t; 4] = [0; 4];
    if getRandomNumber(l_random.as_mut_ptr()) == 0 {
        return 0 as libc::c_int;
    }
    ecc_point_decompress(&mut l_public, p_publicKey);
    ecc_bytes2native(l_private.as_mut_ptr(), p_privateKey);
    let mut l_product: EccPoint = EccPoint {
        x: [0; 4],
        y: [0; 4],
    };
    EccPoint_mult(
        &mut l_product,
        &mut l_public,
        l_private.as_mut_ptr(),
        l_random.as_mut_ptr(),
    );
    ecc_native2bytes(p_secret, (l_product.x).as_mut_ptr() as *const uint64_t);
    return (EccPoint_isZero(&mut l_product) == 0) as libc::c_int;
}
unsafe extern "C" fn vli_modMult(
    mut p_result: *mut uint64_t,
    mut p_left: *mut uint64_t,
    mut p_right: *mut uint64_t,
    mut p_mod: *mut uint64_t,
) {
    let mut l_product: [uint64_t; 8] = [0; 8];
    let mut l_modMultiple: [uint64_t; 8] = [0; 8];
    let mut l_digitShift: uint = 0;
    let mut l_bitShift: uint = 0;
    let mut l_productBits: uint = 0;
    let mut l_modBits: uint = vli_numBits(p_mod);
    vli_mult(l_product.as_mut_ptr(), p_left, p_right);
    l_productBits = vli_numBits(
        l_product
            .as_mut_ptr()
            .offset((32 as libc::c_int / 8 as libc::c_int) as isize),
    );
    if l_productBits != 0 {
        l_productBits = l_productBits
            .wrapping_add((32 as libc::c_int / 8 as libc::c_int * 64 as libc::c_int) as uint);
    } else {
        l_productBits = vli_numBits(l_product.as_mut_ptr());
    }
    if l_productBits < l_modBits {
        vli_set(p_result, l_product.as_mut_ptr());
        return;
    }
    vli_clear(l_modMultiple.as_mut_ptr());
    vli_clear(
        l_modMultiple
            .as_mut_ptr()
            .offset((32 as libc::c_int / 8 as libc::c_int) as isize),
    );
    l_digitShift = l_productBits.wrapping_sub(l_modBits) / 64 as libc::c_int as uint;
    l_bitShift = l_productBits.wrapping_sub(l_modBits) % 64 as libc::c_int as uint;
    if l_bitShift != 0 {
        l_modMultiple
            [l_digitShift.wrapping_add((32 as libc::c_int / 8 as libc::c_int) as uint) as usize] =
            vli_lshift(
                l_modMultiple.as_mut_ptr().offset(l_digitShift as isize),
                p_mod,
                l_bitShift,
            );
    } else {
        vli_set(
            l_modMultiple.as_mut_ptr().offset(l_digitShift as isize),
            p_mod,
        );
    }
    vli_clear(p_result);
    *p_result.offset(0 as libc::c_int as isize) = 1 as libc::c_int as uint64_t;
    while l_productBits > (32 as libc::c_int / 8 as libc::c_int * 64 as libc::c_int) as uint
        || vli_cmp(l_modMultiple.as_mut_ptr(), p_mod) >= 0 as libc::c_int
    {
        let mut l_cmp: libc::c_int = vli_cmp(
            l_modMultiple
                .as_mut_ptr()
                .offset((32 as libc::c_int / 8 as libc::c_int) as isize),
            l_product
                .as_mut_ptr()
                .offset((32 as libc::c_int / 8 as libc::c_int) as isize),
        );
        if l_cmp < 0 as libc::c_int
            || l_cmp == 0 as libc::c_int
                && vli_cmp(l_modMultiple.as_mut_ptr(), l_product.as_mut_ptr()) <= 0 as libc::c_int
        {
            if vli_sub(
                l_product.as_mut_ptr(),
                l_product.as_mut_ptr(),
                l_modMultiple.as_mut_ptr(),
            ) != 0
            {
                vli_sub(
                    l_product
                        .as_mut_ptr()
                        .offset((32 as libc::c_int / 8 as libc::c_int) as isize),
                    l_product
                        .as_mut_ptr()
                        .offset((32 as libc::c_int / 8 as libc::c_int) as isize),
                    p_result,
                );
            }
            vli_sub(
                l_product
                    .as_mut_ptr()
                    .offset((32 as libc::c_int / 8 as libc::c_int) as isize),
                l_product
                    .as_mut_ptr()
                    .offset((32 as libc::c_int / 8 as libc::c_int) as isize),
                l_modMultiple
                    .as_mut_ptr()
                    .offset((32 as libc::c_int / 8 as libc::c_int) as isize),
            );
        }
        let mut l_carry: uint64_t = (l_modMultiple
            [(32 as libc::c_int / 8 as libc::c_int) as usize]
            & 0x1 as libc::c_int as uint64_t)
            << 63 as libc::c_int;
        vli_rshift1(
            l_modMultiple
                .as_mut_ptr()
                .offset((32 as libc::c_int / 8 as libc::c_int) as isize),
        );
        vli_rshift1(l_modMultiple.as_mut_ptr());
        l_modMultiple[(32 as libc::c_int / 8 as libc::c_int - 1 as libc::c_int) as usize] |=
            l_carry;
        l_productBits = l_productBits.wrapping_sub(1);
    }
    vli_set(p_result, l_product.as_mut_ptr());
}
unsafe extern "C" fn umax(mut a: uint, mut b: uint) -> uint {
    return if a > b { a } else { b };
}
#[no_mangle]
pub unsafe extern "C" fn ecdsa_sign(
    mut p_privateKey: *const uint8_t,
    mut p_hash: *const uint8_t,
    mut p_signature: *mut uint8_t,
) -> libc::c_int {
    let mut k: [uint64_t; 4] = [0; 4];
    let mut l_tmp: [uint64_t; 4] = [0; 4];
    let mut l_s: [uint64_t; 4] = [0; 4];
    let mut p: EccPoint = EccPoint {
        x: [0; 4],
        y: [0; 4],
    };
    let mut l_tries: libc::c_uint = 0 as libc::c_int as libc::c_uint;
    loop {
        if getRandomNumber(k.as_mut_ptr()) == 0 || {
            let fresh2 = l_tries;
            l_tries = l_tries.wrapping_add(1);
            fresh2 >= 16 as libc::c_int as libc::c_uint
        } {
            return 0 as libc::c_int;
        }
        if !(vli_isZero(k.as_mut_ptr()) != 0) {
            if vli_cmp(curve_n.as_mut_ptr(), k.as_mut_ptr()) != 1 as libc::c_int {
                vli_sub(k.as_mut_ptr(), k.as_mut_ptr(), curve_n.as_mut_ptr());
            }
            EccPoint_mult(
                &mut p,
                addr_of_mut!(curve_G),
                k.as_mut_ptr(),
                0 as *mut uint64_t,
            );
            if vli_cmp(curve_n.as_mut_ptr(), (p.x).as_mut_ptr()) != 1 as libc::c_int {
                vli_sub((p.x).as_mut_ptr(), (p.x).as_mut_ptr(), curve_n.as_mut_ptr());
            }
        }
        if !(vli_isZero((p.x).as_mut_ptr()) != 0) {
            break;
        }
    }
    ecc_native2bytes(p_signature, (p.x).as_mut_ptr() as *const uint64_t);
    ecc_bytes2native(l_tmp.as_mut_ptr(), p_privateKey);
    vli_modMult(
        l_s.as_mut_ptr(),
        (p.x).as_mut_ptr(),
        l_tmp.as_mut_ptr(),
        curve_n.as_mut_ptr(),
    );
    ecc_bytes2native(l_tmp.as_mut_ptr(), p_hash);
    vli_modAdd(
        l_s.as_mut_ptr(),
        l_tmp.as_mut_ptr(),
        l_s.as_mut_ptr(),
        curve_n.as_mut_ptr(),
    );
    vli_modInv(k.as_mut_ptr(), k.as_mut_ptr(), curve_n.as_mut_ptr());
    vli_modMult(
        l_s.as_mut_ptr(),
        l_s.as_mut_ptr(),
        k.as_mut_ptr(),
        curve_n.as_mut_ptr(),
    );
    ecc_native2bytes(
        p_signature.offset(32 as libc::c_int as isize),
        l_s.as_mut_ptr() as *const uint64_t,
    );
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ecdsa_verify(
    mut p_publicKey: *const uint8_t,
    mut p_hash: *const uint8_t,
    mut p_signature: *const uint8_t,
) -> libc::c_int {
    let mut u1: [uint64_t; 4] = [0; 4];
    let mut u2: [uint64_t; 4] = [0; 4];
    let mut z: [uint64_t; 4] = [0; 4];
    let mut l_public: EccPoint = EccPoint {
        x: [0; 4],
        y: [0; 4],
    };
    let mut l_sum: EccPoint = EccPoint {
        x: [0; 4],
        y: [0; 4],
    };
    let mut rx: [uint64_t; 4] = [0; 4];
    let mut ry: [uint64_t; 4] = [0; 4];
    let mut tx: [uint64_t; 4] = [0; 4];
    let mut ty: [uint64_t; 4] = [0; 4];
    let mut tz: [uint64_t; 4] = [0; 4];
    let mut i: uint32_t = 0 as libc::c_int as uint32_t;
    let mut l_r: [uint64_t; 4] = [0; 4];
    let mut l_s: [uint64_t; 4] = [0; 4];
    ecc_point_decompress(&mut l_public, p_publicKey);
    ecc_bytes2native(l_r.as_mut_ptr(), p_signature);
    ecc_bytes2native(
        l_s.as_mut_ptr(),
        p_signature.offset(32 as libc::c_int as isize),
    );
    if vli_isZero(l_r.as_mut_ptr()) != 0 || vli_isZero(l_s.as_mut_ptr()) != 0 {
        return 0 as libc::c_int;
    }
    if vli_cmp(curve_n.as_mut_ptr(), l_r.as_mut_ptr()) != 1 as libc::c_int
        || vli_cmp(curve_n.as_mut_ptr(), l_s.as_mut_ptr()) != 1 as libc::c_int
    {
        return 0 as libc::c_int;
    }
    vli_modInv(z.as_mut_ptr(), l_s.as_mut_ptr(), curve_n.as_mut_ptr());
    ecc_bytes2native(u1.as_mut_ptr(), p_hash);
    vli_modMult(
        u1.as_mut_ptr(),
        u1.as_mut_ptr(),
        z.as_mut_ptr(),
        curve_n.as_mut_ptr(),
    );
    vli_modMult(
        u2.as_mut_ptr(),
        l_r.as_mut_ptr(),
        z.as_mut_ptr(),
        curve_n.as_mut_ptr(),
    );
    vli_set((l_sum.x).as_mut_ptr(), (l_public.x).as_mut_ptr());
    vli_set((l_sum.y).as_mut_ptr(), (l_public.y).as_mut_ptr());
    vli_set(tx.as_mut_ptr(), (curve_G.x).as_mut_ptr());
    vli_set(ty.as_mut_ptr(), (curve_G.y).as_mut_ptr());
    vli_modSub(
        z.as_mut_ptr(),
        (l_sum.x).as_mut_ptr(),
        tx.as_mut_ptr(),
        curve_p.as_mut_ptr(),
    );
    XYcZ_add(
        tx.as_mut_ptr(),
        ty.as_mut_ptr(),
        (l_sum.x).as_mut_ptr(),
        (l_sum.y).as_mut_ptr(),
    );
    vli_modInv(z.as_mut_ptr(), z.as_mut_ptr(), curve_p.as_mut_ptr());
    apply_z(
        (l_sum.x).as_mut_ptr(),
        (l_sum.y).as_mut_ptr(),
        z.as_mut_ptr(),
    );
    let mut l_points: [*mut EccPoint; 4] = [
        0 as *mut EccPoint,
        addr_of_mut!(curve_G),
        &mut l_public,
        &mut l_sum,
    ];
    let mut l_numBits: uint = umax(vli_numBits(u1.as_mut_ptr()), vli_numBits(u2.as_mut_ptr()));
    let mut l_point: *mut EccPoint = l_points[((vli_testBit(
        u1.as_mut_ptr(),
        l_numBits.wrapping_sub(1 as libc::c_int as uint),
    ) != 0) as libc::c_int
        | ((vli_testBit(
            u2.as_mut_ptr(),
            l_numBits.wrapping_sub(1 as libc::c_int as uint),
        ) != 0) as libc::c_int)
            << 1 as libc::c_int) as usize];
    vli_set(rx.as_mut_ptr(), ((*l_point).x).as_mut_ptr());
    vli_set(ry.as_mut_ptr(), ((*l_point).y).as_mut_ptr());
    vli_clear(z.as_mut_ptr());
    z[0 as libc::c_int as usize] = 1 as libc::c_int as uint64_t;
    i = l_numBits.wrapping_sub(2 as libc::c_int as uint);
    while i >= 0 as libc::c_int as uint32_t {
        EccPoint_double_jacobian(rx.as_mut_ptr(), ry.as_mut_ptr(), z.as_mut_ptr());
        let mut l_index: libc::c_int = (vli_testBit(u1.as_mut_ptr(), i) != 0) as libc::c_int
            | ((vli_testBit(u2.as_mut_ptr(), i) != 0) as libc::c_int) << 1 as libc::c_int;
        let mut l_point_0: *mut EccPoint = l_points[l_index as usize];
        if !l_point_0.is_null() {
            vli_set(tx.as_mut_ptr(), ((*l_point_0).x).as_mut_ptr());
            vli_set(ty.as_mut_ptr(), ((*l_point_0).y).as_mut_ptr());
            apply_z(tx.as_mut_ptr(), ty.as_mut_ptr(), z.as_mut_ptr());
            vli_modSub(
                tz.as_mut_ptr(),
                rx.as_mut_ptr(),
                tx.as_mut_ptr(),
                curve_p.as_mut_ptr(),
            );
            XYcZ_add(
                tx.as_mut_ptr(),
                ty.as_mut_ptr(),
                rx.as_mut_ptr(),
                ry.as_mut_ptr(),
            );
            vli_modMult_fast(z.as_mut_ptr(), z.as_mut_ptr(), tz.as_mut_ptr());
        }
        i = i.wrapping_sub(1);
    }
    vli_modInv(z.as_mut_ptr(), z.as_mut_ptr(), curve_p.as_mut_ptr());
    apply_z(rx.as_mut_ptr(), ry.as_mut_ptr(), z.as_mut_ptr());
    if vli_cmp(curve_n.as_mut_ptr(), rx.as_mut_ptr()) != 1 as libc::c_int {
        vli_sub(rx.as_mut_ptr(), rx.as_mut_ptr(), curve_n.as_mut_ptr());
    }
    return (vli_cmp(rx.as_mut_ptr(), l_r.as_mut_ptr()) == 0 as libc::c_int) as libc::c_int;
}
