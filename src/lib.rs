#[cfg(test)]
#[macro_use]
extern crate approx;
//extern crate rand;
pub mod math;
pub mod primitives;
pub mod scene;
pub mod predef;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
