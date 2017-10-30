use cost::ImageCost;

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

pub fn compute_path_grid(costs: &ImageCost) -> PathGrid {
    let mut grid = Vec::with_capacity(costs.num_pixels());
    grid.resize(costs.num_pixels(), PathElement::default());
    let (width, height) = (costs.width() as usize, costs.height() as usize);

    // Start by copying the pixel costs to the first row separately.
    for x in 0..width {
        let index = x as usize;
        grid[index] = PathElement::new(costs.data()[index], 0);
    }

    for y in 1..(height as usize) {
        for x in 1..(width as usize - 1) {
            let index = x + y * width;
            let above_left_index = (x - 1) + (y - 1) * width;
            let pixel_value = costs.data()[index];

            let left = grid[above_left_index].cost();
            let center = grid[above_left_index + 1].cost();
            let right = grid[above_left_index + 2].cost();

            let (direction, prev_value) = if left < center {
                if left < right {
                    (LEFT, left)
                } else {
                    (RIGHT, right)
                }
            } else if center < right {
                (CENTER, center)
            } else {
                (RIGHT, right)
            };

            grid[index] = PathElement::new(prev_value + pixel_value, direction);
        }
    }

    PathGrid {
        grid,
        width: width as u32,
        height: height as u32,
    }
}
