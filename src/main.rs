use std::path;
use cgmath::{Vector2, InnerSpace};

extern crate image;
extern crate cgmath;

pub mod gradient;

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

    let img_buf = gradient
        .values
        .iter()
        .map(|x| ((x / 255.0).magnitude() / f64::sqrt(2.0) * 255.0) as u8)
        .collect::<Vec<_>>();

    let img = image::GrayImage::from_raw(gradient.width, gradient.height, img_buf).unwrap();
    img.save("TestOut.png").unwrap();
}
