use super::*;
use rand::*;
use std::f32::consts;

pub struct Behavior<V: Vector> {
    pub emission: Color,
    pub color: Color,
    pub next_bounce: Option<Ray<V>>,
}

pub trait Material<V: Vector>: 'static + Send + Sync {
    fn behavior(&self, hit: Hit<V>) -> Behavior<V>;
}

pub struct FlatColorMaterial {
    pub color: Color,
}

impl<V: Vector> Material<V> for FlatColorMaterial {
    fn behavior(&self, _hit: Hit<V>) -> Behavior<V> {
        Behavior {
            emission: self.color,
            color: self.color,
            next_bounce: None,
        }
    }
}

pub struct MirrorMaterial;

impl<V: Vector> Material<V> for MirrorMaterial {
    fn behavior(&self, hit: Hit<V>) -> Behavior<V> {
        let normal = hit.normal;
        let dir = hit.ray_direction;
        let reflected = dir - normal * 2.0 * (dir.dot(normal));
        Behavior {
            emission: Color::black(),
            color: Color::white(),
            next_bounce: Some(Ray::new(hit.intersection, reflected)),
        }
    }
}

pub struct DiffuseMaterial {
    pub color: Color,
}

impl Material<Vec3> for DiffuseMaterial {
    fn behavior(&self, hit: Hit<Vec3>) -> Behavior<Vec3> {
        // construct basis to apply random angles to
        let w = if hit.normal.dot(hit.ray_direction) > 0.0 {
            -hit.normal // hit from inside
        } else {
            hit.normal // hit from outside
        };
        let u = Vec3::new(1.0, 0.0, 0.0).cross(w).normalized();
        let v = w.cross(u).normalized();

        // generate random angles
        // some math taken from http://corysimon.github.io/articles/uniformdistn-on-sphere/
        let mut rng = thread_rng();
        let azimuth = rng.gen::<Component>() * consts::TAU;
        let cos_elevation = rng.gen::<Component>(); // ensures uniform distribution across (hemi-) sphere surface
        let sin_elevation = (1.0 - cos_elevation * cos_elevation).sqrt();

        // factors for each vector in our basis
        let x = cos_elevation * azimuth.cos();
        let y = cos_elevation * azimuth.sin();
        let z = sin_elevation;

        let bounce_direction = u * x + v * y + w * z;

        Behavior {
            emission: Color::black(),
            color: self.color,
            next_bounce: Some(Ray::new(hit.intersection, bounce_direction)),
        }
    }
}
