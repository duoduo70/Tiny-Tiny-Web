/* Compact-X25519-rs
 * No Copyright 2024 Plasma (https://github.com/duoduo70/Compact-X25519-rs).
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

use std::ptr::{addr_of, addr_of_mut};
mod libc {
    pub type c_void = *mut u8;
    pub type c_ulong = u32;
    pub type c_uchar = u8;
    pub type c_ushort = u16;
    pub type c_uint = u32;
    pub type c_int = i32;
    pub type c_ulonglong = u64;
    pub type c_longlong = i64;
}
extern "C" {
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
pub type uint64_t = libc::c_ulonglong;
#[inline]
unsafe fn f25519_copy(x: *mut uint8_t, a: *const uint8_t) {
    memcpy(
        x as *mut libc::c_void,
        a as *const libc::c_void,
        32 as libc::c_ulong,
    );
}
static mut f25519_one: [uint8_t; 32] = [
    1 as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe fn f25519_load(x: *mut uint8_t, mut c: uint32_t) {
    let mut i: isize = 0;
    i = 0;
    while i < core::mem::size_of::<uint32_t>() as isize {
        *x.offset(i) = c as uint8_t;
        c >>= 8;
        i += 1;
    }
    while i < 32 {
        *x.offset(i) = 0;
        i += 1;
    }
}
unsafe fn f25519_normalize(x: *mut uint8_t) {
    let mut minusp: [uint8_t; 32] = [0; 32];
    let mut c: uint16_t = 0;
    let mut i: libc::c_int = 0;
    c = ((*x.offset(31 as libc::c_int as isize) as libc::c_int >> 7 as libc::c_int)
        * 19 as libc::c_int) as uint16_t;
    let fresh0 = &mut (*x.offset(31 as libc::c_int as isize));
    *fresh0 = (*fresh0 as libc::c_int & 127 as libc::c_int) as uint8_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        c = (c as libc::c_int + *x.offset(i as isize) as libc::c_int) as uint16_t;
        *x.offset(i as isize) = c as uint8_t;
        c = (c as libc::c_int >> 8 as libc::c_int) as uint16_t;
        i += 1;
    }
    c = 19 as libc::c_int as uint16_t;
    i = 0 as libc::c_int;
    while (i + 1 as libc::c_int) < 32 as libc::c_int {
        c = (c as libc::c_int + *x.offset(i as isize) as libc::c_int) as uint16_t;
        minusp[i as usize] = c as uint8_t;
        c = (c as libc::c_int >> 8 as libc::c_int) as uint16_t;
        i += 1;
    }
    c = (c as libc::c_int + (*x.offset(i as isize) as uint16_t as libc::c_int - 128 as libc::c_int))
        as uint16_t;
    minusp[31 as libc::c_int as usize] = c as uint8_t;
    f25519_select(
        x,
        minusp.as_mut_ptr(),
        x,
        (c as libc::c_int >> 15 as libc::c_int & 1 as libc::c_int) as uint8_t,
    );
}
unsafe fn f25519_eq(x: *const uint8_t, y: *const uint8_t) -> uint8_t {
    let mut sum: uint8_t = 0 as libc::c_int as uint8_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        sum = (sum as libc::c_int
            | *x.offset(i as isize) as libc::c_int ^ *y.offset(i as isize) as libc::c_int)
            as uint8_t;
        i += 1;
    }
    sum = (sum as libc::c_int | sum as libc::c_int >> 4 as libc::c_int) as uint8_t;
    sum = (sum as libc::c_int | sum as libc::c_int >> 2 as libc::c_int) as uint8_t;
    sum = (sum as libc::c_int | sum as libc::c_int >> 1 as libc::c_int) as uint8_t;

    ((sum as libc::c_int ^ 1 as libc::c_int) & 1 as libc::c_int) as uint8_t
}
unsafe fn f25519_select(
    dst: *mut uint8_t,
    zero: *const uint8_t,
    one: *const uint8_t,
    condition: uint8_t,
) {
    let mask: uint8_t = -(condition as libc::c_int) as uint8_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        *dst.offset(i as isize) = (*zero.offset(i as isize) as libc::c_int
            ^ mask as libc::c_int
                & (*one.offset(i as isize) as libc::c_int
                    ^ *zero.offset(i as isize) as libc::c_int))
            as uint8_t;
        i += 1;
    }
}
unsafe fn f25519_add(r: *mut uint8_t, a: *const uint8_t, b: *const uint8_t) {
    let mut c: uint16_t = 0 as libc::c_int as uint16_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        c = (c as libc::c_int >> 8 as libc::c_int) as uint16_t;
        c = (c as libc::c_int
            + (*a.offset(i as isize) as uint16_t as libc::c_int
                + *b.offset(i as isize) as uint16_t as libc::c_int)) as uint16_t;
        *r.offset(i as isize) = c as uint8_t;
        i += 1;
    }
    let fresh1 = &mut (*r.offset(31 as libc::c_int as isize));
    *fresh1 = (*fresh1 as libc::c_int & 127 as libc::c_int) as uint8_t;
    c = ((c as libc::c_int >> 7 as libc::c_int) * 19 as libc::c_int) as uint16_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        c = (c as libc::c_int + *r.offset(i as isize) as libc::c_int) as uint16_t;
        *r.offset(i as isize) = c as uint8_t;
        c = (c as libc::c_int >> 8 as libc::c_int) as uint16_t;
        i += 1;
    }
}
unsafe fn f25519_sub(r: *mut uint8_t, a: *const uint8_t, b: *const uint8_t) {
    let mut c: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: libc::c_int = 0;
    c = 218 as libc::c_int as uint32_t;
    i = 0 as libc::c_int;
    while (i + 1 as libc::c_int) < 32 as libc::c_int {
        c = (c as libc::c_uint).wrapping_add(
            (65280 as libc::c_int as libc::c_uint)
                .wrapping_add(*a.offset(i as isize) as uint32_t)
                .wrapping_sub(*b.offset(i as isize) as uint32_t),
        ) as uint32_t as uint32_t;
        *r.offset(i as isize) = c as uint8_t;
        c >>= 8 as libc::c_int;
        i += 1;
    }
    c = (c as libc::c_uint).wrapping_add(
        (*a.offset(31 as libc::c_int as isize) as uint32_t)
            .wrapping_sub(*b.offset(31 as libc::c_int as isize) as uint32_t),
    ) as uint32_t as uint32_t;
    *r.offset(31 as libc::c_int as isize) = (c & 127 as libc::c_int as libc::c_uint) as uint8_t;
    c = (c >> 7 as libc::c_int).wrapping_mul(19 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        c = (c as libc::c_uint).wrapping_add(*r.offset(i as isize) as libc::c_uint) as uint32_t
            as uint32_t;
        *r.offset(i as isize) = c as uint8_t;
        c >>= 8 as libc::c_int;
        i += 1;
    }
}
unsafe fn f25519_neg(r: *mut uint8_t, a: *const uint8_t) {
    let mut c: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: libc::c_int = 0;
    c = 218 as libc::c_int as uint32_t;
    i = 0 as libc::c_int;
    while (i + 1 as libc::c_int) < 32 as libc::c_int {
        c = (c as libc::c_uint).wrapping_add(
            (65280 as libc::c_int as libc::c_uint).wrapping_sub(*a.offset(i as isize) as uint32_t),
        ) as uint32_t as uint32_t;
        *r.offset(i as isize) = c as uint8_t;
        c >>= 8 as libc::c_int;
        i += 1;
    }
    c = (c as libc::c_uint).wrapping_sub(*a.offset(31 as libc::c_int as isize) as uint32_t)
        as uint32_t as uint32_t;
    *r.offset(31 as libc::c_int as isize) = (c & 127 as libc::c_int as libc::c_uint) as uint8_t;
    c = (c >> 7 as libc::c_int).wrapping_mul(19 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        c = (c as libc::c_uint).wrapping_add(*r.offset(i as isize) as libc::c_uint) as uint32_t
            as uint32_t;
        *r.offset(i as isize) = c as uint8_t;
        c >>= 8 as libc::c_int;
        i += 1;
    }
}
unsafe fn f25519_mul__distinct(r: *mut uint8_t, a: *const uint8_t, b: *const uint8_t) {
    let mut c: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        let mut j: libc::c_int = 0;
        c >>= 8 as libc::c_int;
        j = 0 as libc::c_int;
        while j <= i {
            c = (c as libc::c_uint).wrapping_add(
                (*a.offset(j as isize) as uint32_t)
                    .wrapping_mul(*b.offset((i - j) as isize) as uint32_t),
            ) as uint32_t as uint32_t;
            j += 1;
        }
        while j < 32 as libc::c_int {
            c = (c as libc::c_uint).wrapping_add(
                (*a.offset(j as isize) as uint32_t)
                    .wrapping_mul(*b.offset((i + 32 as libc::c_int - j) as isize) as uint32_t)
                    .wrapping_mul(38 as libc::c_int as libc::c_uint),
            ) as uint32_t as uint32_t;
            j += 1;
        }
        *r.offset(i as isize) = c as uint8_t;
        i += 1;
    }
    let fresh2 = &mut (*r.offset(31 as libc::c_int as isize));
    *fresh2 = (*fresh2 as libc::c_int & 127 as libc::c_int) as uint8_t;
    c = (c >> 7 as libc::c_int).wrapping_mul(19 as libc::c_int as libc::c_uint);
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        c = (c as libc::c_uint).wrapping_add(*r.offset(i as isize) as libc::c_uint) as uint32_t
            as uint32_t;
        *r.offset(i as isize) = c as uint8_t;
        c >>= 8 as libc::c_int;
        i += 1;
    }
}
unsafe fn f25519_mul_c(r: *mut uint8_t, a: *const uint8_t, b: uint32_t) {
    let mut c: uint32_t = 0 as libc::c_int as uint32_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        c >>= 8 as libc::c_int;
        c = (c as libc::c_uint).wrapping_add(b.wrapping_mul(*a.offset(i as isize) as uint32_t))
            as uint32_t as uint32_t;
        *r.offset(i as isize) = c as uint8_t;
        i += 1;
    }
    let fresh3 = &mut (*r.offset(31 as libc::c_int as isize));
    *fresh3 = (*fresh3 as libc::c_int & 127 as libc::c_int) as uint8_t;
    c >>= 7 as libc::c_int;
    c = (c as libc::c_uint).wrapping_mul(19 as libc::c_int as libc::c_uint) as uint32_t as uint32_t;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        c = (c as libc::c_uint).wrapping_add(*r.offset(i as isize) as libc::c_uint) as uint32_t
            as uint32_t;
        *r.offset(i as isize) = c as uint8_t;
        c >>= 8 as libc::c_int;
        i += 1;
    }
}
unsafe fn f25519_inv__distinct(r: *mut uint8_t, x: *const uint8_t) {
    let mut s: [uint8_t; 32] = [0; 32];
    let mut i: libc::c_int = 0;
    f25519_mul__distinct(s.as_mut_ptr(), x, x);
    f25519_mul__distinct(r, s.as_mut_ptr(), x);
    i = 0 as libc::c_int;
    while i < 248 as libc::c_int {
        f25519_mul__distinct(s.as_mut_ptr(), r, r);
        f25519_mul__distinct(r, s.as_mut_ptr(), x);
        i += 1;
    }
    f25519_mul__distinct(s.as_mut_ptr(), r, r);
    f25519_mul__distinct(r, s.as_mut_ptr(), s.as_mut_ptr());
    f25519_mul__distinct(s.as_mut_ptr(), r, x);
    f25519_mul__distinct(r, s.as_mut_ptr(), s.as_mut_ptr());
    f25519_mul__distinct(s.as_mut_ptr(), r, r);
    f25519_mul__distinct(r, s.as_mut_ptr(), x);
    f25519_mul__distinct(s.as_mut_ptr(), r, r);
    f25519_mul__distinct(r, s.as_mut_ptr(), x);
}
unsafe fn exp2523(r: *mut uint8_t, x: *const uint8_t, s: *mut uint8_t) {
    let mut i: libc::c_int = 0;
    f25519_mul__distinct(r, x, x);
    f25519_mul__distinct(s, r, x);
    i = 0 as libc::c_int;
    while i < 248 as libc::c_int {
        f25519_mul__distinct(r, s, s);
        f25519_mul__distinct(s, r, x);
        i += 1;
    }
    f25519_mul__distinct(r, s, s);
    f25519_mul__distinct(s, r, r);
    f25519_mul__distinct(r, s, x);
}
unsafe fn f25519_sqrt(r: *mut uint8_t, a: *const uint8_t) {
    let mut v: [uint8_t; 32] = [0; 32];
    let mut i: [uint8_t; 32] = [0; 32];
    let mut x: [uint8_t; 32] = [0; 32];
    let mut y: [uint8_t; 32] = [0; 32];
    f25519_mul_c(x.as_mut_ptr(), a, 2 as libc::c_int as uint32_t);
    exp2523(v.as_mut_ptr(), x.as_mut_ptr(), y.as_mut_ptr());
    f25519_mul__distinct(y.as_mut_ptr(), v.as_mut_ptr(), v.as_mut_ptr());
    f25519_mul__distinct(i.as_mut_ptr(), x.as_mut_ptr(), y.as_mut_ptr());
    f25519_load(y.as_mut_ptr(), 1 as libc::c_int as uint32_t);
    f25519_sub(i.as_mut_ptr(), i.as_mut_ptr(), y.as_mut_ptr());
    f25519_mul__distinct(x.as_mut_ptr(), v.as_mut_ptr(), a);
    f25519_mul__distinct(r, x.as_mut_ptr(), i.as_mut_ptr());
}
#[inline]
unsafe fn c25519_prepare(key: *mut uint8_t) {
    let fresh4 = &mut (*key.offset(0 as libc::c_int as isize));
    *fresh4 = (*fresh4 as libc::c_int & 0xf8 as libc::c_int) as uint8_t;
    let fresh5 = &mut (*key.offset(31 as libc::c_int as isize));
    *fresh5 = (*fresh5 as libc::c_int & 0x7f as libc::c_int) as uint8_t;
    let fresh6 = &mut (*key.offset(31 as libc::c_int as isize));
    *fresh6 = (*fresh6 as libc::c_int | 0x40 as libc::c_int) as uint8_t;
}
static mut c25519_base_x: [uint8_t; 32] = [
    9 as libc::c_int as uint8_t,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
unsafe fn xc_double(
    x3: *mut uint8_t,
    z3: *mut uint8_t,
    x1: *const uint8_t,
    z1: *const uint8_t,
) {
    let mut x1sq: [uint8_t; 32] = [0; 32];
    let mut z1sq: [uint8_t; 32] = [0; 32];
    let mut x1z1: [uint8_t; 32] = [0; 32];
    let mut a: [uint8_t; 32] = [0; 32];
    f25519_mul__distinct(x1sq.as_mut_ptr(), x1, x1);
    f25519_mul__distinct(z1sq.as_mut_ptr(), z1, z1);
    f25519_mul__distinct(x1z1.as_mut_ptr(), x1, z1);
    f25519_sub(a.as_mut_ptr(), x1sq.as_mut_ptr(), z1sq.as_mut_ptr());
    f25519_mul__distinct(x3, a.as_mut_ptr(), a.as_mut_ptr());
    f25519_mul_c(
        a.as_mut_ptr(),
        x1z1.as_mut_ptr(),
        486662 as libc::c_int as uint32_t,
    );
    f25519_add(a.as_mut_ptr(), x1sq.as_mut_ptr(), a.as_mut_ptr());
    f25519_add(a.as_mut_ptr(), z1sq.as_mut_ptr(), a.as_mut_ptr());
    f25519_mul__distinct(x1sq.as_mut_ptr(), x1z1.as_mut_ptr(), a.as_mut_ptr());
    f25519_mul_c(z3, x1sq.as_mut_ptr(), 4 as libc::c_int as uint32_t);
}
unsafe fn xc_diffadd(
    x5: *mut uint8_t,
    z5: *mut uint8_t,
    x1: *const uint8_t,
    z1: *const uint8_t,
    x2: *const uint8_t,
    z2: *const uint8_t,
    x3: *const uint8_t,
    z3: *const uint8_t,
) {
    let mut da: [uint8_t; 32] = [0; 32];
    let mut cb: [uint8_t; 32] = [0; 32];
    let mut a: [uint8_t; 32] = [0; 32];
    let mut b: [uint8_t; 32] = [0; 32];
    f25519_add(a.as_mut_ptr(), x2, z2);
    f25519_sub(b.as_mut_ptr(), x3, z3);
    f25519_mul__distinct(da.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    f25519_sub(b.as_mut_ptr(), x2, z2);
    f25519_add(a.as_mut_ptr(), x3, z3);
    f25519_mul__distinct(cb.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    f25519_add(a.as_mut_ptr(), da.as_mut_ptr(), cb.as_mut_ptr());
    f25519_mul__distinct(b.as_mut_ptr(), a.as_mut_ptr(), a.as_mut_ptr());
    f25519_mul__distinct(x5, z1, b.as_mut_ptr());
    f25519_sub(a.as_mut_ptr(), da.as_mut_ptr(), cb.as_mut_ptr());
    f25519_mul__distinct(b.as_mut_ptr(), a.as_mut_ptr(), a.as_mut_ptr());
    f25519_mul__distinct(z5, x1, b.as_mut_ptr());
}
unsafe fn c25519_smult(result: *mut uint8_t, q: *const uint8_t, e: *const uint8_t) {
    let mut xm: [uint8_t; 32] = [0; 32];
    let mut zm: [uint8_t; 32] = [
        1 as libc::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut xm1: [uint8_t; 32] = [
        1 as libc::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut zm1: [uint8_t; 32] = [
        0 as libc::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let mut i: libc::c_int = 0;
    f25519_copy(xm.as_mut_ptr(), q);
    i = 253 as libc::c_int;
    while i >= 0 as libc::c_int {
        let bit: libc::c_int = *e.offset((i >> 3 as libc::c_int) as isize) as libc::c_int
            >> (i & 7 as libc::c_int)
            & 1 as libc::c_int;
        let mut xms: [uint8_t; 32] = [0; 32];
        let mut zms: [uint8_t; 32] = [0; 32];
        xc_diffadd(
            xm1.as_mut_ptr(),
            zm1.as_mut_ptr(),
            q,
            f25519_one.as_ptr(),
            xm.as_mut_ptr(),
            zm.as_mut_ptr(),
            xm1.as_mut_ptr(),
            zm1.as_mut_ptr(),
        );
        xc_double(
            xm.as_mut_ptr(),
            zm.as_mut_ptr(),
            xm.as_mut_ptr(),
            zm.as_mut_ptr(),
        );
        xc_diffadd(
            xms.as_mut_ptr(),
            zms.as_mut_ptr(),
            xm1.as_mut_ptr(),
            zm1.as_mut_ptr(),
            xm.as_mut_ptr(),
            zm.as_mut_ptr(),
            q,
            f25519_one.as_ptr(),
        );
        f25519_select(
            xm1.as_mut_ptr(),
            xm1.as_mut_ptr(),
            xm.as_mut_ptr(),
            bit as uint8_t,
        );
        f25519_select(
            zm1.as_mut_ptr(),
            zm1.as_mut_ptr(),
            zm.as_mut_ptr(),
            bit as uint8_t,
        );
        f25519_select(
            xm.as_mut_ptr(),
            xm.as_mut_ptr(),
            xms.as_mut_ptr(),
            bit as uint8_t,
        );
        f25519_select(
            zm.as_mut_ptr(),
            zm.as_mut_ptr(),
            zms.as_mut_ptr(),
            bit as uint8_t,
        );
        i -= 1;
    }
    f25519_inv__distinct(zm1.as_mut_ptr(), zm.as_mut_ptr());
    f25519_mul__distinct(result, zm1.as_mut_ptr(), xm.as_mut_ptr());
    f25519_normalize(result);
}
unsafe fn compact_wipe(data: *mut libc::c_void, mut length: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_uchar = data as *mut libc::c_uchar;
    loop {
        let fresh7 = length;
        length = length.wrapping_sub(1);
        if fresh7 == 0 {
            break;
        }
        let fresh8 = p;
        p = p.offset(1);
        core::ptr::write_volatile(fresh8, 0 as libc::c_int as libc::c_uchar);
    }
    data
}
pub unsafe fn compact_x25519_keygen(
    private_key: *mut uint8_t,
    public_key: *mut uint8_t,
    random_seed: *mut uint8_t,
) {
    memcpy(
        private_key as *mut libc::c_void,
        random_seed as *const libc::c_void,
        32 as libc::c_int as libc::c_ulong,
    );
    compact_wipe(
        random_seed as *mut libc::c_void,
        32 as libc::c_int as size_t,
    );
    c25519_prepare(private_key);
    c25519_smult(
        public_key,
        c25519_base_x.as_ptr(),
        private_key as *const uint8_t,
    );
}
pub unsafe fn compact_x25519_shared(
    shared_secret: *mut uint8_t,
    my_private_key: *const uint8_t,
    their_public_key: *const uint8_t,
) {
    let mut clamped_private_key: [uint8_t; 32] = [0; 32];
    memcpy(
        clamped_private_key.as_mut_ptr() as *mut libc::c_void,
        my_private_key as *const libc::c_void,
        32 as libc::c_int as libc::c_ulong,
    );
    c25519_prepare(clamped_private_key.as_mut_ptr());
    c25519_smult(
        shared_secret,
        their_public_key,
        clamped_private_key.as_mut_ptr(),
    );
    compact_wipe(
        clamped_private_key.as_mut_ptr() as *mut libc::c_void,
        32 as libc::c_int as size_t,
    );
}

pub fn key_to_vec(key: *mut u8, length: u8) -> Vec<u8> {
    let mut i = 0;
    let mut vec = vec![];
    while i < length {
        vec.push(unsafe { *key.wrapping_add(i.into()) });
        i += 1;
    }
    vec
}

#[derive(Copy, Clone)]
#[repr(C)]
struct sha512_state {
    pub h: [uint64_t; 8],
}
#[no_mangle]
static mut sha512_initial_state: sha512_state = {
    
    sha512_state {
        h: [
            0x6a09e667f3bcc908 as libc::c_longlong as uint64_t,
            0xbb67ae8584caa73b as libc::c_ulonglong as uint64_t,
            0x3c6ef372fe94f82b as libc::c_longlong as uint64_t,
            0xa54ff53a5f1d36f1 as libc::c_ulonglong as uint64_t,
            0x510e527fade682d1 as libc::c_longlong as uint64_t,
            0x9b05688c2b3e6c1f as libc::c_ulonglong as uint64_t,
            0x1f83d9abfb41bd6b as libc::c_longlong as uint64_t,
            0x5be0cd19137e2179 as libc::c_longlong as uint64_t,
        ],
    }
};
static mut round_k: [uint64_t; 80] = [
    0x428a2f98d728ae22 as libc::c_longlong as uint64_t,
    0x7137449123ef65cd as libc::c_longlong as uint64_t,
    0xb5c0fbcfec4d3b2f as libc::c_ulonglong as uint64_t,
    0xe9b5dba58189dbbc as libc::c_ulonglong as uint64_t,
    0x3956c25bf348b538 as libc::c_longlong as uint64_t,
    0x59f111f1b605d019 as libc::c_longlong as uint64_t,
    0x923f82a4af194f9b as libc::c_ulonglong as uint64_t,
    0xab1c5ed5da6d8118 as libc::c_ulonglong as uint64_t,
    0xd807aa98a3030242 as libc::c_ulonglong as uint64_t,
    0x12835b0145706fbe as libc::c_longlong as uint64_t,
    0x243185be4ee4b28c as libc::c_longlong as uint64_t,
    0x550c7dc3d5ffb4e2 as libc::c_longlong as uint64_t,
    0x72be5d74f27b896f as libc::c_longlong as uint64_t,
    0x80deb1fe3b1696b1 as libc::c_ulonglong as uint64_t,
    0x9bdc06a725c71235 as libc::c_ulonglong as uint64_t,
    0xc19bf174cf692694 as libc::c_ulonglong as uint64_t,
    0xe49b69c19ef14ad2 as libc::c_ulonglong as uint64_t,
    0xefbe4786384f25e3 as libc::c_ulonglong as uint64_t,
    0xfc19dc68b8cd5b5 as libc::c_longlong as uint64_t,
    0x240ca1cc77ac9c65 as libc::c_longlong as uint64_t,
    0x2de92c6f592b0275 as libc::c_longlong as uint64_t,
    0x4a7484aa6ea6e483 as libc::c_longlong as uint64_t,
    0x5cb0a9dcbd41fbd4 as libc::c_longlong as uint64_t,
    0x76f988da831153b5 as libc::c_longlong as uint64_t,
    0x983e5152ee66dfab as libc::c_ulonglong as uint64_t,
    0xa831c66d2db43210 as libc::c_ulonglong as uint64_t,
    0xb00327c898fb213f as libc::c_ulonglong as uint64_t,
    0xbf597fc7beef0ee4 as libc::c_ulonglong as uint64_t,
    0xc6e00bf33da88fc2 as libc::c_ulonglong as uint64_t,
    0xd5a79147930aa725 as libc::c_ulonglong as uint64_t,
    0x6ca6351e003826f as libc::c_longlong as uint64_t,
    0x142929670a0e6e70 as libc::c_longlong as uint64_t,
    0x27b70a8546d22ffc as libc::c_longlong as uint64_t,
    0x2e1b21385c26c926 as libc::c_longlong as uint64_t,
    0x4d2c6dfc5ac42aed as libc::c_longlong as uint64_t,
    0x53380d139d95b3df as libc::c_longlong as uint64_t,
    0x650a73548baf63de as libc::c_longlong as uint64_t,
    0x766a0abb3c77b2a8 as libc::c_longlong as uint64_t,
    0x81c2c92e47edaee6 as libc::c_ulonglong as uint64_t,
    0x92722c851482353b as libc::c_ulonglong as uint64_t,
    0xa2bfe8a14cf10364 as libc::c_ulonglong as uint64_t,
    0xa81a664bbc423001 as libc::c_ulonglong as uint64_t,
    0xc24b8b70d0f89791 as libc::c_ulonglong as uint64_t,
    0xc76c51a30654be30 as libc::c_ulonglong as uint64_t,
    0xd192e819d6ef5218 as libc::c_ulonglong as uint64_t,
    0xd69906245565a910 as libc::c_ulonglong as uint64_t,
    0xf40e35855771202a as libc::c_ulonglong as uint64_t,
    0x106aa07032bbd1b8 as libc::c_longlong as uint64_t,
    0x19a4c116b8d2d0c8 as libc::c_longlong as uint64_t,
    0x1e376c085141ab53 as libc::c_longlong as uint64_t,
    0x2748774cdf8eeb99 as libc::c_longlong as uint64_t,
    0x34b0bcb5e19b48a8 as libc::c_longlong as uint64_t,
    0x391c0cb3c5c95a63 as libc::c_longlong as uint64_t,
    0x4ed8aa4ae3418acb as libc::c_longlong as uint64_t,
    0x5b9cca4f7763e373 as libc::c_longlong as uint64_t,
    0x682e6ff3d6b2b8a3 as libc::c_longlong as uint64_t,
    0x748f82ee5defb2fc as libc::c_longlong as uint64_t,
    0x78a5636f43172f60 as libc::c_longlong as uint64_t,
    0x84c87814a1f0ab72 as libc::c_ulonglong as uint64_t,
    0x8cc702081a6439ec as libc::c_ulonglong as uint64_t,
    0x90befffa23631e28 as libc::c_ulonglong as uint64_t,
    0xa4506cebde82bde9 as libc::c_ulonglong as uint64_t,
    0xbef9a3f7b2c67915 as libc::c_ulonglong as uint64_t,
    0xc67178f2e372532b as libc::c_ulonglong as uint64_t,
    0xca273eceea26619c as libc::c_ulonglong as uint64_t,
    0xd186b8c721c0c207 as libc::c_ulonglong as uint64_t,
    0xeada7dd6cde0eb1e as libc::c_ulonglong as uint64_t,
    0xf57d4f7fee6ed178 as libc::c_ulonglong as uint64_t,
    0x6f067aa72176fba as libc::c_longlong as uint64_t,
    0xa637dc5a2c898a6 as libc::c_longlong as uint64_t,
    0x113f9804bef90dae as libc::c_longlong as uint64_t,
    0x1b710b35131c471b as libc::c_longlong as uint64_t,
    0x28db77f523047d84 as libc::c_longlong as uint64_t,
    0x32caab7b40c72493 as libc::c_longlong as uint64_t,
    0x3c9ebe0a15c9bebc as libc::c_longlong as uint64_t,
    0x431d67c49c100d4c as libc::c_longlong as uint64_t,
    0x4cc5d4becb3e42b6 as libc::c_longlong as uint64_t,
    0x597f299cfc657e2a as libc::c_longlong as uint64_t,
    0x5fcb6fab3ad6faec as libc::c_longlong as uint64_t,
    0x6c44198c4a475817 as libc::c_longlong as uint64_t,
];
#[inline]
unsafe extern "C" fn load64(mut x: *const uint8_t) -> uint64_t {
    let mut r: uint64_t = 0;
    let fresh0 = x;
    x = x.offset(1);
    r = *fresh0 as uint64_t;
    let fresh1 = x;
    x = x.offset(1);
    r = r << 8 as libc::c_int | *fresh1 as u64;
    let fresh2 = x;
    x = x.offset(1);
    r = r << 8 as libc::c_int | *fresh2 as u64;
    let fresh3 = x;
    x = x.offset(1);
    r = r << 8 as libc::c_int | *fresh3 as u64;
    let fresh4 = x;
    x = x.offset(1);
    r = r << 8 as libc::c_int | *fresh4 as u64;
    let fresh5 = x;
    x = x.offset(1);
    r = r << 8 as libc::c_int | *fresh5 as u64;
    let fresh6 = x;
    x = x.offset(1);
    r = r << 8 as libc::c_int | *fresh6 as u64;
    let fresh7 = x;
    x = x.offset(1);
    r = r << 8 as libc::c_int | *fresh7 as u64;

    r
}
#[inline]
unsafe extern "C" fn store64(mut x: *mut uint8_t, mut v: uint64_t) {
    x = x.offset(7 as libc::c_int as isize);
    let fresh8 = x;
    x = x.offset(-1);
    *fresh8 = v as uint8_t;
    v >>= 8 as libc::c_int;
    let fresh9 = x;
    x = x.offset(-1);
    *fresh9 = v as uint8_t;
    v >>= 8 as libc::c_int;
    let fresh10 = x;
    x = x.offset(-1);
    *fresh10 = v as uint8_t;
    v >>= 8 as libc::c_int;
    let fresh11 = x;
    x = x.offset(-1);
    *fresh11 = v as uint8_t;
    v >>= 8 as libc::c_int;
    let fresh12 = x;
    x = x.offset(-1);
    *fresh12 = v as uint8_t;
    v >>= 8 as libc::c_int;
    let fresh13 = x;
    x = x.offset(-1);
    *fresh13 = v as uint8_t;
    v >>= 8 as libc::c_int;
    let fresh14 = x;
    x = x.offset(-1);
    *fresh14 = v as uint8_t;
    v >>= 8 as libc::c_int;
    let fresh15 = x;
    x = x.offset(-1);
    *fresh15 = v as uint8_t;
}
#[inline]
unsafe extern "C" fn rot64(x: uint64_t, bits: libc::c_int) -> uint64_t {
    x >> bits | x << (64 as libc::c_int - bits)
}
#[no_mangle]
unsafe extern "C" fn sha512_block(s: *mut sha512_state, mut blk: *const uint8_t) {
    let mut w: [uint64_t; 16] = [0; 16];
    let mut a: uint64_t = 0;
    let mut b: uint64_t = 0;
    let mut c: uint64_t = 0;
    let mut d: uint64_t = 0;
    let mut e: uint64_t = 0;
    let mut f: uint64_t = 0;
    let mut g: uint64_t = 0;
    let mut h: uint64_t = 0;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        w[i as usize] = load64(blk);
        blk = blk.offset(8 as libc::c_int as isize);
        i += 1;
    }
    a = (*s).h[0 as libc::c_int as usize];
    b = (*s).h[1 as libc::c_int as usize];
    c = (*s).h[2 as libc::c_int as usize];
    d = (*s).h[3 as libc::c_int as usize];
    e = (*s).h[4 as libc::c_int as usize];
    f = (*s).h[5 as libc::c_int as usize];
    g = (*s).h[6 as libc::c_int as usize];
    h = (*s).h[7 as libc::c_int as usize];
    i = 0 as libc::c_int;
    while i < 80 as libc::c_int {
        let wi: uint64_t = w[(i & 15 as libc::c_int) as usize];
        let wi15: uint64_t = w[((i + 1 as libc::c_int) & 15 as libc::c_int) as usize];
        let wi2: uint64_t = w[((i + 14 as libc::c_int) & 15 as libc::c_int) as usize];
        let wi7: uint64_t = w[((i + 9 as libc::c_int) & 15 as libc::c_int) as usize];
        let s0: uint64_t = rot64(wi15, 1 as libc::c_int)
            ^ rot64(wi15, 8 as libc::c_int)
            ^ wi15 >> 7 as libc::c_int;
        let s1: uint64_t =
            rot64(wi2, 19 as libc::c_int) ^ rot64(wi2, 61 as libc::c_int) ^ wi2 >> 6 as libc::c_int;
        let S0: uint64_t =
            rot64(a, 28 as libc::c_int) ^ rot64(a, 34 as libc::c_int) ^ rot64(a, 39 as libc::c_int);
        let S1: uint64_t =
            rot64(e, 14 as libc::c_int) ^ rot64(e, 18 as libc::c_int) ^ rot64(e, 41 as libc::c_int);
        let ch: uint64_t = e & f ^ !e & g;
        let temp1: uint64_t = h
            .wrapping_add(S1)
            .wrapping_add(ch)
            .wrapping_add(round_k[i as usize])
            .wrapping_add(wi);
        let maj: uint64_t = a & b ^ a & c ^ b & c;
        let temp2: uint64_t = S0.wrapping_add(maj);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
        w[(i & 15 as libc::c_int) as usize] =
            wi.wrapping_add(s0).wrapping_add(wi7).wrapping_add(s1);
        i += 1;
    }
    let fresh16 = &mut (*s).h[0 as libc::c_int as usize];
    *fresh16 = (*fresh16 as libc::c_ulong).wrapping_add(a as u32) as uint64_t as uint64_t;
    let fresh17 = &mut (*s).h[1 as libc::c_int as usize];
    *fresh17 = (*fresh17 as libc::c_ulong).wrapping_add(b as u32) as uint64_t as uint64_t;
    let fresh18 = &mut (*s).h[2 as libc::c_int as usize];
    *fresh18 = (*fresh18 as libc::c_ulong).wrapping_add(c as u32) as uint64_t as uint64_t;
    let fresh19 = &mut (*s).h[3 as libc::c_int as usize];
    *fresh19 = (*fresh19 as libc::c_ulong).wrapping_add(d as u32) as uint64_t as uint64_t;
    let fresh20 = &mut (*s).h[4 as libc::c_int as usize];
    *fresh20 = (*fresh20 as libc::c_ulong).wrapping_add(e as u32) as uint64_t as uint64_t;
    let fresh21 = &mut (*s).h[5 as libc::c_int as usize];
    *fresh21 = (*fresh21 as libc::c_ulong).wrapping_add(f as u32) as uint64_t as uint64_t;
    let fresh22 = &mut (*s).h[6 as libc::c_int as usize];
    *fresh22 = (*fresh22 as libc::c_ulong).wrapping_add(g as u32) as uint64_t as uint64_t;
    let fresh23 = &mut (*s).h[7 as libc::c_int as usize];
    *fresh23 = (*fresh23 as libc::c_ulong).wrapping_add(h as u32) as uint64_t as uint64_t;
}
#[no_mangle]
unsafe extern "C" fn sha512_final(
    s: *mut sha512_state,
    blk: *const uint8_t,
    total_size: size_t,
) {
    let mut temp: [uint8_t; 128] = [
        0 as libc::c_int as uint8_t,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
        0,
    ];
    let last_size: size_t = total_size & (128 as libc::c_int - 1 as libc::c_int) as libc::c_ulong;
    if last_size != 0 {
        memcpy(
            temp.as_mut_ptr() as *mut libc::c_void,
            blk as *const libc::c_void,
            last_size,
        );
    }
    temp[last_size as usize] = 0x80 as libc::c_int as uint8_t;
    if last_size > 111 as libc::c_int as libc::c_ulong {
        sha512_block(s, temp.as_mut_ptr());
        memset(
            temp.as_mut_ptr() as *mut libc::c_void,
            0 as libc::c_int,
            ::std::mem::size_of::<[uint8_t; 128]>() as libc::c_ulong,
        );
    }
    store64(
        temp.as_mut_ptr()
            .offset(128 as libc::c_int as isize)
            .offset(-(8 as libc::c_int as isize)),
        (total_size as u64) << 3,
    );
    sha512_block(s, temp.as_mut_ptr());
}
#[no_mangle]
unsafe extern "C" fn sha512_get(
    s: *const sha512_state,
    mut hash: *mut uint8_t,
    mut offset: libc::c_uint,
    mut len: libc::c_uint,
) {
    let mut i: libc::c_int = 0;
    if offset > 128 as libc::c_int as libc::c_uint {
        return;
    }
    if len > (128 as libc::c_int as libc::c_uint).wrapping_sub(offset) {
        len = (128 as libc::c_int as libc::c_uint).wrapping_sub(offset);
    }
    i = (offset >> 3 as libc::c_int) as libc::c_int;
    offset &= 7 as libc::c_int as libc::c_uint;
    if offset != 0 {
        let mut tmp: [uint8_t; 8] = [0; 8];
        let mut c: libc::c_uint = (8 as libc::c_int as libc::c_uint).wrapping_sub(offset);
        if c > len {
            c = len;
        }
        let fresh24 = i;
        i += 1;
        store64(tmp.as_mut_ptr(), (*s).h[fresh24 as usize]);
        memcpy(
            hash as *mut libc::c_void,
            tmp.as_mut_ptr().offset(offset as isize) as *const libc::c_void,
            c as libc::c_ulong,
        );
        len = len.wrapping_sub(c);
        hash = hash.offset(c as isize);
    }
    while len >= 8 as libc::c_int as libc::c_uint {
        let fresh25 = i;
        i += 1;
        store64(hash, (*s).h[fresh25 as usize]);
        hash = hash.offset(8 as libc::c_int as isize);
        len = len.wrapping_sub(8 as libc::c_int as libc::c_uint);
    }
    if len != 0 {
        let mut tmp_0: [uint8_t; 8] = [0; 8];
        store64(tmp_0.as_mut_ptr(), (*s).h[i as usize]);
        memcpy(
            hash as *mut libc::c_void,
            tmp_0.as_mut_ptr() as *const libc::c_void,
            len as libc::c_ulong,
        );
    }
}

#[inline]
unsafe extern "C" fn fprime_copy(x: *mut uint8_t, a: *const uint8_t) {
    memcpy(
        x as *mut libc::c_void,
        a as *const libc::c_void,
        32 as libc::c_int as libc::c_ulong,
    );
}
unsafe extern "C" fn raw_add(x: *mut uint8_t, p: *const uint8_t) {
    let mut c: uint16_t = 0 as libc::c_int as uint16_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        c = (c as libc::c_int
            + (*x.offset(i as isize) as uint16_t as libc::c_int
                + *p.offset(i as isize) as uint16_t as libc::c_int)) as uint16_t;
        *x.offset(i as isize) = c as uint8_t;
        c = (c as libc::c_int >> 8 as libc::c_int) as uint16_t;
        i += 1;
    }
}
unsafe extern "C" fn raw_try_sub(x: *mut uint8_t, p: *const uint8_t) {
    let mut minusp: [uint8_t; 32] = [0; 32];
    let mut c: uint16_t = 0 as libc::c_int as uint16_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        c = (*x.offset(i as isize) as uint16_t as libc::c_int
            - *p.offset(i as isize) as uint16_t as libc::c_int
            - c as libc::c_int) as uint16_t;
        minusp[i as usize] = c as uint8_t;
        c = (c as libc::c_int >> 8 as libc::c_int & 1 as libc::c_int) as uint16_t;
        i += 1;
    }
    fprime_select(x, minusp.as_mut_ptr(), x, c as u8);
}
unsafe extern "C" fn prime_msb(p: *const uint8_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    let mut x: uint8_t = 0;
    i = 32 as libc::c_int - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if *p.offset(i as isize) != 0 {
            break;
        }
        i -= 1;
    }
    x = *p.offset(i as isize);
    i <<= 3 as libc::c_int;
    while x != 0 {
        x = (x as libc::c_int >> 1 as libc::c_int) as uint8_t;
        i += 1;
    }
    i - 1 as libc::c_int
}
unsafe extern "C" fn shift_n_bits(x: *mut uint8_t, n: libc::c_int) {
    let mut c: uint16_t = 0 as libc::c_int as uint16_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        c = (c as libc::c_int | (*x.offset(i as isize) as uint16_t as libc::c_int) << n)
            as uint16_t;
        *x.offset(i as isize) = c as uint8_t;
        c = (c as libc::c_int >> 8 as libc::c_int) as uint16_t;
        i += 1;
    }
}
#[inline]
unsafe extern "C" fn min_int(a: libc::c_int, b: libc::c_int) -> libc::c_int {
    if a < b {
        a
    } else {
        b
    }
}
#[no_mangle]
unsafe extern "C" fn fprime_from_bytes(
    n: *mut uint8_t,
    x: *const uint8_t,
    len: size_t,
    modulus: *const uint8_t,
) {
    let preload_total: libc::c_int = min_int(
        prime_msb(modulus) - 1 as libc::c_int,
        (len << 3 as libc::c_int) as libc::c_int,
    );
    let preload_bytes: libc::c_int = preload_total >> 3 as libc::c_int;
    let preload_bits: libc::c_int = preload_total & 7 as libc::c_int;
    let rbits: libc::c_int =
        (len << 3 as libc::c_int).wrapping_sub(preload_total as libc::c_ulong) as libc::c_int;
    let mut i: libc::c_int = 0;
    memset(
        n as *mut libc::c_void,
        0 as libc::c_int,
        32 as libc::c_int as libc::c_ulong,
    );
    i = 0 as libc::c_int;
    while i < preload_bytes {
        *n.offset(i as isize) = *x.offset(
            len.wrapping_sub(preload_bytes as libc::c_ulong)
                .wrapping_add(i as libc::c_ulong) as isize,
        );
        i += 1;
    }
    if preload_bits != 0 {
        shift_n_bits(n, preload_bits);
        let fresh0 = &mut (*n.offset(0 as libc::c_int as isize));
        *fresh0 = (*fresh0 as libc::c_int
            | *x.offset(
                len.wrapping_sub(preload_bytes as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong) as isize,
            ) as libc::c_int >> (8 as libc::c_int - preload_bits)) as uint8_t;
    }
    i = rbits - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        let bit: uint8_t = (*x.offset((i >> 3 as libc::c_int) as isize) as libc::c_int
            >> (i & 7 as libc::c_int)
            & 1 as libc::c_int) as uint8_t;
        shift_n_bits(n, 1 as libc::c_int);
        let fresh1 = &mut (*n.offset(0 as libc::c_int as isize));
        *fresh1 = (*fresh1 as libc::c_int | bit as libc::c_int) as uint8_t;
        raw_try_sub(n, modulus);
        i -= 1;
    }
}
unsafe extern "C" fn fprime_select(
    dst: *mut uint8_t,
    zero: *const uint8_t,
    one: *const uint8_t,
    condition: uint8_t,
) {
    let mask: uint8_t = -(condition as libc::c_int) as uint8_t;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 32 as libc::c_int {
        *dst.offset(i as isize) = (*zero.offset(i as isize) as libc::c_int
            ^ mask as libc::c_int
                & (*one.offset(i as isize) as libc::c_int
                    ^ *zero.offset(i as isize) as libc::c_int))
            as uint8_t;
        i += 1;
    }
}
#[no_mangle]
unsafe extern "C" fn fprime_add(
    r: *mut uint8_t,
    a: *const uint8_t,
    modulus: *const uint8_t,
) {
    raw_add(r, a);
    raw_try_sub(r, modulus);
}
#[no_mangle]
unsafe extern "C" fn fprime_mul(
    r: *mut uint8_t,
    a: *const uint8_t,
    b: *const uint8_t,
    modulus: *const uint8_t,
) {
    let mut i: libc::c_int = 0;
    memset(
        r as *mut libc::c_void,
        0 as libc::c_int,
        32 as libc::c_int as libc::c_ulong,
    );
    i = prime_msb(modulus);
    while i >= 0 as libc::c_int {
        let bit: uint8_t = (*b.offset((i >> 3 as libc::c_int) as isize) as libc::c_int
            >> (i & 7 as libc::c_int)
            & 1 as libc::c_int) as uint8_t;
        let mut plusa: [uint8_t; 32] = [0; 32];
        shift_n_bits(r, 1 as libc::c_int);
        raw_try_sub(r, modulus);
        fprime_copy(plusa.as_mut_ptr(), r);
        fprime_add(plusa.as_mut_ptr(), a, modulus);
        fprime_select(r, r, plusa.as_mut_ptr(), bit);
        i -= 1;
    }
}

#[derive(Copy, Clone)]
#[repr(C)]
struct ed25519_pt {
    pub x: [uint8_t; 32],
    pub y: [uint8_t; 32],
    pub t: [uint8_t; 32],
    pub z: [uint8_t; 32],
}
#[inline]
unsafe extern "C" fn ed25519_copy(dst: *mut ed25519_pt, src: *const ed25519_pt) {
    memcpy(
        dst as *mut libc::c_void,
        src as *const libc::c_void,
        ::std::mem::size_of::<ed25519_pt>() as libc::c_ulong,
    );
}
#[no_mangle]
static mut ed25519_base: ed25519_pt = {
    
    ed25519_pt {
        x: [
            0x1a as libc::c_int as uint8_t,
            0xd5 as libc::c_int as uint8_t,
            0x25 as libc::c_int as uint8_t,
            0x8f as libc::c_int as uint8_t,
            0x60 as libc::c_int as uint8_t,
            0x2d as libc::c_int as uint8_t,
            0x56 as libc::c_int as uint8_t,
            0xc9 as libc::c_int as uint8_t,
            0xb2 as libc::c_int as uint8_t,
            0xa7 as libc::c_int as uint8_t,
            0x25 as libc::c_int as uint8_t,
            0x95 as libc::c_int as uint8_t,
            0x60 as libc::c_int as uint8_t,
            0xc7 as libc::c_int as uint8_t,
            0x2c as libc::c_int as uint8_t,
            0x69 as libc::c_int as uint8_t,
            0x5c as libc::c_int as uint8_t,
            0xdc as libc::c_int as uint8_t,
            0xd6 as libc::c_int as uint8_t,
            0xfd as libc::c_int as uint8_t,
            0x31 as libc::c_int as uint8_t,
            0xe2 as libc::c_int as uint8_t,
            0xa4 as libc::c_int as uint8_t,
            0xc0 as libc::c_int as uint8_t,
            0xfe as libc::c_int as uint8_t,
            0x53 as libc::c_int as uint8_t,
            0x6e as libc::c_int as uint8_t,
            0xcd as libc::c_int as uint8_t,
            0xd3 as libc::c_int as uint8_t,
            0x36 as libc::c_int as uint8_t,
            0x69 as libc::c_int as uint8_t,
            0x21 as libc::c_int as uint8_t,
        ],
        y: [
            0x58 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
        ],
        t: [
            0xa3 as libc::c_int as uint8_t,
            0xdd as libc::c_int as uint8_t,
            0xb7 as libc::c_int as uint8_t,
            0xa5 as libc::c_int as uint8_t,
            0xb3 as libc::c_int as uint8_t,
            0x8a as libc::c_int as uint8_t,
            0xde as libc::c_int as uint8_t,
            0x6d as libc::c_int as uint8_t,
            0xf5 as libc::c_int as uint8_t,
            0x52 as libc::c_int as uint8_t,
            0x51 as libc::c_int as uint8_t,
            0x77 as libc::c_int as uint8_t,
            0x80 as libc::c_int as uint8_t,
            0x9f as libc::c_int as uint8_t,
            0xf0 as libc::c_int as uint8_t,
            0x20 as libc::c_int as uint8_t,
            0x7d as libc::c_int as uint8_t,
            0xe3 as libc::c_int as uint8_t,
            0xab as libc::c_int as uint8_t,
            0x64 as libc::c_int as uint8_t,
            0x8e as libc::c_int as uint8_t,
            0x4e as libc::c_int as uint8_t,
            0xea as libc::c_int as uint8_t,
            0x66 as libc::c_int as uint8_t,
            0x65 as libc::c_int as uint8_t,
            0x76 as libc::c_int as uint8_t,
            0x8b as libc::c_int as uint8_t,
            0xd7 as libc::c_int as uint8_t,
            0xf as libc::c_int as uint8_t,
            0x5f as libc::c_int as uint8_t,
            0x87 as libc::c_int as uint8_t,
            0x67 as libc::c_int as uint8_t,
        ],
        z: [
            1 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    }
};
#[no_mangle]
static mut ed25519_neutral: ed25519_pt = {
    
    ed25519_pt {
        x: [
            0 as libc::c_int as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        y: [
            1 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        t: [
            0 as libc::c_int as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
        z: [
            1 as libc::c_int as uint8_t,
            0 as libc::c_int as uint8_t,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
            0,
        ],
    }
};
#[no_mangle]
unsafe extern "C" fn ed25519_project(
    p: *mut ed25519_pt,
    x: *const uint8_t,
    y: *const uint8_t,
) {
    f25519_copy(((*p).x).as_mut_ptr(), x);
    f25519_copy(((*p).y).as_mut_ptr(), y);
    f25519_load(((*p).z).as_mut_ptr(), 1);
    f25519_mul__distinct(((*p).t).as_mut_ptr(), x, y);
}
#[no_mangle]
unsafe extern "C" fn ed25519_unproject(
    x: *mut uint8_t,
    y: *mut uint8_t,
    p: *const ed25519_pt,
) {
    let mut z1: [uint8_t; 32] = [0; 32];
    f25519_inv__distinct(z1.as_mut_ptr(), ((*p).z).as_ptr());
    f25519_mul__distinct(x, ((*p).x).as_ptr(), z1.as_mut_ptr());
    f25519_mul__distinct(y, ((*p).y).as_ptr(), z1.as_mut_ptr());
    f25519_normalize(x);
    f25519_normalize(y);
}
static mut ed25519_d: [uint8_t; 32] = [
    0xa3 as libc::c_int as uint8_t,
    0x78 as libc::c_int as uint8_t,
    0x59 as libc::c_int as uint8_t,
    0x13 as libc::c_int as uint8_t,
    0xca as libc::c_int as uint8_t,
    0x4d as libc::c_int as uint8_t,
    0xeb as libc::c_int as uint8_t,
    0x75 as libc::c_int as uint8_t,
    0xab as libc::c_int as uint8_t,
    0xd8 as libc::c_int as uint8_t,
    0x41 as libc::c_int as uint8_t,
    0x41 as libc::c_int as uint8_t,
    0x4d as libc::c_int as uint8_t,
    0xa as libc::c_int as uint8_t,
    0x70 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x98 as libc::c_int as uint8_t,
    0xe8 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0x77 as libc::c_int as uint8_t,
    0x79 as libc::c_int as uint8_t,
    0x40 as libc::c_int as uint8_t,
    0xc7 as libc::c_int as uint8_t,
    0x8c as libc::c_int as uint8_t,
    0x73 as libc::c_int as uint8_t,
    0xfe as libc::c_int as uint8_t,
    0x6f as libc::c_int as uint8_t,
    0x2b as libc::c_int as uint8_t,
    0xee as libc::c_int as uint8_t,
    0x6c as libc::c_int as uint8_t,
    0x3 as libc::c_int as uint8_t,
    0x52 as libc::c_int as uint8_t,
];
#[no_mangle]
unsafe extern "C" fn ed25519_pack(
    c: *mut uint8_t,
    x: *const uint8_t,
    y: *const uint8_t,
) {
    let mut tmp: [uint8_t; 32] = [0; 32];
    let mut parity: uint8_t = 0;
    f25519_copy(tmp.as_mut_ptr(), x);
    f25519_normalize(tmp.as_mut_ptr());
    parity = ((tmp[0 as libc::c_int as usize] as libc::c_int & 1 as libc::c_int)
        << 7 as libc::c_int) as uint8_t;
    f25519_copy(c, y);
    f25519_normalize(c);
    let fresh0 = &mut (*c.offset(31 as libc::c_int as isize));
    *fresh0 = (*fresh0 as libc::c_int | parity as libc::c_int) as uint8_t;
}
#[no_mangle]
unsafe extern "C" fn ed25519_try_unpack(
    x: *mut uint8_t,
    y: *mut uint8_t,
    comp: *const uint8_t,
) -> uint8_t {
    let parity: libc::c_int =
        *comp.offset(31 as libc::c_int as isize) as libc::c_int >> 7 as libc::c_int;
    let mut a: [uint8_t; 32] = [0; 32];
    let mut b: [uint8_t; 32] = [0; 32];
    let mut c: [uint8_t; 32] = [0; 32];
    f25519_copy(y, comp);
    let fresh1 = &mut (*y.offset(31 as libc::c_int as isize));
    *fresh1 = (*fresh1 as libc::c_int & 127 as libc::c_int) as uint8_t;
    f25519_mul__distinct(c.as_mut_ptr(), y, y);
    f25519_mul__distinct(b.as_mut_ptr(), c.as_mut_ptr(), ed25519_d.as_ptr());
    f25519_add(a.as_mut_ptr(), b.as_mut_ptr(), f25519_one.as_ptr());
    f25519_inv__distinct(b.as_mut_ptr(), a.as_mut_ptr());
    f25519_sub(a.as_mut_ptr(), c.as_mut_ptr(), f25519_one.as_ptr());
    f25519_mul__distinct(c.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    f25519_sqrt(a.as_mut_ptr(), c.as_mut_ptr());
    f25519_neg(b.as_mut_ptr(), a.as_mut_ptr());
    f25519_select(
        x,
        a.as_mut_ptr(),
        b.as_mut_ptr(),
        (a[0 as libc::c_int as usize] as libc::c_int ^ parity) as u8 & 1,
    );
    f25519_mul__distinct(a.as_mut_ptr(), x, x);
    f25519_normalize(a.as_mut_ptr());
    f25519_normalize(c.as_mut_ptr());

    f25519_eq(a.as_mut_ptr(), c.as_mut_ptr()) as uint8_t
}
static mut ed25519_k: [uint8_t; 32] = [
    0x59 as libc::c_int as uint8_t,
    0xf1 as libc::c_int as uint8_t,
    0xb2 as libc::c_int as uint8_t,
    0x26 as libc::c_int as uint8_t,
    0x94 as libc::c_int as uint8_t,
    0x9b as libc::c_int as uint8_t,
    0xd6 as libc::c_int as uint8_t,
    0xeb as libc::c_int as uint8_t,
    0x56 as libc::c_int as uint8_t,
    0xb1 as libc::c_int as uint8_t,
    0x83 as libc::c_int as uint8_t,
    0x82 as libc::c_int as uint8_t,
    0x9a as libc::c_int as uint8_t,
    0x14 as libc::c_int as uint8_t,
    0xe0 as libc::c_int as uint8_t,
    0 as libc::c_int as uint8_t,
    0x30 as libc::c_int as uint8_t,
    0xd1 as libc::c_int as uint8_t,
    0xf3 as libc::c_int as uint8_t,
    0xee as libc::c_int as uint8_t,
    0xf2 as libc::c_int as uint8_t,
    0x80 as libc::c_int as uint8_t,
    0x8e as libc::c_int as uint8_t,
    0x19 as libc::c_int as uint8_t,
    0xe7 as libc::c_int as uint8_t,
    0xfc as libc::c_int as uint8_t,
    0xdf as libc::c_int as uint8_t,
    0x56 as libc::c_int as uint8_t,
    0xdc as libc::c_int as uint8_t,
    0xd9 as libc::c_int as uint8_t,
    0x6 as libc::c_int as uint8_t,
    0x24 as libc::c_int as uint8_t,
];
#[no_mangle]
unsafe extern "C" fn ed25519_add(
    r: *mut ed25519_pt,
    p1: *const ed25519_pt,
    p2: *const ed25519_pt,
) {
    let mut a: [uint8_t; 32] = [0; 32];
    let mut b: [uint8_t; 32] = [0; 32];
    let mut c: [uint8_t; 32] = [0; 32];
    let mut d: [uint8_t; 32] = [0; 32];
    let mut e: [uint8_t; 32] = [0; 32];
    let mut f: [uint8_t; 32] = [0; 32];
    let mut g: [uint8_t; 32] = [0; 32];
    let mut h: [uint8_t; 32] = [0; 32];
    f25519_sub(c.as_mut_ptr(), ((*p1).y).as_ptr(), ((*p1).x).as_ptr());
    f25519_sub(d.as_mut_ptr(), ((*p2).y).as_ptr(), ((*p2).x).as_ptr());
    f25519_mul__distinct(a.as_mut_ptr(), c.as_mut_ptr(), d.as_mut_ptr());
    f25519_add(c.as_mut_ptr(), ((*p1).y).as_ptr(), ((*p1).x).as_ptr());
    f25519_add(d.as_mut_ptr(), ((*p2).y).as_ptr(), ((*p2).x).as_ptr());
    f25519_mul__distinct(b.as_mut_ptr(), c.as_mut_ptr(), d.as_mut_ptr());
    f25519_mul__distinct(d.as_mut_ptr(), ((*p1).t).as_ptr(), ((*p2).t).as_ptr());
    f25519_mul__distinct(c.as_mut_ptr(), d.as_mut_ptr(), ed25519_k.as_ptr());
    f25519_mul__distinct(d.as_mut_ptr(), ((*p1).z).as_ptr(), ((*p2).z).as_ptr());
    f25519_add(d.as_mut_ptr(), d.as_mut_ptr(), d.as_mut_ptr());
    f25519_sub(e.as_mut_ptr(), b.as_mut_ptr(), a.as_mut_ptr());
    f25519_sub(f.as_mut_ptr(), d.as_mut_ptr(), c.as_mut_ptr());
    f25519_add(g.as_mut_ptr(), d.as_mut_ptr(), c.as_mut_ptr());
    f25519_add(h.as_mut_ptr(), b.as_mut_ptr(), a.as_mut_ptr());
    f25519_mul__distinct(((*r).x).as_mut_ptr(), e.as_mut_ptr(), f.as_mut_ptr());
    f25519_mul__distinct(((*r).y).as_mut_ptr(), g.as_mut_ptr(), h.as_mut_ptr());
    f25519_mul__distinct(((*r).t).as_mut_ptr(), e.as_mut_ptr(), h.as_mut_ptr());
    f25519_mul__distinct(((*r).z).as_mut_ptr(), f.as_mut_ptr(), g.as_mut_ptr());
}
#[no_mangle]
unsafe extern "C" fn ed25519_double(r: *mut ed25519_pt, p: *const ed25519_pt) {
    let mut a: [uint8_t; 32] = [0; 32];
    let mut b: [uint8_t; 32] = [0; 32];
    let mut c: [uint8_t; 32] = [0; 32];
    let mut e: [uint8_t; 32] = [0; 32];
    let mut f: [uint8_t; 32] = [0; 32];
    let mut g: [uint8_t; 32] = [0; 32];
    let mut h: [uint8_t; 32] = [0; 32];
    f25519_mul__distinct(a.as_mut_ptr(), ((*p).x).as_ptr(), ((*p).x).as_ptr());
    f25519_mul__distinct(b.as_mut_ptr(), ((*p).y).as_ptr(), ((*p).y).as_ptr());
    f25519_mul__distinct(c.as_mut_ptr(), ((*p).z).as_ptr(), ((*p).z).as_ptr());
    f25519_add(c.as_mut_ptr(), c.as_mut_ptr(), c.as_mut_ptr());
    f25519_add(f.as_mut_ptr(), ((*p).x).as_ptr(), ((*p).y).as_ptr());
    f25519_mul__distinct(e.as_mut_ptr(), f.as_mut_ptr(), f.as_mut_ptr());
    f25519_sub(e.as_mut_ptr(), e.as_mut_ptr(), a.as_mut_ptr());
    f25519_sub(e.as_mut_ptr(), e.as_mut_ptr(), b.as_mut_ptr());
    f25519_sub(g.as_mut_ptr(), b.as_mut_ptr(), a.as_mut_ptr());
    f25519_sub(f.as_mut_ptr(), g.as_mut_ptr(), c.as_mut_ptr());
    f25519_neg(h.as_mut_ptr(), b.as_mut_ptr());
    f25519_sub(h.as_mut_ptr(), h.as_mut_ptr(), a.as_mut_ptr());
    f25519_mul__distinct(((*r).x).as_mut_ptr(), e.as_mut_ptr(), f.as_mut_ptr());
    f25519_mul__distinct(((*r).y).as_mut_ptr(), g.as_mut_ptr(), h.as_mut_ptr());
    f25519_mul__distinct(((*r).t).as_mut_ptr(), e.as_mut_ptr(), h.as_mut_ptr());
    f25519_mul__distinct(((*r).z).as_mut_ptr(), f.as_mut_ptr(), g.as_mut_ptr());
}
#[no_mangle]
unsafe extern "C" fn ed25519_smult(
    r_out: *mut ed25519_pt,
    p: *const ed25519_pt,
    e: *const uint8_t,
) {
    let mut r: ed25519_pt = ed25519_pt {
        x: [0; 32],
        y: [0; 32],
        t: [0; 32],
        z: [0; 32],
    };
    let mut i: libc::c_int = 0;
    ed25519_copy(&mut r, addr_of!(ed25519_neutral));
    i = 255 as libc::c_int;
    while i >= 0 as libc::c_int {
        let bit: uint8_t = (*e.offset((i >> 3 as libc::c_int) as isize) as libc::c_int
            >> (i & 7 as libc::c_int)
            & 1 as libc::c_int) as uint8_t;
        let mut s: ed25519_pt = ed25519_pt {
            x: [0; 32],
            y: [0; 32],
            t: [0; 32],
            z: [0; 32],
        };
        ed25519_double(&mut r, &mut r);
        ed25519_add(&mut s, &mut r, p);
        f25519_select(
            (r.x).as_mut_ptr(),
            (r.x).as_mut_ptr(),
            (s.x).as_mut_ptr(),
            bit,
        );
        f25519_select(
            (r.y).as_mut_ptr(),
            (r.y).as_mut_ptr(),
            (s.y).as_mut_ptr(),
            bit,
        );
        f25519_select(
            (r.z).as_mut_ptr(),
            (r.z).as_mut_ptr(),
            (s.z).as_mut_ptr(),
            bit,
        );
        f25519_select(
            (r.t).as_mut_ptr(),
            (r.t).as_mut_ptr(),
            (s.t).as_mut_ptr(),
            bit,
        );
        i -= 1;
    }
    ed25519_copy(r_out, &mut r);
}

static mut ed25519_order: [u8; 32] = [
    0xed, 0xd3, 0xf5, 0x5c, 0x1a, 0x63, 0x12, 0x58, 0xd6, 0x9c, 0xf7, 0xa2, 0xde, 0xf9, 0xde, 0x14,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x10,
];
unsafe fn sha512_init(s: *mut sha512_state) {
    memcpy(
        s as *mut libc::c_void,
        sha512_initial_state.h.as_mut_ptr() as *mut libc::c_void,
        core::mem::size_of::<sha512_state>() as u32,
    );
}
unsafe fn ed25519_prepare(e: *mut uint8_t) {
    *e.wrapping_add(0) &= 0xf8;
    *e.wrapping_add(31) &= 0x7f;
    *e.wrapping_add(31) |= 0x40;
}
unsafe fn expand_key(expanded: *mut u8, secret: *const u8) {
    let mut s = sha512_state { h: [0; 8] };
    sha512_init(&mut s);
    sha512_final(&mut s, secret, 32);
    sha512_get(&s, expanded, 0, 64);
    ed25519_prepare(expanded);
}
unsafe fn upp(p: *mut ed25519_pt, packed: *const uint8_t) -> uint8_t {
    let mut x: [u8; 32] = [0; 32];
    let mut y: [u8; 32] = [0; 32];
    let ok = ed25519_try_unpack(x.as_mut_ptr(), y.as_mut_ptr(), packed);

    ed25519_project(p, x.as_mut_ptr(), y.as_mut_ptr());

    ok
}
unsafe fn pp(packed: *mut uint8_t, p: *const ed25519_pt) {
    let mut x: [uint8_t; 32] = [0; 32];
    let mut y: [uint8_t; 32] = [0; 32];
    ed25519_unproject(x.as_mut_ptr(), y.as_mut_ptr(), p);
    ed25519_pack(packed, x.as_mut_ptr(), y.as_mut_ptr());
}
unsafe fn sm_pack(r: *mut uint8_t, k: *const uint8_t) {
    let p: ed25519_pt = ed25519_pt {
        x: [0; 32],
        y: [0; 32],
        t: [0; 32],
        z: [0; 32],
    };
    ed25519_smult(r as *mut ed25519_pt, addr_of_mut!(ed25519_base), k);
    pp(r, &p);
}
unsafe fn edsign_sec_to_pub(_pub: *mut u8, secret: *const u8) {
    let mut expanded: [u8; 64] = [0; 64];

    expand_key(expanded.as_mut_ptr(), secret);
    sm_pack(_pub, expanded.as_mut_ptr());
}
unsafe fn hash_with_prefix(
    out_fp: *mut u8,
    init_block: *mut u8,
    prefix_size: uint32_t,
    message: *const u8,
    len: size_t,
) {
    let mut s = sha512_state { h: [0; 8] };

    sha512_init(&mut s);

    if len < 128 && len + prefix_size < 128 {
        memcpy(
            init_block.wrapping_add(prefix_size as usize) as *mut libc::c_void,
            message as *mut libc::c_void,
            len,
        );
        sha512_final(&mut s, init_block, len + prefix_size);
    } else {
        let mut i: size_t;

        memcpy(
            init_block.wrapping_add(prefix_size as usize) as *mut libc::c_void,
            message as *mut libc::c_void,
            128 - prefix_size,
        );
        sha512_block(&mut s, init_block);

        i = 128 - prefix_size;
        while i + 128 <= len {
            sha512_block(&mut s, message.wrapping_add(i as usize));

            sha512_final(&mut s, message.wrapping_add(i as usize), len + prefix_size);
            i += 128
        }
    }
    sha512_get(&s, init_block, 0, 64);
    fprime_from_bytes(out_fp, init_block, 64, ed25519_order.as_mut_ptr());
}

unsafe fn generate_k(k: *mut u8, kgen_key: *const u8, message: *const u8, len: u32) {
    let mut block: [u8; 128] = [0; 128];

    memcpy(
        block.as_mut_ptr() as *mut libc::c_void,
        kgen_key as *mut libc::c_void,
        32,
    );
    hash_with_prefix(k, block.as_mut_ptr(), 32, message, len);
}

unsafe fn hash_message(z: *mut u8, r: *const u8, a: *const u8, m: *const u8, len: u32) {
    let mut block: [u8; 128] = [0; 128];

    memcpy(
        block.as_mut_ptr() as *mut libc::c_void,
        r as *mut libc::c_void,
        32,
    );
    memcpy(
        block.as_mut_ptr().wrapping_add(32) as *mut libc::c_void,
        a as *mut libc::c_void,
        32,
    );
    hash_with_prefix(z, block.as_mut_ptr(), 64, m, len);
}

unsafe fn edsign_sign(
    signature: *mut u8,
    _pub: *const u8,
    secret: *const u8,
    message: *const u8,
    len: u32,
) {
    let mut expanded: [u8; 64] = [0; 64];
    let mut e: [u8; 32] = [0; 32];
    let mut s: [u8; 32] = [0; 32];
    let mut k: [u8; 32] = [0; 32];
    let mut z: [u8; 32] = [0; 32];

    expand_key(expanded.as_mut_ptr(), secret);

    /* Generate k and R = kB */
    generate_k(
        k.as_mut_ptr(),
        expanded.as_mut_ptr().wrapping_add(32),
        message,
        len,
    );
    sm_pack(signature, k.as_mut_ptr());

    /* Compute z = H(R, A, M) */
    hash_message(z.as_mut_ptr(), signature, _pub, message, len);

    /* Obtain e */
    fprime_from_bytes(
        e.as_mut_ptr(),
        expanded.as_mut_ptr(),
        32,
        ed25519_order.as_mut_ptr(),
    );

    /* Compute s = ze + k */
    fprime_mul(
        s.as_mut_ptr(),
        z.as_mut_ptr(),
        e.as_mut_ptr(),
        ed25519_order.as_mut_ptr(),
    );
    fprime_add(s.as_mut_ptr(), k.as_mut_ptr(), ed25519_order.as_mut_ptr());
    memcpy(
        signature.wrapping_add(32) as *mut libc::c_void,
        s.as_mut_ptr() as *mut libc::c_void,
        32,
    );
}

