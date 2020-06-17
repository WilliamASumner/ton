use crate::math::point3::Point3;
use crate::math::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub t: f64,
}

impl Ray {
    pub fn new_norm(o: Point3, d: Vec3, len: f64) -> Ray {
        Ray {
            origin: o,
            direction: d.norm(),
            t: len,
        }
    }

    pub fn new(o: Point3, d: Vec3, len: f64) -> Ray {
        Ray {
            origin: o,
            direction: d, // no normalization
            t: len,
        }
    }

    pub fn norm(&mut self) -> &mut Self {
        self.direction = self.direction.norm();
        self
    }

    pub fn origin(&self) -> &Point3 {
        &self.origin
    }

    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }

    pub fn t(&self) -> &f64 {
        &self.t
    }

    pub fn set_t(&mut self, val: f64) {
        self.t = val;
    }

    pub fn as_point3(&self) -> Point3 {
        self.origin.add(&self.direction.scale(self.t))
    }

    pub fn at_t(&self, t: f64) -> Point3 {
        self.origin.add(&self.direction.scale(t))
    }

    pub fn clone(&self) -> Self {
        Ray {
            origin: self.origin,
            direction: self.direction,
            t: self.t,
        }
    }

    pub fn dot(&self, other: &Ray) -> f64 {
        self.direction.dot(&other.direction)
    }

    pub fn clone_with_t(&self, t: f64) -> Self {
        Ray {
            origin: self.origin,
            direction: self.direction,
            t: t,
        }
    }
}
