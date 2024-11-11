use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Copy, Clone)]
pub(crate) struct ThreeD {
    x: f64,
    y: f64,
    z: f64,
}

impl ThreeD {
    pub(crate) fn new(x: f64, y: f64, z: f64) -> ThreeD {
        ThreeD { x, y, z }
    }
    pub(crate) fn as_vector(&self) -> FourD {
        FourD {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 0.,
        }
    }

    pub(crate) fn as_point(&self) -> FourD {
        FourD {
            x: self.x,
            y: self.y,
            z: self.z,
            w: 1.,
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct FourD {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}

impl ops::Sub for FourD {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        FourD {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl FourD {
    pub(crate) fn new(x: f64, y: f64, z: f64, a: f64) -> Self {
        Self { x, y, z, w: a }
    }
    pub(crate) fn det2d(&self, rhs: &Self) -> f64 {
        self.x * rhs.y - self.y * rhs.x
    }

    pub(crate) fn x(&self) -> f64 {
        self.x
    }

    pub(crate) fn y(&self) -> f64 {
        self.y
    }

    pub(crate) fn z(&self) -> f64 {
        self.z
    }

    pub(crate) fn w(&self) -> f64 {
        self.w
    }

    pub(crate) fn perspective_divide(&mut self) {
        self.x /= self.w;
        self.y /= self.w;
        self.z /= self.w;
    }

    pub(crate) fn dot(&self, rhs: &FourD) -> f64 {
        self.x() * rhs.x() + self.y() * rhs.y() + self.z() * rhs.z() + self.w() * rhs.w()
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Matrix {
    pub(crate) values: [f64; 16],
}

impl Display for Matrix {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut result = String::new();

        for i in 0..16 {
            result += &*format!("{} ", self.values[i]).to_string();
            if i % 4 == 3 {
                result += "\n";
            }
        }

        return f.write_str(&*result);
    }
}

impl ops::Mul<FourD> for Matrix {
    type Output = FourD;

    fn mul(self, rhs: FourD) -> Self::Output {
        FourD {
            x: self.values[0] * rhs.x
                + self.values[1] * rhs.y
                + self.values[2] * rhs.z
                + self.values[3] * rhs.w,
            y: self.values[4] * rhs.x
                + self.values[5] * rhs.y
                + self.values[6] * rhs.z
                + self.values[7] * rhs.w,
            z: self.values[8] * rhs.x
                + self.values[9] * rhs.y
                + self.values[10] * rhs.z
                + self.values[11] * rhs.w,
            w: self.values[12] * rhs.x
                + self.values[13] * rhs.y
                + self.values[14] * rhs.z
                + self.values[15] * rhs.w,
        }
    }
}

impl ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut values: [f64; 16] = [0.; 16];

        for i in 0..4 {
            for j in 0..4 {
                for k in 0..4 {
                    values[4 * i + j] += self.values[4 * i + k] * rhs.values[4 * k + j];
                }
            }
        }

        Matrix { values }
    }
}
impl Matrix {
    pub(crate) fn identity() -> Self {
        Matrix {
            values: [
                1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ],
        }
    }

    pub(crate) fn scale(s: ThreeD) -> Matrix {
        Matrix {
            values: [
                s.x, 0., 0., 0., 0., s.y, 0., 0., 0., 0., s.z, 0., 0., 0., 0., 1.,
            ],
        }
    }
    pub(crate) fn scale_scalar(s: f64) -> Matrix {
        Self::scale(ThreeD::new(s, s, s))
    }

    pub(crate) fn translate(s: ThreeD) -> Matrix {
        Matrix {
            values: [
                1., 0., 0., s.x, 0., 1., 0., s.y, 0., 0., 1., s.z, 0., 0., 0., 1.,
            ],
        }
    }

    pub(crate) fn rotate_xy(angle: f64) -> Matrix {
        let cos = angle.cos();
        let sin = angle.sin();

        Matrix {
            values: [
                cos, -sin, 0., 0., sin, cos, 0., 0., 0., 0., 1., 0., 0., 0., 0., 1.,
            ],
        }
    }

    pub(crate) fn rotate_yz(angle: f64) -> Matrix {
        let cos = angle.cos();
        let sin = angle.sin();

        Matrix {
            values: [
                1., 0., 0., 0., 0., cos, -sin, 0., 0., sin, cos, 0., 0., 0., 0., 1.,
            ],
        }
    }

    pub(crate) fn rotate_zx(angle: f64) -> Matrix {
        let cos = angle.cos();
        let sin = angle.sin();

        Matrix {
            values: [
                cos, 0., sin, 0., 0., 1., 0., 0., -sin, 0., cos, 0., 0., 0., 0., 1.,
            ],
        }
    }

    pub(crate) fn perspective(near: f64, far: f64, fovY: f64, aspect_ratio: f64) -> Matrix {
        let top = near * (fovY / 2.).tan();
        let right = top * aspect_ratio;

        Matrix {
            values: [
                near / right,
                0.,
                0.,
                0.,
                0.,
                near / top,
                0.,
                0.,
                0.,
                0.,
                -(far + near) / (far - near),
                -2. * far * near / (far - near),
                0.,
                0.,
                -1.,
                0.,
            ],
        }
    }
}