unsafe fn edsign_verify(signature: *const u8, _pub: *const u8, message: *const u8, len: u32) -> u8 {
    let mut p = ed25519_pt {
        x: [0; 32],
        y: [0; 32],
        t: [0; 32],
        z: [0; 32],
    };
    let mut q = ed25519_pt {
        x: [0; 32],
        y: [0; 32],
        t: [0; 32],
        z: [0; 32],
    };
    let mut lhs: [u8; 32] = [0; 32];
    let mut rhs: [u8; 32] = [0; 32];
    let mut z: [u8; 32] = [0; 32];
    let mut ok: u8 = 1;

    /* Compute z = H(R, A, M) */
    hash_message(z.as_mut_ptr(), signature, _pub, message, len);

    /* sB = (ze + k)B = ... */
    sm_pack(lhs.as_mut_ptr(), signature.wrapping_add(32));

    /* ... = zA + R */
    ok &= upp(&mut p, _pub);
    ed25519_smult(&mut p, &p, z.as_mut_ptr());
    ok &= upp(&mut q, signature);
    ed25519_add(&mut p, &p, &q);
    pp(rhs.as_mut_ptr(), &p);

    /* Equal? */
    ok & f25519_eq(lhs.as_mut_ptr(), rhs.as_mut_ptr())
}

