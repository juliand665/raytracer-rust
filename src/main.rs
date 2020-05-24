#![feature(array_value_iter)]
#![feature(iterator_fold_self)]
#![feature(tau_constant)]

extern crate image as image_lib;
extern crate num_cpus;
extern crate rand;

mod camera;
mod color;
mod image;
mod material;
mod rendering;
mod scene;
mod shape;
mod tracing;
mod vectors;

pub use crate::image::*;
pub use camera::*;
pub use color::*;
pub use material::*;
pub use rendering::*;
pub use scene::*;
pub use shape::*;
pub use tracing::*;
pub use vectors::*;

use std::fs::*;
use std::sync::Arc;
use std::time::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let camera = Simple3DCamera::new(
        Vec3::new(0.0, 0.0, -20.0),
        Vec3::positive_z(),
        Vec3::positive_y(),
    );

    let mut scene = VecScene::new();

    let diffuse = DiffuseMaterial {
        color: Color::white(),
    };
    let light = FlatColorMaterial {
        color: Color::new_gray(5.0, 1.0),
    };
    let mirror = MirrorMaterial;

    scene.add(MaterialShape {
        material: mirror,
        shape: Sphere {
            center: Vec3::new(-5.0, -5.0, 5.0),
            radius: 3.0,
        },
    });

    scene.add(MaterialShape {
        material: diffuse,
        shape: Sphere {
            center: Vec3::new(5.0, -7.0, 5.0),
            radius: 3.0,
        },
    });

    scene.add(MaterialShape {
        material: light,
        shape: Sphere {
            center: Vec3::new(0.0, 100.0, 0.0),
            radius: 90.1,
        },
    });

    {
        let pale_red = Color::new(0.75, 0.25, 0.25, 1.0);
        let pale_blue = Color::new(0.25, 0.25, 0.75, 1.0);
        let details = vec![
            (pale_red, Vec3::new(-1000.0, 0.0, 0.0)),
            (pale_blue, Vec3::new(1000.0, 0.0, 0.0)),
            (Color::white(), Vec3::new(0.0, -1000.0, 5.0)),
            (Color::white(), Vec3::new(0.0, 1000.0, 5.0)),
            (Color::white(), Vec3::new(0.0, 0.0, 1000.0)),
            (Color::white(), Vec3::new(0.0, 0.0, -1000.0)),
        ];
        for (color, center) in details {
            scene.add(MaterialShape {
                material: DiffuseMaterial { color },
                shape: Sphere {
                    center,
                    radius: 990.0,
                },
            });
        }
    }

    let raytracer = Raytracer {
        camera,
        element: scene,
    };

    let options = TracingOptions {
        background_color: Color::clear(),
        max_bounces: 5,
        near_clipping: 0.0001,
    };

    let start = Instant::now();
    let raytracer_ref = Arc::new(raytracer);
    let image = render_image(raytracer_ref, 1024, 1024, 1000, &options);
    let duration = Instant::now().duration_since(start);
    println!(
        "Finished rendering in {}.{}s",
        duration.as_secs(),
        duration.subsec_millis(),
    );

    image.write_png(File::create("render.png")?)?;

    Ok(())
}
