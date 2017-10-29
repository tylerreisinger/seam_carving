use std::path;
use std::cmp::Ordering;
use cgmath::{Vector2, InnerSpace};

extern crate image;
extern crate cgmath;

pub mod gradient;
pub mod seams;

fn load_image(path: &path::Path) -> image::ImageResult<image::RgbaImage> {
    image::open(path).map(|img| img.to_rgba())
}

fn main() {
    let path = path::Path::new("test_images/Broadway_tower_edit.jpg");
    let img = load_image(&path);

    if let Err(e) = img {
        println!("Target file '{}' could not be loaded:", path.display());
        println!("\t{}", e);
    } else {
        process_image(img.unwrap());
    }
}

fn process_image(image: image::RgbaImage) {
    println!("{}x{}", image.width(), image.height());

    let gradient = gradient::ImageGradient::from_luma_image(&image);
    let path_grid = seams::compute_path_grid(&gradient);

    save_gradient_image(&gradient, "output/GradientOut.png");
    save_path_grid_image(&path_grid, "output/PathGridOut.png");
}

fn save_gradient_image<P>(gradient: &gradient::ImageGradient, path: P)
where
    P: AsRef<path::Path>,
{
    let img_buf = gradient
        .values
        .iter()
        .map(|x| ((x / 255.0).magnitude() / f64::sqrt(2.0) * 255.0) as u8)
        .collect::<Vec<_>>();

    let img = image::GrayImage::from_raw(gradient.width, gradient.height, img_buf).unwrap();
    img.save(path).unwrap();
}

fn save_path_grid_image<P>(grid: &seams::PathGrid, path: P)
where
    P: AsRef<path::Path>,
{
    let (width, height) = (grid.width() as usize, grid.height() as usize);
    let bottom_row_idx = (height - 1) * width;

    let max = grid.data()[..]
        .iter()
        .max_by(|x, y| if let Some(c) = x.cost().partial_cmp(&y.cost()) {
            c
        } else {
            Ordering::Less
        })
        .unwrap();

    let img_buf = grid.data()
        .iter()
        .map(|x| ((x.cost() / max.cost()) * 255.0) as u8)
        .collect::<Vec<_>>();

    let img = image::GrayImage::from_raw(grid.width(), grid.height(), img_buf).unwrap();
    img.save(path).unwrap();

    println!("Max cost: {}", max.cost());
}
