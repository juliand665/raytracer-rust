use super::*;

pub struct CameraLocation<V: Vector> {
    pub position: V,
    pub forward: V,
    pub up: V,
}

trait Camera {
    type V: Vector;
    type Offset: Vector;

    fn location(&self) -> CameraLocation<Self::V>;
    fn set_location(&mut self, location: CameraLocation<Self::V>);
}
