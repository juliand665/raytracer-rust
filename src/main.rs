#![feature(array_value_iter)]
#![feature(iterator_fold_self)]
#![feature(tau_constant)]

extern crate image as image_lib;
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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let camera = Simple3DCamera::new(
        Vec3::zero(),
        Vec3::new(0.0, 0.0, 1.0).normalized(),
        Vec3::new(0.0, 1.0, 0.0).normalized(),
    );

    let mut scene = VecScene::new();

    let diffuse = DiffuseMaterial {
        color: Color::new(0.8, 0.2, 0.2, 1.0),
    };
    let light = FlatColorMaterial {
        color: Color::new_gray(5.0, 1.0),
    };
    let mirror = MirrorMaterial;

    scene.add(MaterialShape {
        material: mirror,
        shape: Sphere {
            center: Vec3::new(-2.0, -3.0, 7.0),
            radius: 2.0,
        },
    });

    scene.add(MaterialShape {
        material: diffuse,
        shape: Sphere {
            center: Vec3::new(2.0, -3.0, 5.0),
            radius: 2.0,
        },
    });

    scene.add(MaterialShape {
        material: light,
        shape: Sphere {
            center: Vec3::new(0.0, 100.0, 5.0),
            radius: 90.2,
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
            (Color::white(), Vec3::new(0.0, 0.0, 1005.0)),
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
        camera: Box::new(camera),
        element: Box::new(scene),
    };

    let options = TracingOptions {
        background_color: Color::clear(),
        max_bounces: 5,
        near_clipping: 0.0001,
    };

    let image = render_image(raytracer, 256, 256, 50, &options);

    image.write_png(File::create("render.png")?)?;

    Ok(())
}
