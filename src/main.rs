extern crate image;
mod complex;

use std::fs::File;
use std::path::Path;
use complex::Complex;

fn in_set(num: Complex) -> bool {
    let mut tmp = num.clone();

    for x in 0..110 {
        // Z' = Z * Z + C
        tmp = tmp.multiply(tmp).add(num);
    }

    tmp.real * tmp.i < 0.4f64
}

fn main() {
    let filename = "mandelbrot.png";
    let magnification = 200f64;
    let panx = 2f64;
    let pany = 1.5f64;
    let width = 600;
    let height = 600;

    let mut img: image::RgbImage = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let now = Complex {
            real: (x as f64 / magnification - panx),
            i: (y as f64 / magnification - pany)
        };
        if in_set(now) {
            *pixel = image::Rgb([255u8, 0u8, 0u8])
        } else {
            *pixel = image::Rgb([0u8, 0u8, 0u8])
        }
    }

    let ref mut file = File::create(&Path::new(filename)).unwrap();
    image::ImageRgb8(img).save(file, image::PNG).unwrap();
}
