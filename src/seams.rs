use cgmath::Vector2;

use gradient;

pub const LEFT: i8 = -1;
pub const CENTER: i8 = 0;
pub const RIGHT: i8 = 1;

#[derive(Debug, Clone, Default)]
pub struct PathElement {
    cumulative_cost: f64,
    direction: i8,
}

#[derive(Debug, Clone)]
pub struct PathGrid {
    grid: Vec<PathElement>,
    width: u32,
    height: u32,
}

impl PathElement {
    pub fn new(cumulative_cost: f64, direction: i8) -> PathElement {
        PathElement {
            cumulative_cost,
            direction,
        }
    }
    pub fn cost(&self) -> f64 {
        self.cumulative_cost
    }
    pub fn direction(&self) -> i8 {
        self.direction
    }
}

impl PathGrid {
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn data(&self) -> &[PathElement] {
        &self.grid
    }
}

pub fn compute_path_grid(grad: &gradient::ImageGradient) -> PathGrid {
    let mut grid = Vec::with_capacity(grad.num_pixels());
    grid.resize(grad.num_pixels(), PathElement::default());
    let (width, height) = (grad.width as usize, grad.height as usize);

    // Start by copying the pixel costs to the first row separately.
    for x in 0..width {
        let index = x as usize;
        grid[index] = PathElement::new(grad.values[index].x.abs(), 0);
    }

    for y in 1..(height as usize) {
        for x in 1..(width as usize - 1) {
            let index = x + y * width;
            let above_left_index = (x - 1) + (y - 1) * width;
            let pixel_value = grad.values[index].x.abs();

            let left = grid[above_left_index].cost();
            let center = grid[above_left_index + 1].cost();
            let right = grid[above_left_index + 2].cost();

            let max = left.min(center.min(right));

            let direction;
            let prev_value;

            if max == left {
                direction = LEFT;
                prev_value = left;
            } else if max == center {
                direction = CENTER;
                prev_value = center;
            } else {
                direction = RIGHT;
                prev_value = right;
            }

            grid[index] = PathElement::new(prev_value + pixel_value, direction);
        }
    }

    PathGrid {
        grid,
        width: width as u32,
        height: height as u32,
    }
}