/// private_key: [u8; 64],
/// public_key: [u8; 32],
/// random_seed: [u8; 32]
pub unsafe fn compact_ed25519_keygen(
    private_key: *mut u8,
    public_key: *mut u8,
    random_seed: *mut u8,
) {
    // private key is seed + public key, like golang and others
    edsign_sec_to_pub(public_key, random_seed);
    memcpy(
        private_key as *mut libc::c_void,
        random_seed as *mut libc::c_void,
        32,
    );
    memcpy(
        private_key.wrapping_add(32) as *mut libc::c_void,
        public_key as *mut libc::c_void,
        32,
    );
    compact_wipe(random_seed as *mut libc::c_void, 32);
}

/// public_key: [u8; 32],
/// private_key: [u8; 64]
pub unsafe fn compact_ed25519_calc_public_key(public_key: *mut u8, private_key: *mut u8) {
    memcpy(
        public_key as *mut libc::c_void,
        private_key.wrapping_add(32) as *mut libc::c_void,
        32,
    );
}

/// signature: [u8; 64],
/// private_key: [u8; 64],
/// message: *const u8,
/// msg_length: u32
pub unsafe fn compact_ed25519_sign(
    signature: *mut u8,
    private_key: *mut u8,
    message: *const u8,
    msg_length: u32,
) {
    edsign_sign(
        signature,
        private_key.wrapping_add(32),
        private_key,
        message,
        msg_length,
    );
}

