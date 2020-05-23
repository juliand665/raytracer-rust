use super::*;

pub fn render_image<V: Vector>(
    raytracer: Raytracer<V>,
    width: usize,
    height: usize,
    samples: usize,
    options: &TracingOptions,
) -> Image {
    assert!(samples > 0);

    let scaled_width = width as Component / 2.0;
    let scaled_height = height as Component / -2.0;
    let samples_f = samples as Component;

    // TODO: check that these values make sense
    let x_offset = 1.0 / width as Component - 1.0;
    let y_offset = 1.0 / height as Component + 1.0;

    let mut image = Image::new(width, height);
    for y in 0..height {
        println!("tracing line {}/{}", y + 1, height);
        let mut row: Vec<Color> = (0..width)
            .into_iter()
            .map(|x| {
                let offset = Vec2::new(
                    x as Component / scaled_width + x_offset,
                    y as Component / scaled_height + y_offset,
                );
                (0..samples)
                    .into_iter()
                    .map(|_| raytracer.trace(offset, options))
                    .fold_first(|c1, c2| c1 + c2)
                    .unwrap()
                    / samples_f
            })
            .collect();
        image.row(y).swap_with_slice(row.as_mut_slice());
    }
    image
}
