use super::*;
use std::sync::mpsc;
use std::thread;

pub fn render_image<V: Vector, C: Camera<V = V>, E: SceneElement<V = V>>(
    raytracer: &Raytracer<V, C, E>,
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

    let (tx, rx) = mpsc::channel();
    let handles = vec![];
    let thread_count = num_cpus::get();
    for thread_index in 0..thread_count {
        let tx = tx.clone();
        let options = options.clone();
        let min_y = height * thread_index / thread_count;
        let max_y = height * (thread_index + 1) / thread_count;
        let handle = thread::spawn(move || {
            for y in min_y..max_y {
                println!("tracing line {}/{}", y + 1, height);
                let mut row: Vec<Color> = (0..width)
                    .into_iter()
                    .map(|x| {
                        let offset = Vec2::new(
                            x as Component / scaled_width + x_offset,
                            y as Component / scaled_height + y_offset,
                        );
                        let sum = (0..samples)
                            .into_iter()
                            .map(|_| raytracer.trace(offset, &options))
                            .fold_first(|c1, c2| c1 + c2)
                            .unwrap();
                        (sum / samples_f).clamped()
                    })
                    .collect();
                tx.send((y, row));
            }
        });
        handles.push(handle);
    }
    for _ in 0..height {
        let (y, row) = rx.recv().unwrap();
        image.row(y).swap_with_slice(row.as_mut_slice());
    }
    image
}
