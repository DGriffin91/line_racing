use bevy::math::{vec3, Vec3};

use crate::TINY_LINES;

#[inline(always)]
pub fn uhash(a: u32, b: u32) -> u32 {
    let mut x = (a.overflowing_mul(1597334673).0) ^ (b.overflowing_mul(3812015801).0);
    // from https://nullprogram.com/blog/2018/07/31/
    x = x ^ (x >> 16);
    x = x.overflowing_mul(0x7feb352d).0;
    x = x ^ (x >> 15);
    x = x.overflowing_mul(0x846ca68b).0;
    x = x ^ (x >> 16);
    x
}

#[inline(always)]
pub fn unormf(n: u32) -> f32 {
    n as f32 * (1.0 / 0xffffffffu32 as f32)
}

#[inline(always)]
pub fn hash_noise(x: u32, y: u32, z: u32) -> f32 {
    unormf(uhash(x, (y << 11) + z))
}

#[inline(always)]
pub fn rng_line(x: u32) -> (Vec3, Vec3) {
    let a = vec3(
        hash_noise(x, 1, 0),
        hash_noise(x, 2, 0),
        hash_noise(x, 3, 0),
    ) * 2.0
        - 1.0;
    let b = vec3(
        hash_noise(x, 4, 0),
        hash_noise(x, 5, 0),
        hash_noise(x, 6, 0),
    ) * 2.0
        - 1.0;
    if TINY_LINES {
        (a, a + b * 0.005)
    } else {
        (a, b)
    }
}
