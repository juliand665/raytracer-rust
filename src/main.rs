#![feature(tau_constant)]

extern crate rand;

mod camera;
mod color;
mod material;
mod scene;
mod shape;
mod tracing;
mod vectors;

pub use camera::*;
pub use color::*;
pub use material::*;
pub use scene::*;
pub use shape::*;
pub use tracing::*;
pub use vectors::*;

fn main() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    println!("{}", v + v);
}
