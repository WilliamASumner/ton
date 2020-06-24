extern crate image;
use image::{ImageBuffer, Rgb};

use crate::math::ray::Ray;
use crate::primitives::camera::Camera;
use crate::primitives::material::Color;
use crate::primitives::material::MaterialType;
use crate::primitives::Intersection;
use crate::primitives::LightSource;
use crate::primitives::Shadable;


pub struct Scene<'a> {
    camera: Camera,
    primitives: Vec<Box<dyn Shadable + 'a>>,
    lights: Vec<Box<dyn LightSource + 'a>>,
    background_col: Color,
}

impl<'a> Scene<'a> {
    pub fn new() -> Scene<'a> {
        Default::default()
    }

    pub fn set_camera(&mut self, c: Camera) {
        self.camera = c;
    }

    pub fn set_background_col(&mut self, col: Color) {
        self.background_col = col;
    }

    pub fn add_object<T: Shadable + 'a>(&mut self, obj: T) {
        self.primitives.push(Box::new(obj));
    }

    pub fn add_light<L: LightSource + 'a>(&mut self, light: L) {
        self.lights.push(Box::new(light));
    }

    pub fn print_objects(&self) {
        for object in self.primitives.iter() {
            print!("{:?}", object);
        }
        println!();
    }

    pub fn find_nearest_intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut current_hit = None;
        for obj in self.primitives.iter() {
            // find nearest hit
            if let Some(intersection) = obj.intersect(&ray) {
                match current_hit {
                    Some(hit) => {
                        if intersection < hit {
                            current_hit = Some(intersection);
                        } else {
                            current_hit = Some(hit);
                        }
                    }
                    None => current_hit = Some(intersection),
                }
            }
        }
        current_hit
    }

    pub fn is_occluded(&self, ray: &Ray) -> bool {
        for obj in self.primitives.iter() {
            if let Some(_intersection) = obj.intersect(&ray) {
                return true;
            }
        }
        false
    }

    pub fn reflect_off(&self, hit: &Intersection) -> Ray {
        let ang = hit.ray.direction.dot(&hit.normal);
        let dir = hit.ray.direction.sub(&hit.normal.scale(2.*(ang)));
        Ray::new(hit.point(),dir,1.)
    }

    pub fn refract_through(&self, hit: &Intersection, prev_ior: f64) -> Option<Ray> {
        let cos = hit.ray.direction.dot(&hit.normal);
        let ior_frac = prev_ior / 1.333; // ior of water, TODO add to material
        let sin_theta_t = ior_frac.powi(2) * (1. - cos.powi(2));
        //if sin_theta_t > 0.9 {
            //return None;
        //}
        let incident_comp = ior_frac;
        let normal_comp = ior_frac*cos - (1. - sin_theta_t.sqrt());
        let dir = hit.ray.direction.scale(incident_comp).add(&hit.normal.scale(normal_comp)).norm();
        Some(Ray::new(hit.point(),dir,1.))
    }

    pub fn shade(&self, hit: Intersection, depth: u32, max_depth: u32) -> Color {
        let mut col = self.background_col.mix(&hit.material.diff_col, 0.9); // start off as background col + some material color
        if depth == max_depth {
            return col;
        }
        let hit_point = hit.point();

        match hit.mat() {
            MaterialType::Diffuse => {
                for light in self.lights.iter() {
                    let shadow_ray = light.trace_light(&hit_point);
                    if !self.is_occluded(&shadow_ray) {
                        let cos = shadow_ray.direction().dot(&hit.normal).abs();
                        col.add(&hit.material.diff_col.mult(cos));
                        col = col.clamp(); // TODO add tone mapping to remove this
                    }
                }
                col
            },

            MaterialType::Specular => {
                let refl_ray = self.reflect_off(&hit);
                let new_hit = self.find_nearest_intersect(&refl_ray);
                match new_hit {
                    Some(intersection) => self.shade(intersection,depth+1,max_depth).mix(&hit.material.diff_col,0.9),
                    None => col,
                }
            },

            MaterialType::Refractive => {
                let refr_ray = self.refract_through(&hit,1.);
                let new_hit = match refr_ray {
                    Some(ray) => self.find_nearest_intersect(&ray),
                    None => None
                };
                match new_hit {
                    Some(intersection) => self.shade(intersection,depth+1,max_depth),
                    None => col,
                }
            },

            _ => panic!("MaterialType not implemented!"),
        }
    }

    pub fn render(&mut self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut buff = self.camera.new_buffer();

        for (x, y, pixel) in buff.enumerate_pixels_mut() {
            let ray = self.camera.generate_ray(x, y);
            let hit = self.find_nearest_intersect(&ray);

            *pixel = match hit {
                Some(intersection) => self.shade(intersection,0,32).to_rgb(),
                None => self.background_col.to_rgb(),
            };
        }
        buff
    }
}

impl<'a> Default for Scene<'a> {
    fn default() -> Scene<'a> {
        Scene {
            camera: Camera::default(),
            primitives: Vec::new(),
            lights: Vec::new(),
            background_col: Color::new(0.1, 0.1, 0.1),
        }
    }
}
