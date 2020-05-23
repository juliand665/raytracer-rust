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
            emission: Color::clear(),
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
        // TODO: all this is way too magical rn--improve own understanding and then naming

        let mut rng = thread_rng();
        let azimuth = rng.gen::<Component>() * 2.0 * consts::TAU;
        let polar = rng.gen::<Component>();
        // TODO: try exchanging some of these with basic random doubles
        let x = polar.sin() * azimuth.cos();
        let y = polar.sin() * azimuth.sin();
        let z = polar.cos();

        let n = if hit.normal.dot(hit.ray_direction) > 0.0 {
            -hit.normal
        } else {
            hit.normal
        }; // TODO: necessary? not sure this can even happen
        let w = n;
        let u = Vec3::new(1.0, 0.0, 0.0).cross(w).normalized();
        let v = w.cross(u).normalized();
        let bounce_direction = u * x + v * y + w * z;

        Behavior {
            emission: Color::black(),
            color: self.color,
            next_bounce: Some(Ray::new(hit.intersection, bounce_direction)),
        }
    }
}
