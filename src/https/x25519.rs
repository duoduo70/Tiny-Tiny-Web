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
    unused_mut
)]
mod libc {
    pub type c_void = *mut u8;
    pub type c_ulong = u32;
    pub type c_uchar = u8;
    pub type c_ushort = u16;
    pub type c_uint = u32;
    pub type c_int = i32;
}
extern "C" {
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type uint8_t = libc::c_uchar;
pub type uint16_t = libc::c_ushort;
pub type uint32_t = libc::c_uint;
#[inline]
unsafe fn f25519_copy(mut x: *mut uint8_t, mut a: *const uint8_t) {
    memcpy(
        x as *mut libc::c_void,
        a as *const libc::c_void,
        32 as libc::c_ulong,
    );
}
static mut F25519_ONE: [uint8_t; 32] = [
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
unsafe fn f25519_load(mut x: *mut uint8_t, mut c: uint32_t) {
    let mut i: isize = 0;
    i = 0;
    while i < core::mem::size_of::<uint32_t>() as isize {
        *x.offset(i) = c as uint8_t;
        c >>= 8;
        i+=1;
    }
    while i < 32 {
        *x.offset(i) = 0;
        i+=1;
    }
}
unsafe fn f25519_normalize(mut x: *mut uint8_t) {
    let mut minusp: [uint8_t; 32] = [0; 32];
    let mut c: uint16_t = 0;
    let mut i: libc::c_int = 0;
    c = ((*x.offset(31 as libc::c_int as isize) as libc::c_int >> 7 as libc::c_int)
        * 19 as libc::c_int) as uint16_t;
    let ref mut fresh0 = *x.offset(31 as libc::c_int as isize);
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
unsafe fn f25519_eq(mut x: *const uint8_t, mut y: *const uint8_t) -> uint8_t {
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
    return ((sum as libc::c_int ^ 1 as libc::c_int) & 1 as libc::c_int) as uint8_t;
}
unsafe fn f25519_select(
    mut dst: *mut uint8_t,
    mut zero: *const uint8_t,
    mut one: *const uint8_t,
    mut condition: uint8_t,
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
unsafe fn f25519_add(mut r: *mut uint8_t, mut a: *const uint8_t, mut b: *const uint8_t) {
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
    let ref mut fresh1 = *r.offset(31 as libc::c_int as isize);
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
unsafe fn f25519_sub(mut r: *mut uint8_t, mut a: *const uint8_t, mut b: *const uint8_t) {
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
unsafe fn f25519_neg(mut r: *mut uint8_t, mut a: *const uint8_t) {
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
unsafe fn f25519_mul(mut r: *mut uint8_t, mut a: *const uint8_t, mut b: *const uint8_t) {
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
    let ref mut fresh2 = *r.offset(31 as libc::c_int as isize);
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
unsafe fn f25519_mul_c(mut r: *mut uint8_t, mut a: *const uint8_t, mut b: uint32_t) {
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
    let ref mut fresh3 = *r.offset(31 as libc::c_int as isize);
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
unsafe fn f25519_inv(mut r: *mut uint8_t, mut x: *const uint8_t) {
    let mut s: [uint8_t; 32] = [0; 32];
    let mut i: libc::c_int = 0;
    f25519_mul(s.as_mut_ptr(), x, x);
    f25519_mul(r, s.as_mut_ptr(), x);
    i = 0 as libc::c_int;
    while i < 248 as libc::c_int {
        f25519_mul(s.as_mut_ptr(), r, r);
        f25519_mul(r, s.as_mut_ptr(), x);
        i += 1;
    }
    f25519_mul(s.as_mut_ptr(), r, r);
    f25519_mul(r, s.as_mut_ptr(), s.as_mut_ptr());
    f25519_mul(s.as_mut_ptr(), r, x);
    f25519_mul(r, s.as_mut_ptr(), s.as_mut_ptr());
    f25519_mul(s.as_mut_ptr(), r, r);
    f25519_mul(r, s.as_mut_ptr(), x);
    f25519_mul(s.as_mut_ptr(), r, r);
    f25519_mul(r, s.as_mut_ptr(), x);
}
unsafe fn exp2523(mut r: *mut uint8_t, mut x: *const uint8_t, mut s: *mut uint8_t) {
    let mut i: libc::c_int = 0;
    f25519_mul(r, x, x);
    f25519_mul(s, r, x);
    i = 0 as libc::c_int;
    while i < 248 as libc::c_int {
        f25519_mul(r, s, s);
        f25519_mul(s, r, x);
        i += 1;
    }
    f25519_mul(r, s, s);
    f25519_mul(s, r, r);
    f25519_mul(r, s, x);
}
unsafe fn f25519_sqrt(mut r: *mut uint8_t, mut a: *const uint8_t) {
    let mut v: [uint8_t; 32] = [0; 32];
    let mut i: [uint8_t; 32] = [0; 32];
    let mut x: [uint8_t; 32] = [0; 32];
    let mut y: [uint8_t; 32] = [0; 32];
    f25519_mul_c(x.as_mut_ptr(), a, 2 as libc::c_int as uint32_t);
    exp2523(v.as_mut_ptr(), x.as_mut_ptr(), y.as_mut_ptr());
    f25519_mul(y.as_mut_ptr(), v.as_mut_ptr(), v.as_mut_ptr());
    f25519_mul(i.as_mut_ptr(), x.as_mut_ptr(), y.as_mut_ptr());
    f25519_load(y.as_mut_ptr(), 1 as libc::c_int as uint32_t);
    f25519_sub(i.as_mut_ptr(), i.as_mut_ptr(), y.as_mut_ptr());
    f25519_mul(x.as_mut_ptr(), v.as_mut_ptr(), a);
    f25519_mul(r, x.as_mut_ptr(), i.as_mut_ptr());
}
#[inline]
unsafe fn c25519_prepare(mut key: *mut uint8_t) {
    let ref mut fresh4 = *key.offset(0 as libc::c_int as isize);
    *fresh4 = (*fresh4 as libc::c_int & 0xf8 as libc::c_int) as uint8_t;
    let ref mut fresh5 = *key.offset(31 as libc::c_int as isize);
    *fresh5 = (*fresh5 as libc::c_int & 0x7f as libc::c_int) as uint8_t;
    let ref mut fresh6 = *key.offset(31 as libc::c_int as isize);
    *fresh6 = (*fresh6 as libc::c_int | 0x40 as libc::c_int) as uint8_t;
}
static mut C22519_BASE_X: [uint8_t; 32] = [
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
    mut x3: *mut uint8_t,
    mut z3: *mut uint8_t,
    mut x1: *const uint8_t,
    mut z1: *const uint8_t,
) {
    let mut x1sq: [uint8_t; 32] = [0; 32];
    let mut z1sq: [uint8_t; 32] = [0; 32];
    let mut x1z1: [uint8_t; 32] = [0; 32];
    let mut a: [uint8_t; 32] = [0; 32];
    f25519_mul(x1sq.as_mut_ptr(), x1, x1);
    f25519_mul(z1sq.as_mut_ptr(), z1, z1);
    f25519_mul(x1z1.as_mut_ptr(), x1, z1);
    f25519_sub(a.as_mut_ptr(), x1sq.as_mut_ptr(), z1sq.as_mut_ptr());
    f25519_mul(x3, a.as_mut_ptr(), a.as_mut_ptr());
    f25519_mul_c(
        a.as_mut_ptr(),
        x1z1.as_mut_ptr(),
        486662 as libc::c_int as uint32_t,
    );
    f25519_add(a.as_mut_ptr(), x1sq.as_mut_ptr(), a.as_mut_ptr());
    f25519_add(a.as_mut_ptr(), z1sq.as_mut_ptr(), a.as_mut_ptr());
    f25519_mul(x1sq.as_mut_ptr(), x1z1.as_mut_ptr(), a.as_mut_ptr());
    f25519_mul_c(z3, x1sq.as_mut_ptr(), 4 as libc::c_int as uint32_t);
}
unsafe fn xc_diffadd(
    mut x5: *mut uint8_t,
    mut z5: *mut uint8_t,
    mut x1: *const uint8_t,
    mut z1: *const uint8_t,
    mut x2: *const uint8_t,
    mut z2: *const uint8_t,
    mut x3: *const uint8_t,
    mut z3: *const uint8_t,
) {
    let mut da: [uint8_t; 32] = [0; 32];
    let mut cb: [uint8_t; 32] = [0; 32];
    let mut a: [uint8_t; 32] = [0; 32];
    let mut b: [uint8_t; 32] = [0; 32];
    f25519_add(a.as_mut_ptr(), x2, z2);
    f25519_sub(b.as_mut_ptr(), x3, z3);
    f25519_mul(da.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    f25519_sub(b.as_mut_ptr(), x2, z2);
    f25519_add(a.as_mut_ptr(), x3, z3);
    f25519_mul(cb.as_mut_ptr(), a.as_mut_ptr(), b.as_mut_ptr());
    f25519_add(a.as_mut_ptr(), da.as_mut_ptr(), cb.as_mut_ptr());
    f25519_mul(b.as_mut_ptr(), a.as_mut_ptr(), a.as_mut_ptr());
    f25519_mul(x5, z1, b.as_mut_ptr());
    f25519_sub(a.as_mut_ptr(), da.as_mut_ptr(), cb.as_mut_ptr());
    f25519_mul(b.as_mut_ptr(), a.as_mut_ptr(), a.as_mut_ptr());
    f25519_mul(z5, x1, b.as_mut_ptr());
}
unsafe fn c25519_smult(mut result: *mut uint8_t, mut q: *const uint8_t, mut e: *const uint8_t) {
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
            F25519_ONE.as_ptr(),
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
            F25519_ONE.as_ptr(),
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
    f25519_inv(zm1.as_mut_ptr(), zm.as_mut_ptr());
    f25519_mul(result, zm1.as_mut_ptr(), xm.as_mut_ptr());
    f25519_normalize(result);
}
unsafe fn compact_wipe(mut data: *mut libc::c_void, mut length: size_t) -> *mut libc::c_void {
    let mut p: *mut libc::c_uchar = data as *mut libc::c_uchar;
    loop {
        let fresh7 = length;
        length = length.wrapping_sub(1);
        if !(fresh7 != 0) {
            break;
        }
        let fresh8 = p;
        p = p.offset(1);
        core::ptr::write_volatile(fresh8, 0 as libc::c_int as libc::c_uchar);
    }
    return data;
}
pub unsafe fn compact_x25519_keygen(
    mut private_key: *mut uint8_t,
    mut public_key: *mut uint8_t,
    mut random_seed: *mut uint8_t,
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
        C22519_BASE_X.as_ptr(),
        private_key as *const uint8_t,
    );
}
pub unsafe fn compact_x25519_shared(
    mut shared_secret: *mut uint8_t,
    mut my_private_key: *const uint8_t,
    mut their_public_key: *const uint8_t,
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

pub fn key_to_vec(key: *mut u8) -> Vec<u8> {
    let mut i = 0;
    let mut vec = vec![];
    while i < 32 {
        vec.push(unsafe { *key.wrapping_add(i) });
        i += 1;
    }
    vec
}

#[cfg(test)]
mod tests {

    #[test]
    fn test() {
        let public_key: *mut u8 = [0; 32].as_mut_ptr();
        let pravite_key: *mut u8 = [0; 32].as_mut_ptr();
        unsafe {
            super::compact_x25519_keygen(
                pravite_key,
                public_key,
                [
                    11, 58, 224, 235, 223, 165, 29, 103, 20, 105, 48, 68, 189, 162, 49, 241, 2,
                    152, 190, 37, 90, 135, 181, 155, 113, 143, 226, 123, 238, 210, 134, 175,
                ]
                .as_mut_ptr(),
            )
        };
        println!("public_key: {:?}", super::key_to_vec(public_key));
        println!("pravite_key: {:?}", super::key_to_vec(pravite_key));
        if super::key_to_vec(public_key)
            == [
                89, 140, 130, 200, 210, 170, 31, 220, 69, 205, 146, 27, 220, 190, 10, 126, 197, 6,
                201, 170, 106, 25, 111, 52, 241, 82, 93, 163, 28, 181, 227, 109,
            ]
            && super::key_to_vec(pravite_key)
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