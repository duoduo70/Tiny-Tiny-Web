use core::cmp::min;

// ==== FROM https://github.com/torao/tinymt/ ====
const TINYMT32_MEXP: u32 = 127;
const TINYMT32_SH0: u32 = 1;
const TINYMT32_SH1: u32 = 10;
const TINYMT32_SH8: u32 = 8;
const TINYMT32_MASK: u32 = 0x7fff_ffff_u32;
const TINYMT32_MUL: f64 = 1.0f64 / 16_777_216.0_f64;
const MIN_LOOP: usize = 8;
const PRE_LOOP: usize = 8;

pub struct TinyMT32 {
    status: [u32; 4],
    mat1: u32,
    mat2: u32,
    tmat: u32,
}

impl TinyMT32 {
    #[allow(dead_code)]
    pub fn new(status: [u32; 4], mat1: u32, mat2: u32, tmat: u32) -> TinyMT32 {
        TinyMT32 {
            status,
            mat1,
            mat2,
            tmat,
        }
    }
}

/// This function represents a function used in the initialization by init_by_array
#[allow(dead_code)]
fn ini_func1(x: u32) -> u32 {
    (x ^ (x >> 27)).wrapping_mul(1_664_525_u32)
}

/// This function represents a function used in the initialization by init_by_array
#[allow(dead_code)]
fn ini_func2(x: u32) -> u32 {
    (x ^ (x >> 27)).wrapping_mul(1_566_083_941_u32)
}

/// This function certificate the period of 2^127-1.
/// @param random tinymt state vector.
fn period_certification(random: &mut TinyMT32) {
    if random.status[0] & TINYMT32_MASK == 0
        && random.status[1] == 0
        && random.status[2] == 0
        && random.status[3] == 0
    {
        random.status[0] = 'T' as u32;
        random.status[1] = 'I' as u32;
        random.status[2] = 'N' as u32;
        random.status[3] = 'Y' as u32;
    }
}

/// This function initializes the internal state array with a 32-bit unsigned integer seed.
/// @param random tinymt state vector.
/// @param seed a 32-bit unsigned integer used as a seed.
fn tinymt32_init(random: &mut TinyMT32, seed: u32) {
    random.status[0] = seed;
    random.status[1] = random.mat1;
    random.status[2] = random.mat2;
    random.status[3] = random.tmat;
    for i in 1..MIN_LOOP {
        random.status[i & 3] ^= (i as u32).wrapping_add(
            1_812_433_253_u32
                .wrapping_mul(random.status[(i - 1) & 3] ^ (random.status[(i - 1) & 3] >> 30)),
        );
    }
    period_certification(random);
    for _ in 0..PRE_LOOP {
        tinymt32_next_state(random);
    }
}

/// This function initializes the internal state array, with an array of 32-bit unsigned integers used as seeds
/// @param init_key the array of 32-bit integers, used as a seed.
/// @param key_length the length of init_key.
#[allow(dead_code)]
fn tinymt32_init_by_array(random: &mut TinyMT32, init_key: &[u32]) {
    let key_length: usize = init_key.len();
    let lag: usize = 1;
    let mid: usize = 1;
    let size: usize = 4;

    let st: &mut [u32; 4] = &mut random.status;
    st[0] = 0;
    st[1] = random.mat1;
    st[2] = random.mat2;
    st[3] = random.tmat;
    let mut count: usize = if key_length + 1 > MIN_LOOP {
        key_length + 1
    } else {
        MIN_LOOP
    };
    let mut r: u32 = ini_func1(st[0] ^ st[mid % size] ^ st[(size - 1) % size]);
    st[mid % size] = st[mid % size].wrapping_add(r);
    r += key_length as u32;
    st[(mid + lag) % size] = st[(mid + lag) % size].wrapping_add(r);
    st[0] = r;
    count -= 1;
    let mut i: usize = 1;
    let boundary = min(count, key_length);
    for key in init_key.iter().take(boundary) {
        r = ini_func1(st[i % size] ^ st[(i + mid) % size] ^ st[(i + size - 1) % size]);
        st[(i + mid) % size] = st[(i + mid) % size].wrapping_add(r);
        r += key + i as u32;
        st[(i + mid + lag) % size] = st[(i + mid + lag) % size].wrapping_add(r);
        st[i % size] = r;
        i = (i + 1) % size;
    }
    for _ in min(count, key_length)..count {
        r = ini_func1(st[i % size] ^ st[(i + mid) % size] ^ st[(i + size - 1) % size]);
        st[(i + mid) % size] = st[(i + mid) % size].wrapping_add(r);
        r += i as u32;
        st[(i + mid + lag) % size] = st[(i + mid + lag) % size].wrapping_add(r);
        st[i % size] = r;
        i = (i + 1) % size;
    }
    for _ in 0..size {
        r = ini_func2(
            st[i % size]
                .wrapping_add(st[(i + mid) % size])
                .wrapping_add(st[(i + size - 1) % size]),
        );
        st[(i + mid) % size] ^= r;
        r -= i as u32;
        st[(i + mid + lag) % size] ^= r;
        st[i % size] = r;
        i = (i + 1) % size;
    }
    period_certification(random);
    for _ in 0..PRE_LOOP {
        tinymt32_next_state(random);
    }
}

