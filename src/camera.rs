use super::*;

pub trait Camera {
    type V: Vector;

    fn position(&self) -> Self::V;
    fn set_position(&mut self, position: Self::V);

    fn ray(&self, offset: Vec2) -> Ray<Self::V>;
}

pub struct Simple3DCamera {
    position: Vec3,
    forward: Normalized<Vec3>,
    up: Normalized<Vec3>,
    right: Normalized<Vec3>,
}

impl Simple3DCamera {
    pub fn new(position: Vec3, forward: Normalized<Vec3>, up: Normalized<Vec3>) -> Self {
        let right = forward.cross(up);
        Self {
            position,
            forward,
            up: right.cross(forward),
            right,
        }
    }
}

impl Camera for Simple3DCamera {
    type V = Vec3;

    fn position(&self) -> Self::V {
        self.position
    }

    fn set_position(&mut self, position: Self::V) {
        self.position = position;
    }

    fn ray(&self, offset: Vec2) -> Ray<Self::V> {
        let offset = self.right * offset.x + self.up * offset.y;
        Ray::new(self.position, self.forward + offset)
    }
}