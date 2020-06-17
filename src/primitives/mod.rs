use std::cmp::Ordering;

use crate::math::point3::Point3;
use crate::math::ray::Ray;
use crate::math::vec3::Vec3;


use material::Color;
use material::Material;
use material::MaterialType;

// Primitives
#[derive(Debug)]
pub struct Sphere {
    origin: Point3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(o: Point3, r: f64, mat: Material) -> Sphere {
        Sphere {
            origin: o,
            radius: r,
            material: mat,
        }
    }
}

#[derive(Debug)]
pub struct PointLight {
    pub origin: Point3,
    pub col: Color,
    pub brightness: f64,
}

impl PointLight {
    pub fn new(o: Point3, c: Color, b: f64) -> PointLight {
        PointLight {
            origin: o,
            col: c,
            brightness: b,
        }
    }

    pub fn move_to(&mut self, p: &Point3) -> &mut Self {
        self.origin = *p;
        self
    }
}

impl Default for PointLight {
    fn default() -> Self {
        PointLight {
            origin: Point3::new(0., 0., 0.),
            col: Color::new(1., 1., 1.),
            brightness: 1.,
        }
    }
}

#[derive(Debug)]
pub struct Intersection<'a> {
    pub normal: Vec3,
    pub material: &'a Material,
    pub ray: Ray,
    //pub next_bounce: Box<Intersection<'a>>, // next bounce in a series of rays
}

impl<'a> Intersection<'a> {
    pub fn new(n: Vec3, mat: &'a Material, r: Ray) -> Intersection<'a> {
        Intersection {
            normal: n,
            material: mat,
            ray: r,
        }
    }

    pub fn point(&self) -> Point3 {
        self.ray.as_point3()
    }

    pub fn mat(&self) -> MaterialType {
        self.material.mat_type
    }
}

impl<'a> PartialOrd for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.ray.t.partial_cmp(&other.ray.t)
    }
}

impl<'a> PartialEq for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.ray.t == other.ray.t
    }
}

// Traits
pub trait Shadable: std::fmt::Debug {
    fn normal(&self, p: &Point3) -> Option<Vec3>;
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

impl Shadable for Sphere {
    fn normal(&self, p: &Point3) -> Option<Vec3> {
        Some(p.sub(&self.origin).norm())
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // TODO refactor
        let oc = ray.origin().sub(&self.origin);
        let a = ray.direction().square();
        let b = ray.direction().dot(&oc) * -2.;
        let c = oc.dot(&oc) - (self.radius * self.radius);

        let det = b.mul_add(b, -(4. * a * c));
        if det > 0. {
            let inv = (2. * a).recip();
            let tt = (b - det.sqrt()) * inv;
            let t = (b + det.sqrt()) * inv;
            let min = t.min(tt);
            if min >= -0.00000000001 {
                return Some(Intersection::new(
                    self.normal(&ray.at_t(min)).unwrap(),
                    &self.material,
                    ray.clone_with_t(min),
                ));
            }
        }
        None
    }
}

pub trait LightSource: std::fmt::Debug {
    fn trace_light(&self, p: &Point3) -> Ray;
}

impl LightSource for PointLight {
    fn trace_light(&self, p: &Point3) -> Ray {
        let vec = self.origin.sub(&p);
        let t = vec.mag();
        Ray::new(p.clone(), vec.norm(), t)
    }
}

pub mod camera;
pub mod material;
