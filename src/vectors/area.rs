use super::*;
use rand::*;

#[derive(Clone)]
pub struct VectorArea<V: Vector> {
    pub x0_y0: V,
    pub x0_y1: V,
    pub x1_y0: V,
    pub x1_y1: V,
}

impl VectorArea<Vec2> {
    pub fn new_with_corner_2d(x0_y0: Vec2, dx: Component, dy: Component) -> Self {
        Self::new_with_corner(x0_y0, Vec2::new(dx, 0.0), Vec2::new(0.0, dy))
    }
}

impl<V: Vector> VectorArea<V> {
    pub fn new_with_corner(x0_y0: V, dx: V, dy: V) -> Self {
        Self {
            x0_y0,
            x0_y1: x0_y0 + dy,
            x1_y0: x0_y0 + dx,
            x1_y1: x0_y0 + dx + dy,
        }
    }

    pub fn vector(&self, x: Component, y: Component) -> V {
        let y0 = self.x0_y0.lerp(self.x1_y0, x);
        let y1 = self.x0_y1.lerp(self.x1_y1, x);
        y0.lerp(y1, y)
    }

    pub fn random_vector(&self) -> V {
        let mut rng = thread_rng();
        self.vector(rng.gen(), rng.gen())
    }
}
