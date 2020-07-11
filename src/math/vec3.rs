use crate::math::point3::Point3;

#[derive(Debug, Copy, Clone)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn new(u: f64, v: f64, w: f64) -> Vec3 {
        Vec3 { x: u, y: v, z: w }
    }

    pub fn x(&self) -> &f64 {
        &self.x
    }

    pub fn y(&self) -> &f64 {
        &self.y
    }

    pub fn z(&self) -> &f64 {
        &self.z
    }

    pub fn add(&self, p: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + p.x(),
            y: self.y + p.y(),
            z: self.z + p.z(),
        }
    }

    pub fn add_mut(&mut self, p: &Vec3) {
        self.x += p.x();
        self.y += p.y();
        self.z += p.z();
    }

    pub fn sub(&self, p: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - p.x(),
            y: self.y - p.y(),
            z: self.z - p.z(),
        }
    }

    pub fn sub_mut(&mut self, p: &Vec3) {
        self.x -= p.x();
        self.y -= p.y();
        self.z -= p.z();
    }

    pub fn scale(&self, fact: f64) -> Vec3 {
        Vec3 {
            x: self.x * fact,
            y: self.y * fact,
            z: self.z * fact,
        }
    }

    pub fn square(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn mag(&self) -> f64 {
        self.square().sqrt().abs()
    }

    pub fn dot(&self, p: &Vec3) -> f64 {
        self.x * p.x() + self.y * p.y() + self.z * p.z()
    }

    pub fn cross(&self, v: &Vec3) -> Vec3 {
        Vec3 {
            x: (self.y * v.z()) - (self.z * v.y()),
            y: (self.x * v.z()) - (self.z * v.x()),
            z: (self.x * v.y()) - (self.y * v.x()),
        }
    }

    pub fn norm(&self) -> Vec3 {
        let m: f64 = 1. / self.mag();
        Vec3 {
            x: self.x * m,
            y: self.y * m,
            z: self.z * m,
        }
    }

    pub fn as_point3(&self) -> Point3 {
        Point3::new(self.x, self.y, self.z)
    }
}
