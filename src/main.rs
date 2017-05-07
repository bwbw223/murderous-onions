extern crate image;
extern crate hsl;
mod complex;

use hsl::HSL;
use complex::Complex;

use std::fs::File;
use std::path::Path;
fn hsl_tuple_as_array(slice: (u8, u8, u8)) -> [u8; 3] {
    [slice.0, slice.1, slice.2]
}
fn in_set(num: Complex, max: u16) -> f64 {
    let mut tmp = num.clone();
    let mut out = 0f64;
    for x in 0..max {
        // Z' = Z * Z + C
        tmp = tmp.multiply(tmp).add(num);
        if 2f64 < tmp.real.abs() && 2f64 < tmp.i.abs() {out = x as f64; break;}
    }

    if out == max as f64 - 1f64 {0f64} else {out / 100f64}
}

fn main() {
    let filename = "mandelbrot.png";
    let magnification = 1280f64;
    let panx = 2f64;
    let pany = 1.5f64;
    let width = 3840;
    let height = 3840;
    let iterations = 100u16;

    let mut img: image::RgbImage = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let now = Complex {
            real: (x as f64 / magnification - panx),
            i: (y as f64 / magnification - pany)
        };
        let inset = in_set(now, iterations);
        if  inset == 0f64 {
            *pixel = image::Rgb([0u8, 0u8, 0u8])
        } else {
            *pixel = image::Rgb(hsl_tuple_as_array(HSL {h: 0f64, s: 1f64, l: inset}.to_rgb()))
        }
    }

    let ref mut file = File::create(&Path::new(filename)).unwrap();
    image::ImageRgb8(img).save(file, image::PNG).unwrap();
}