/// signature: [u8; 64],
/// public_key: [u8; 32],
/// message: *const libc::c_void,
/// msg_length: u32
pub unsafe fn compact_ed25519_verify(
    signature: *mut u8,
    public_key: *mut u8,
    message: *const u8,
    msg_length: u32,
) -> bool {
    edsign_verify(signature, public_key, message, msg_length) != 0
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_x25519() {
        let public_key: *mut u8 = [0; 32].as_mut_ptr();
        let private_key: *mut u8 = [0; 32].as_mut_ptr();
        unsafe {
            super::compact_x25519_keygen(
                private_key,
                public_key,
                [
                    11, 58, 224, 235, 223, 165, 29, 103, 20, 105, 48, 68, 189, 162, 49, 241, 2,
                    152, 190, 37, 90, 135, 181, 155, 113, 143, 226, 123, 238, 210, 134, 175,
                ]
                .as_mut_ptr(),
            )
        };
        println!("public_key: {:?}", super::key_to_vec(public_key, 32));
        println!("private_key: {:?}", super::key_to_vec(private_key, 32));
        if super::key_to_vec(public_key, 32)
            == [
                89, 140, 130, 200, 210, 170, 31, 220, 69, 205, 146, 27, 220, 190, 10, 126, 197, 6,
                201, 170, 106, 25, 111, 52, 241, 82, 93, 163, 28, 181, 227, 109,
            ]
            && super::key_to_vec(private_key, 32)
                == [
                    8, 58, 224, 235, 223, 165, 29, 103, 20, 105, 48, 68, 189, 162, 49, 241, 2, 152,
                    190, 37, 90, 135, 181, 155, 113, 143, 226, 123, 238, 210, 134, 111,
                ]
        {
            assert!(true)
        } else {
            assert!(false)
        }
    }
}
