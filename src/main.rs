//use std::fs::File;
//use std::io;
//use std::io::prelude::*;
use ton::math::point3::Point3;
use ton::math::vec3::Vec3;
use ton::primitives::camera::Camera;
use ton::primitives::material::{Material,Color};
use ton::predef::colors;
use ton::primitives::{PointLight,Sphere,Plane};
use ton::scene::Scene;

fn main() {
    let red_refractive = Material::refractive(colors::WHITE,colors::WHITE,1.5);
    let red_diffuse = Material::diffuse(colors::RED);
    let _red_specular = Material::specular(colors::RED);
    let blue_diffuse = Material::diffuse(colors::BLUE);
    let grey_diffuse = Material::diffuse(Color::new(0.25,0.25,0.25));

    let mut scene = Scene::new();
    let sphere_one   = Sphere::new(Point3::new(0., 0., -0.5), 0.25, red_diffuse);
    let sphere_two   = Sphere::new(Point3::new(-0.3, 0.0, 0.2), 0.25, blue_diffuse);
    let sphere_three = Sphere::new(Point3::new(0.2, 0., 0.1), 0.25, red_refractive);

    let plane_one = Plane::new(Point3::new(0.,-0.25,0.),Vec3::new(0.,1.,0.),grey_diffuse);

    let mut camera = Camera::default();
    camera.set_origin(Point3::new(0., 0., 1.))
          .point_at(&Point3::new(0., 0., 0.));

    let mut light = PointLight::default();

    light.move_to(&Point3::new(2., 5., 0.));

    scene.set_camera(camera);

    scene.add_object(sphere_one);
    //scene.add_object(sphere_two);
    scene.add_object(sphere_three);

    scene.add_object(plane_one);

    scene.add_light(light);

    scene.set_background_col(colors::BLACK);

    scene.render().save("output/test.png").ok();
}
