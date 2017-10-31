use std::f64;

use image::{self, Pixel};
use cgmath::Vector2;

use grid::Grid;

#[derive(Debug, Clone)]
pub struct ImageGradient {
    values: Vec<Vector2<f64>>,
    width: u32,
    height: u32,
}

impl Grid for ImageGradient {
    type Element = Vector2<f64>;

    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn data(&self) -> &[Vector2<f64>] {
        &self.values
    }
    fn data_mut(&mut self) -> &mut [Vector2<f64>] {
        &mut self.values
    }
}

impl ImageGradient {
    pub fn from_luma_image(image: &image::RgbaImage) -> ImageGradient {
        let (width, height) = (image.width(), image.height());

        let total_size = (width as usize) * (height as usize);
        let mut gradient = Vec::with_capacity(total_size);
        gradient.resize(total_size, Vector2::new(0.0, 0.0));

        for y in 0..height {
            for x in 1..(width - 1) {
                let index = x + y * width;
                let val = f64::from(image[(x + 1, y)].to_luma().data[0]) -
                    f64::from(image[(x - 1, y)].to_luma().data[0]);
                gradient[index as usize].x = val;
            }
        }

        for x in 0..width {
            for y in 1..(height - 1) {
                let index = x + y * width;
                let val = f64::from(image[(x, (y + 1))].to_luma().data[0]) -
                    f64::from(image[(x, (y - 1))].to_luma().data[0]);
                gradient[index as usize].y = val;
            }
        }

        ImageGradient {
            values: gradient,
            width,
            height,
        }
    }
}