/// This function always returns 127
/// @return always 127
#[allow(dead_code)]
fn tinymt32_get_mexp(_: &TinyMT32) -> usize {
    TINYMT32_MEXP.try_into().unwrap()
}

/// This function changes internal state of tinymt32. Users should not call this function directly.
/// @param random tinymt internal status
fn tinymt32_next_state(random: &mut TinyMT32) {
    let mut y: u32 = random.status[3];
    let mut x: u32 = (random.status[0] & TINYMT32_MASK) ^ random.status[1] ^ random.status[2];
    x ^= x << TINYMT32_SH0;
    y ^= (y >> TINYMT32_SH0) ^ x;
    random.status[0] = random.status[1];
    random.status[1] = random.status[2];
    random.status[2] = x ^ (y << TINYMT32_SH1);
    random.status[3] = y;
    random.status[1] ^= (-((y & 1) as i32) as u32) & random.mat1;
    random.status[2] ^= (-((y & 1) as i32) as u32) & random.mat2;
}

/// This function outputs 32-bit unsigned integer from internal state. Users should not call this function directly.
/// @param random tinymt internal status
/// @return 32-bit unsigned pseudorandom number
#[allow(dead_code)]
fn tinymt32_temper(random: &mut TinyMT32) -> u32 {
    let mut t0: u32 = random.status[3];
    // defined(LINEARITY_CHECK)
    // t1 = random->status[0]^ (random->status[2] >> TINYMT32_SH8);
    let t1: u32 = random.status[0].wrapping_add(random.status[2] >> TINYMT32_SH8);
    t0 ^= t1;
    t0 ^ (-((t1 & 1) as i32) as u32) & random.tmat
}

/// This function outputs floating point number from internal state. Users should not call this function directly.
/// @param random tinymt internal status
/// @return floating point number r (1.0 <= r < 2.0)
#[allow(dead_code)]
fn tinymt32_temper_conv(random: &mut TinyMT32) -> f32 {
    let mut t0: u32 = random.status[3];
    // defined(LINEARITY_CHECK)
    // t1 = random->status[0]^ (random->status[2] >> TINYMT32_SH8);
    let t1: u32 = random.status[0].wrapping_add(random.status[2] >> TINYMT32_SH8);
    t0 ^= t1;
    let u: u32 = ((t0 ^ ((-((t1 & 1) as i32) as u32) & random.tmat)) >> 9) | 0x3f80_0000_u32;
    f32::from_le_bytes(u.to_le_bytes())
}

/// This function outputs floating point number from internal state. Users should not call this function directly.
/// @return floating point number r (1.0 < r < 2.0)
#[allow(dead_code)]
fn tinymt32_temper_conv_open(random: &mut TinyMT32) -> f32 {
    let mut t0: u32 = random.status[3];
    // defined(LINEARITY_CHECK)
    // t1 = random->status[0] ^ (random->status[2] >> TINYMT32_SH8);
    let t1: u32 = random.status[0].wrapping_add(random.status[2] >> TINYMT32_SH8);
    t0 ^= t1;
    let u: u32 = ((t0 ^ ((-((t1 & 1) as i32) as u32) & random.tmat)) >> 9) | 0x3f80_0001_u32;
    f32::from_le_bytes(u.to_le_bytes())
}

