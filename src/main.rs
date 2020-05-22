mod vectors;
pub use vectors::*;
fn main() {
    let v = Vec3::new(1.0, 2.0, 3.0);
    println!("{}", v + v);
}
