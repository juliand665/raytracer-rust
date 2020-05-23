use super::*;

pub struct Intersection<Data> {
    pub distance: Component,
    pub data: Data,
}

pub struct Hit<V: Vector> {
    pub ray_direction: Normalized<V>,
    pub intersection: V,
    pub normal: Normalized<V>,
}

type IntersectionResult<V> = Option<Intersection<Hit<V>>>;

pub trait Shape: 'static + Send + Sync {
    type V: Vector;

    fn first_intersection(
        &self,
        ray: &Ray<Self::V>,
        near_clipping: Component,
    ) -> IntersectionResult<Self::V>;
}

pub type Circle = NSphere<Vec2>;
pub type Sphere = NSphere<Vec3>;
pub type Hypersphere = NSphere<Vec4>;

pub struct NSphere<V: Vector> {
    pub center: V,
    pub radius: Component,
}

impl<V: Vector> Shape for NSphere<V> {
    type V = V;

    fn first_intersection(&self, ray: &Ray<V>, near_clipping: Component) -> IntersectionResult<V> {
        let offset_center = self.center - ray.origin;
        // check that sphere is in front of the ray
        if offset_center.dot(*ray.direction) <= 0.0 {
            return None;
        }
        // project sphere center onto ray
        let projection_length = offset_center.dot(*ray.direction);
        let projection = *ray.direction * projection_length;
        // calculate distance from projection to sphere edge (pythagoras)
        let hypotenuse_sq = self.radius.squared();
        let cathetus_sq = (offset_center - projection).squared_sum();
        if hypotenuse_sq < cathetus_sq {
            return None; // TODO: when does this occur?
        }
        let distance = projection_length - (hypotenuse_sq - cathetus_sq).sqrt();
        if distance < near_clipping {
            None
        } else {
            let intersection = ray.at(distance);
            Some(Intersection {
                distance,
                data: Hit {
                    ray_direction: ray.direction,
                    intersection,
                    normal: (intersection - self.center).normalized(),
                },
            })
        }
    }
}

trait Squareable {
    fn squared(self) -> Self;
}

impl Squareable for Component {
    fn squared(self) -> Self {
        self * self
    }
}
