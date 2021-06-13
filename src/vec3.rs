use std::ops;

#[derive(Copy, Clone)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 (
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z()
        )
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 (
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z()
        )
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 (
            self * v.x(),
            self * v.y(),
            self * v.z()
        )
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, t: f64) -> Vec3 {
        Vec3 (
            self.x() / t,
            self.y() / t,
            self.z() / t
        )
    }
}

pub fn unit_vector(v: &Vec3) -> Vec3 {
    (1.0 / v.length()) * (*v)
}

pub fn dot(v: &Vec3, w: &Vec3) -> f64 {
    v.x() * w.x() + v.y() * w.y() + v.z() * w.z()
}
