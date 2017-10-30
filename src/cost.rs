use std::f64;
use cgmath::InnerSpace;

use gradient;

#[derive(Clone, Debug)]
pub struct ImageCost {
    pixels: Vec<f64>,
    width: u32,
    height: u32,
}

impl ImageCost {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn data(&self) -> &[f64] {
        &self.pixels
    }

    pub fn num_pixels(&self) -> usize {
        (self.width as usize) * (self.height as usize)
    }
}

pub fn compute_gradient_magnitude_cost(gradient: &gradient::ImageGradient) -> ImageCost {
    let grid = gradient
        .data()
        .iter()
        .map(|x| x.magnitude() / (255.0 * f64::sqrt(2.0)))
        .collect::<Vec<_>>();

    ImageCost {
        pixels: grid,
        width: gradient.width,
        height: gradient.height,
    }
}
