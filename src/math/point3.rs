use crate::math::vec3::Vec3;

#[derive(Debug, Copy, Clone)]
pub struct Point3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Point3 {
    pub fn new(u: f64, v: f64, w: f64) -> Point3 {
        Point3 { x: u, y: v, z: w }
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

    pub fn add(&self, v: &Vec3) -> Point3 {
        Point3 {
            x: self.x + v.x(),
            y: self.y + v.y(),
            z: self.z + v.z(),
        }
    }

    pub fn sub(&self, p: &Point3) -> Vec3 {
        Vec3::new(self.x - p.x(), self.y - p.y(), self.z - p.z())
    }

    pub fn clone(&self) -> Point3 {
        Point3 {
            x: self.x,
            y: self.y,
            z: self.z,
        }
    }
}

impl Default for Point3 {
    fn default() -> Self {
        Point3 {
            x: 0.,
            y: 0.,
            z: 0.,
        }
    }
}
