use std::path;

extern crate image;

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
}
