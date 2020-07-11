use std::cmp::Ordering;
use std::f64;


use crate::math::point3::Point3;
use crate::math::ray::Ray;
use crate::math::vec3::Vec3;

use material::Color;
use material::Material;

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
pub struct Plane {
    origin: Point3,
    normal: Vec3,
    material: Material,
}

impl Plane {
    pub fn new(o: Point3, n: Vec3, mat: Material) -> Plane {
        Plane {
            origin: o,
            normal: n,
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

    pub fn biased_point(&self,dir: &Vec3) -> Point3 {
        self.ray.as_point3().add(&dir.scale(1e-4))
    }

    pub fn mat(&self) -> Material {
        *self.material
    }

    pub fn reflect(&self) -> Ray {
        let ang = self.ray.direction.dot(&self.normal);
        let dir = self.ray.direction.sub(&self.normal.scale(2.*(ang)));
        Ray::new(self.point(),dir,1.)
    }

    pub fn refract(&self) -> Option<Ray> {
        if let Material::Refractive { spec_col, refr_col, ior } = &self.mat() {
            let mut cos = self.ray.direction.dot(&self.normal);
            let n1 : f64;
            let n2 : f64;
            let normal: Vec3;
            if cos <= 0. { // entering a medium (incident to normal direction > 90 deg)
                n2 = *ior;
                n1 = 1.; // ior of water, TODO add to material
                cos = self.ray.direction.scale(-1.).dot(&self.normal);
                normal = self.normal.scale(-1.);
            } else { // exiting a medium
                n1 = *ior;
                n2 = 1.;
                normal = self.normal;
            }

            let ior_frac = n1/n2;
            let incident_comp = ior_frac;

            let sin_theta_t = (ior_frac * ior_frac) * (1. - cos * cos);
            if 1. - sin_theta_t < 0. { // total interal reflection
                return None;
            }
            let normal_comp = ior_frac*cos - (1. - sin_theta_t).sqrt();

            let dir = self.ray.direction.scale(incident_comp).add(&self.normal.scale(normal_comp)).norm();
            Some(Ray::new(self.biased_point(&normal),dir,1.))
        } else {
            None
        }
    }

    pub fn add_bias(&mut self) {
        //self.ray.origin = self.ray.origin.add(&self.normal.scale(1e-7));
        self.ray.t -= 1e-7;
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
            if min > 0.0 {
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

impl Shadable for Plane {
    fn normal(&self, _p: &Point3) -> Option<Vec3> {
        Some(self.normal)
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let ray_normal = ray.direction.dot(&self.normal);

        if ray_normal.abs() > 0.001f64 {
            let t = self.origin.sub(&ray.origin).dot(&self.normal) / ray_normal;
            if t > 0.001f64 {
                return Some(Intersection::new(self.normal(&ray.at_t(0.)).unwrap(),
                                              &self.material,
                                              ray.clone_with_t(t),
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

#[cfg(test)]
mod test {
    use super::*;
    #[allow(unused_imports)]
    use crate::approx::RelativeEq;

    use std::f64;
    use crate::predef::materials::WATER;

    #[test]
    fn test_refr_ray_mag() {
        let normal = Vec3::new(0.,1.,0.);
        let ray_origin = Point3::new(-0.5,3_f64.sqrt()/2.,0.);
        let origin = Point3::new(0.,0.,0.);
        let ray = Ray::new(ray_origin,origin.sub(&ray_origin),1.); // form a ray with 30 deg offset from normal
        let hit = Intersection::new(Vec3::new(0.,1.,0.),&WATER,ray);
        let refr_ray = hit.refract().unwrap();
        assert_relative_eq!(1.,refr_ray.direction.mag());
    }

    #[test]
    fn test_refract_angle() {
        let normal = Vec3::new(0.,1.,0.);
        let ray_origin = Point3::new(-0.5,3_f64.sqrt()/2.,0.);
        let origin = Point3::new(0.,0.,0.);
        let ray = Ray::new(ray_origin,origin.sub(&ray_origin),1.); // form a ray with 30 deg offset from normal
        let hit = Intersection::new(normal,&WATER,ray);
        let refr_ray = hit.refract().unwrap();
        println!("refr_ray: {:?}",refr_ray.direction);
        assert_relative_eq!(refr_ray.direction.dot(&hit.normal.scale(-1.)).acos() * 180./f64::consts::PI, 0.375_f64.asin() * 180./f64::consts::PI)
    }

}
