use std::f64;
use cgmath::InnerSpace;

use gradient;
use grid::Grid;

#[derive(Clone, Debug)]
pub struct ImageCost {
    pixels: Vec<f64>,
    width: u32,
    height: u32,
}

impl Grid for ImageCost {
    type Element = f64;

    fn width(&self) -> u32 {
        self.width
    }
    fn height(&self) -> u32 {
        self.height
    }
    fn data(&self) -> &[f64] {
        &self.pixels
    }
    fn data_mut(&mut self) -> &mut [f64] {
        &mut self.pixels
    }
}

impl ImageCost {
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
        width: gradient.width(),
        height: gradient.height(),
    }
}
