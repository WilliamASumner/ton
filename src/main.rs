//use std::fs::File;
//use std::io;
//use std::io::prelude::*;
use ton::math::point3::Point3;
use ton::primitives::camera::Camera;
use ton::primitives::material::Color;
use ton::primitives::material::Material;
use ton::primitives::material::MaterialType;
use ton::primitives::material::{RED,BLUE,WHITE,BLACK};
use ton::primitives::PointLight;
use ton::primitives::Sphere;

use ton::scene::Scene;

fn main() {
    let red_spec = Material::new(RED, WHITE, 0.1, MaterialType::Refractive);
    let red_diffuse = Material::new(RED, WHITE, 0.1, MaterialType::Diffuse);
    let blue_diffuse = Material::new(WHITE, WHITE, 0.1, MaterialType::Diffuse);
    let cyan_diffuse = Material::new(Color::new(0.,0.5,0.75), WHITE, 0.1,MaterialType::Diffuse);

    let mut scene = Scene::new();
    let sphere_one = Sphere::new(Point3::new(-0.25, 0., 0.), 0.3, red_spec);
    let sphere_two = Sphere::new(Point3::new(0.25, 0.2, -0.2), 0.25, blue_diffuse);
    let sphere_three = Sphere::new(Point3::new(0.25, 0.3, 0.5), 0.1, cyan_diffuse);
    let sphere_four = Sphere::new(Point3::new(-0.45, 0.6, 0.1), 0.3, cyan_diffuse);
    let sphere_five = Sphere::new(Point3::new(-0.25, 0., -0.5), 0.3, red_diffuse);

    let mut camera = Camera::default();
    camera.set_origin(Point3::new(0., 0., 1.))
          .point_at(&Point3::new(0., 0., 0.));
    let mut light = PointLight::default();
    let mut light2 = PointLight::default();
    light.move_to(&Point3::new(1., 0., 0.1));
    light2.move_to(&Point3::new(-2., -1.5, 0.));

    scene.set_camera(camera);
    scene.add_object(sphere_one);
    scene.add_object(sphere_two);
    scene.add_object(sphere_three);
    scene.add_object(sphere_four);
    scene.add_object(sphere_five);
    scene.add_light(light);
    scene.add_light(light2);
    scene.set_background_col(BLACK);

    scene.render().save("output/test.png").ok();
}
