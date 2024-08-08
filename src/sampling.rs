use bevy::math::{vec3, Vec3};

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

pub struct ContinuousRandomLineGenerator {
    last_vert: Vec3,
    radius: f32,
    n: u32,
    length: f32,
}

impl Default for ContinuousRandomLineGenerator {
    fn default() -> Self {
        Self {
            last_vert: Vec3::ZERO,
            radius: 1.0,
            n: 0,
            length: 0.03,
        }
    }
}

impl ContinuousRandomLineGenerator {
    pub fn next(&mut self) -> Vec3 {
        let mut noise = vec3(
            hash_noise(self.n, 1, 0),
            hash_noise(self.n, 2, 0),
            hash_noise(self.n, 3, 0),
        ) * 2.0
            - 1.0;

        if self.last_vert.x.abs() > self.radius {
            noise.x = noise.x.copysign(-self.last_vert.x.signum());
        }
        if self.last_vert.y.abs() > self.radius {
            noise.y = noise.y.copysign(-self.last_vert.y.signum());
        }
        if self.last_vert.z.abs() > self.radius {
            noise.z = noise.z.copysign(-self.last_vert.z.signum());
        }

        let next_offset = noise * self.length;
        let next_vert = self.last_vert + next_offset;
        self.last_vert = next_vert;
        self.n += 1;
        next_vert
    }

    pub fn next_line(&mut self) -> (Vec3, Vec3) {
        (self.last_vert, self.next())
    }
}
