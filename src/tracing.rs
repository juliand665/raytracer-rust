use super::*;

#[derive(Clone)]
pub struct TracingOptions {
    pub background_color: Color,
    pub max_bounces: usize,
    pub near_clipping: Component,
}

pub struct Raytracer<V: Vector, C: Camera<V = V>, E: SceneElement<V = V>> {
    pub camera: C,
    pub element: E,
}

impl<V: Vector, C: Camera<V = V>, E: SceneElement<V = V>> Raytracer<V, C, E> {
    pub fn trace(&self, offset: Vec2, options: &TracingOptions) -> Color {
        let ray = self.camera.ray(offset);
        self.rec_trace(ray, options, options.max_bounces)
    }

    fn rec_trace(&self, ray: Ray<V>, options: &TracingOptions, bounces_left: usize) -> Color {
        let mut color = options.background_color;

        if let Some(intersection) = self.element.first_intersection(ray, options.near_clipping) {
            let behavior = intersection.data;
            color = behavior.emission;

            if bounces_left > 0 {
                if let Some(next_bounce) = behavior.next_bounce {
                    color = color
                        + behavior.color * self.rec_trace(next_bounce, options, bounces_left - 1);
                }
            }
        }

        color
    }
}
