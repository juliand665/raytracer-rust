use super::*;
use rayon::prelude::*;
use std::sync::atomic::*;

pub fn render_image<V: Vector, C: Camera<V = V>, E: SceneElement<V = V>>(
    raytracer: &Raytracer<V, C, E>,
    width: usize,
    height: usize,
    samples: usize,
    options: &TracingOptions,
) -> Image {
    assert!(samples > 0);

    let width_f = width as Component;
    let height_f = height as Component;
    let diagonal = Component::hypot(width_f, height_f) / 2.0;
    let center = Vec2::new(width_f, height_f) / 2.0;
    let pixel_size = 1.0 / diagonal;

    let samples_f = samples as Component;

    let lines_traced = AtomicU32::new(0);
    let mut image = Image::new(width, height);
    image
        .pixels_mut()
        .par_chunks_mut(width)
        .enumerate()
        .for_each(|(y, pixels)| {
            for (x, pixel) in (0..width).zip(pixels) {
                let area = VectorArea::new_with_corner_2d(
                    (Vec2::new(x as Component, (height - y - 1) as Component) - center) / diagonal,
                    pixel_size,
                    pixel_size,
                );
                let sum = (0..samples)
                    .map(|_| raytracer.trace(&area, &options))
                    .fold_first(|c1, c2| c1 + c2)
                    .unwrap();
                *pixel = (sum / samples_f).clamped();
            }
            let previous = lines_traced.fetch_add(1, Ordering::Relaxed);
            println!("traced line {}/{}", previous + 1, height);
        });
    image
}
