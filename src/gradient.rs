use image::{self, Pixel};
use cgmath::Vector2;
use std::f64;

#[derive(Debug, Clone)]
pub struct ImageGradient {
    pub values: Vec<Vector2<f64>>,
    pub width: u32,
    pub height: u32,
}

impl ImageGradient {
    pub fn num_pixels(&self) -> usize {
        (self.width as usize) * (self.height as usize)
    }
    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
    pub fn data(&self) -> &[Vector2<f64>] {
        &self.values
    }

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