/// This function outputs 32-bit unsigned integer from internal state.
/// @return 32-bit unsigned integer r (0 <= r < 2^32)
#[allow(dead_code)]
fn tinymt32_generate_uint32(random: &mut TinyMT32) -> u32 {
    tinymt32_next_state(random);
    tinymt32_temper(random)
}

/// This function outputs floating point number from internal state. This function is implemented using multiplying by (1 / 2^24). floating point multiplication is faster than using union trick in my Intel CPU.
/// @return floating point number r (0.0 <= r < 1.0)
#[allow(dead_code)]
fn tinymt32_generate_float(random: &mut TinyMT32) -> f32 {
    tinymt32_next_state(random);
    ((tinymt32_temper(random) >> 8) as f64 * TINYMT32_MUL) as f32
}

/// This function outputs floating point number from internal state. This function is implemented using union trick.
/// @return floating point number r (1.0 <= r < 2.0)
#[allow(dead_code)]
fn tinymt32_generate_float12(random: &mut TinyMT32) -> f32 {
    tinymt32_next_state(random);
    tinymt32_temper_conv(random)
}

/// This function outputs floating point number from internal state.
/// This function is implemented using union trick.
/// @return floating point number r (0.0 <= r < 1.0)
#[allow(dead_code)]
fn tinymt32_generate_float01(random: &mut TinyMT32) -> f32 {
    tinymt32_next_state(random);
    tinymt32_temper_conv(random) - 1.0f32
}

/// This function outputs floating point number from internal state. This function may return 1.0 and never returns 0.0.
/// @return floating point number r (0.0 < r <= 1.0)
#[allow(dead_code)]
fn tinymt32_generate_float_oc(random: &mut TinyMT32) -> f32 {
    tinymt32_next_state(random);
    1.0f32 - tinymt32_generate_float(random)
}

/// This function outputs floating point number from internal state. This function returns neither 0.0 nor 1.0.
/// @return floating point number r (0.0 < r < 1.0)
#[allow(dead_code)]
fn tinymt32_generate_float_oo(random: &mut TinyMT32) -> f32 {
    tinymt32_next_state(random);
    tinymt32_temper_conv_open(random) - 1.0f32
}

/// This function outputs double precision floating point number from internal state. The returned value has 32-bit precision.  In other words, this function makes one double precision floating point number from one 32-bit unsigned integer.
/// @return floating point number r (0.0 <= r < 1.0)
#[allow(dead_code)]
fn tinymt32_generate_32double(random: &mut TinyMT32) -> f64 {
    tinymt32_next_state(random);
    tinymt32_temper(random) as f64 * (1.0f64 / 4_294_967_296.0_f64)
}

#[allow(dead_code)]
pub fn random_init(seed: u32) -> TinyMT32 {
    let mut random = TinyMT32 {
        status: [0, 0, 0, 0],
        mat1: 0,
        mat2: 0,
        tmat: 0,
    };
    tinymt32_init(&mut random, seed);
    random
}

impl TinyMT32 {
    #[allow(dead_code)]
    pub fn next_u32(&mut self) -> u32 {
        tinymt32_generate_uint32(self)
    }

    #[allow(dead_code)]
    pub fn next_u64(&mut self) -> u64 {
        ((self.next_u32() as u64) << 32) | (self.next_u32() as u64)
    }

    #[allow(dead_code)]
    pub fn fill_bytes(&mut self, dest: &mut [u8]) {
        let mut position = 0;
        let mut remaining = dest.len();
        while remaining > 0 {
            let bytes = self.next_u32().to_le_bytes();
            for b in bytes.iter().take(min(remaining, bytes.len())) {
                dest[position] = *b;
                position += 1;
                remaining -= 1;
            }
        }
    }
}
