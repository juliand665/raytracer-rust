use super::*;

pub trait Scene: SceneElement {
    fn add<E: SceneElement<V = Self::V>>(&mut self, element: E);
}

type IntersectionResult<V> = Option<Intersection<Behavior<V>>>;

pub trait SceneElement: 'static {
    type V: Vector;

    fn first_intersection(
        &self,
        ray: Ray<Self::V>,
        near_clipping: Component,
    ) -> IntersectionResult<Self::V>;
}

pub struct VecScene<V: Vector> {
    elements: Vec<Box<dyn SceneElement<V = V>>>,
}

impl<V: Vector> Scene for VecScene<V> {
    fn add<E: SceneElement<V = Self::V>>(&mut self, element: E) {
        self.elements.push(Box::new(element));
    }
}

impl<V: Vector> SceneElement for VecScene<V> {
    type V = V;

    fn first_intersection(
        &self,
        ray: Ray<Self::V>,
        near_clipping: Component,
    ) -> IntersectionResult<Self::V> {
        self.elements
            .iter()
            .filter_map(|e| e.first_intersection(ray, near_clipping))
            .min_by(|l, r| l.distance.partial_cmp(&r.distance).unwrap())
    }
}