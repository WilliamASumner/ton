//extern crate rand;
pub mod math;
pub mod primitives;
pub mod scene;
//pub mod vec3;
//pub mod ppm;
//pub mod mat;
//pub mod scene;
//pub mod tracer;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
