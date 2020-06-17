use crate::math::point3::Point3;
use crate::math::ray::Ray;
use crate::math::vec3::Vec3;
use image::{ImageBuffer, Rgb};

#[derive(Debug)]
pub struct Camera {
    pub origin: Point3,
    view_vec: Vec3,
    up: Vec3,
    right: Vec3,
    fstop: f64,
    film_size: f64,
    pub res_x: u32,
    pub res_y: u32,
}

impl Camera {
    pub fn new(
        o: Point3,
        v: Vec3,
        u: Vec3,
        r: Vec3,
        f: f64,
        size: f64,
        r_x: u32,
        r_y: u32,
    ) -> Camera {
        Camera {
            origin: o,
            view_vec: v,
            up: u,
            right: r,
            fstop: f,
            film_size: size,
            res_x: r_x,
            res_y: r_y,
        }
    }

    pub fn set_resolution(&mut self, x: u32, y: u32) -> &mut Self {
        self.res_x = x;
        self.res_y = y;
        self
    }

    pub fn new_buffer(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        ImageBuffer::new(self.res_x, self.res_y)
    }

    pub fn set_origin(&mut self, p: Point3) -> &mut Self {
        let old_point = self.origin.add(&self.view_vec);
        self.view_vec = p.sub(&old_point).norm();
        self.origin = p;
        self
    }

    pub fn point_at(&mut self, p: &Point3) -> &mut Self {
        self.view_vec = p.sub(&self.origin).norm();
        self
    }

    pub fn generate_ray(&self, pix_x: u32, pix_y: u32) -> Ray {
        if pix_x >= self.res_x || pix_y >= self.res_y {
            panic!(
                "Invalid coordinates {}, {} with resolution {} x {}",
                pix_x, pix_y, self.res_x, self.res_y
            )
        }

        /* 0 +----------------------------+
         *   |                            |
         *   | *                          |
         *   | (2,3)                      |
         *   |                            |
         * x +----------------------------+
         *   0                            y
         *   multiply y by x/y
         * 0 +----------------------------+
         *   |                            |
         *   | *                          |
         *   | (2,3*x/y)                  |
         *   |                            |
         * x +----------------------------+
         *   0                            x
         *
         *   divide by x and add 0.5
         * 1 +----------------------------+
         *   |                            |
         *   | *                          |
         *   | (2/x + 0.5,3/y + 0.5)      |
         *   |                            |
         *-1 +----------------------------+
         *   -1                           1
         */

        let ratio = self.res_y as f64 / self.res_x as f64;
        let transform_x = 2. * (pix_x as f64 / self.res_x as f64) - 1.;
        let transform_y = (2. * (pix_y as f64 / self.res_y as f64) - 1.) * ratio;
        let offset = Vec3::new(transform_x, transform_y, 0.);
        let p = self.origin.add(&self.view_vec).add(&offset);

        Ray::new(self.origin, p.sub(&self.origin).norm(), 1.0)
    }
}

impl Default for Camera {
    fn default() -> Camera {
        let o = Point3::new(0., 0., 0.);
        let v = Vec3::new(0., 0., -1.); // negative z
        let u = Vec3::new(0., 0., 1.);
        let r = v.cross(&u).norm();
        Camera {
            origin: o,
            view_vec: v,
            up: u,
            right: r,
            fstop: 1.0,
            film_size: 1.0,
            res_x: 640,
            res_y: 480,
        }
    }
}
