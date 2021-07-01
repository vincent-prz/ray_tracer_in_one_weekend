extern crate rand;
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

    pub fn length_squared(&self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn random(min: f64, max: f64) -> Vec3 {
        let x: f64 = rand::random();
        let y: f64 = rand::random();
        let z: f64 = rand::random();
        Vec3(
            x * (max - min) + min,
            y * (max - min) + min,
            z * (max - min) + min,
        )
    }

    pub fn random_in_unit_sphere() -> Vec3 {
        loop {
            let p = Vec3::random(-1.0, 1.0);
            if p.length_squared() < 1.0 {
                return p;
            }
        }
    }

    pub fn random_unit_vector() -> Vec3 {
        unit_vector(Vec3::random_in_unit_sphere())
    }

    pub fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
        let v = Vec3::random_in_unit_sphere();
        if dot(v, *normal) > 0.0 {
            return v;
        }
        -v
    }

    pub fn near_zero(&self) -> bool {
        let threshold = 1e-8;
        self.0.abs() < threshold && self.1.abs() < threshold && self.2.abs() < threshold
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        let b = -dot(v, n) * n;
        v + 2.0 * b
    }

    pub fn refract(v: Vec3, n: Vec3, etai_over_etat: f64) -> Vec3 {
        let orthogonal = etai_over_etat * (v - dot(v, n) * n);
        let parallel = -((1.0 - orthogonal.length_squared()).abs()).sqrt() * n;
        orthogonal + parallel
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

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, v: Vec3) -> Vec3 {
        Vec3 (
            self.x() * v.x(),
            self.y() * v.y(),
            self.z() * v.z()
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

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Vec3 {
        Vec3 (
            -self.x(),
            -self.y(),
            -self.z()
        )
    }
}

pub fn unit_vector(v: Vec3) -> Vec3 {
    (1.0 / v.length()) * v
}

pub fn dot(v: Vec3, w: Vec3) -> f64 {
    v.x() * w.x() + v.y() * w.y() + v.z() * w.z()
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3(
        u.y() * v.z() - u.z() * v.y(),
        u.z() * v.x() - u.x() * v.z(),
        u.x() * v.y() - u.y() * v.x(),
    )
}
