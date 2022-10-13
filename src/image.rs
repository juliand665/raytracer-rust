use super::*;
use image_lib::*;
use std::io::Write;
use std::iter::*;

pub struct Image {
    pixels: Vec<Color>,
    width: usize,
    height: usize,
}

impl Image {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![Color::clear(); width * height],
            width,
            height,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn index(&self, x: usize, y: usize) -> usize {
        x + y * self.width
    }

    pub fn row_mut(&mut self, y: usize) -> &mut [Color] {
        let range = self.index(0, y)..self.index(0, y + 1);
        &mut self.pixels[range]
    }

    pub fn pixels_mut(&mut self) -> &mut [Color] {
        &mut self.pixels
    }

    pub fn write_png<W: Write>(&self, target: W) -> ImageResult<()> {
        let encoder = png::PNGEncoder::new(target);

        fn byte(value: Component) -> u8 {
            (value * 255.0) as u8
        }
        let data: Vec<u8> = self
            .pixels
            .iter()
            .flat_map(|px| [byte(px.red), byte(px.green), byte(px.blue), byte(px.alpha)])
            .collect();

        encoder.encode(
            data.as_slice(),
            self.width as u32,
            self.height as u32,
            ColorType::Rgba8,
        )
    }
}
