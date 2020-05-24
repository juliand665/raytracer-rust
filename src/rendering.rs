use super::*;
use std::sync::mpsc;
use std::thread;

pub fn render_image<V: Vector, C: Camera<V = V>, E: SceneElement<V = V>>(
    raytracer: Arc<Raytracer<V, C, E>>,
    width: usize,
    height: usize,
    samples: usize,
    options: &TracingOptions,
) -> Image {
    assert!(samples > 0);

    let width_f = width as Component;
    let height_f = height as Component;
    let diagonal = Component::hypot(width_f, height_f) / 2.0;
    let center = Vec2::new((width_f - 1.0) / 2.0, (height_f - 1.0) / 2.0);

    let samples_f = samples as Component;

    let mut image = Image::new(width, height);

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];
    let thread_count = num_cpus::get();
    for thread_index in 0..thread_count {
        let tx = tx.clone();
        let options = options.clone();
        let min_y = height * thread_index / thread_count;
        let max_y = height * (thread_index + 1) / thread_count;
        let raytracer = raytracer.clone();
        let handle = thread::spawn(move || {
            for y in min_y..max_y {
                let row: Vec<Color> = (0..width)
                    .into_iter()
                    .map(|x| {
                        let offset = (Vec2::new(x as Component, (height - y - 1) as Component)
                            - center)
                            / diagonal;
                        let sum = (0..samples)
                            .into_iter()
                            .map(|_| raytracer.trace(offset, &options))
                            .fold_first(|c1, c2| c1 + c2)
                            .unwrap();
                        (sum / samples_f).clamped()
                    })
                    .collect();
                tx.send((y, row)).unwrap();
            }
        });
        handles.push(handle);
    }
    for line_num in 0..height {
        let (y, mut row) = rx.recv().unwrap();
        image.row(y).swap_with_slice(row.as_mut_slice());
        println!("received line {}/{}", line_num, height);
    }
    image
}
