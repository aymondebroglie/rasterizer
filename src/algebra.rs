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
            a: 0.,
        }
    }

    pub(crate) fn as_point(&self) -> FourD {
        FourD {
            x: self.x,
            y: self.y,
            z: self.z,
            a: 1.,
        }
    }
}

#[derive(Copy, Clone)]
pub(crate) struct FourD {
    x: f64,
    y: f64,
    z: f64,
    a: f64,
}

impl ops::Sub for FourD {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        FourD {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            a: self.a - rhs.a,
        }
    }
}

impl FourD {
    pub(crate) fn new(x: f64, y: f64, z: f64, a: f64) -> Self {
        Self { x, y, z, a }
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

    pub(crate) fn a(&self) -> f64 {
        self.a
    }
}

#[derive(Copy, Clone)]
pub(crate) struct Matrix {
    pub(crate) values: [f64; 16],
}

impl ops::Mul<FourD> for Matrix  {
    type Output = FourD;

    fn mul(self, rhs: FourD) -> Self::Output {
       FourD {
           x : self.values[0] * rhs.x + self.values[1] * rhs.y + self.values[2] *rhs.z +  self.values[3] * rhs.a,
           y : self.values[4] * rhs.x + self.values[5] * rhs.y + self.values[6] *rhs.z +  self.values[7] * rhs.a,
           z : self.values[8] * rhs.x + self.values[9] * rhs.y + self.values[10] *rhs.z +  self.values[11] * rhs.a,
           a : self.values[12] * rhs.x + self.values[13] * rhs.y + self.values[14] *rhs.z +  self.values[15] * rhs.a
       }
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
}
