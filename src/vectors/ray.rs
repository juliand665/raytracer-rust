use super::*;

#[derive(Debug, Copy, Clone)]
pub struct Ray<V: Vector> {
    pub origin: V,
    pub direction: Normalized<V>,
}

impl<V: Vector> Ray<V> {
    pub fn new(origin: V, direction: V) -> Self {
        Self {
            origin,
            direction: direction.normalized(),
        }
    }

    pub fn at(&self, t: Component) -> V {
        self.origin + *self.direction * t
    }
}
