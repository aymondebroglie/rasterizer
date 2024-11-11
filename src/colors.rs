use std::ops;

#[derive(Copy, Clone)]
pub(crate) struct RGBA {
    x: f64,
    y: f64,
    z: f64,
}

impl RGBA {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> Self {
        RGBA { x, y, z }
    }

    pub(crate) fn red() -> Self {
        Self::new(1., 0., 0.)
    }

    pub(crate) fn green() -> Self {
        Self::new(0., 1., 0.)
    }
    pub(crate) fn blue() -> Self {
        Self::new(0., 0., 1.)
    }
    fn r(&self) -> u8 {
        (self.x * 255.) as u8
    }
    fn g(&self) -> u8 {
        (self.y * 255.) as u8
    }
    fn b(&self) -> u8 {
        (self.z * 255.) as u8
    }
    pub(crate) fn to_color(&self) -> u32 {
        let r = self.r() as u32;
        let g = self.g() as u32;
        let b = self.b() as u32;

        (r << 16) + (g << 8) + (b)
    }
}

impl ops::Mul<f64> for RGBA {
    type Output = RGBA;

    fn mul(self, rhs: f64) -> Self::Output {
        RGBA {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl ops::Add for RGBA {
    type Output = RGBA;

    fn add(self, rhs: Self) -> Self::Output {
        RGBA {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}
