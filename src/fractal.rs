use super::complex;
use super::image;

static COLOR_PALETTE1: [image::Color; 4] = [
    image::Color {
        r: 96,
        g: 76,
        b: 195,
    },
    image::Color {
        r: 143,
        g: 209,
        b: 79,
    },
    image::Color {
        r: 245,
        g: 245,
        b: 245,
    },
    image::Color {
        r: 255,
        g: 102,
        b: 0,
    },
];

pub fn get_color(n: usize, z: complex::Complex) -> image::Color {
    let v = (n as f64 + 1. - z.abs().log10().log2()).ln_1p() * 3.;
    let col1 = COLOR_PALETTE1[v.floor() as usize % 4];
    let col2 = COLOR_PALETTE1[v.ceil() as usize % 4];
    image::Color::new(
        (col1.r as f64 + ((v - v.floor()) * (col2.r as f64 - col1.r as f64))) as u8,
        (col1.g as f64 + ((v - v.floor()) * (col2.g as f64 - col1.g as f64))) as u8,
        (col1.b as f64 + ((v - v.floor()) * (col2.b as f64 - col1.b as f64))) as u8,
    )
}

pub fn mandelbrot(
    img: &mut image::Image,
    scale: f64,
    off_x: f64,
    off_y: f64,
    max_iter: usize,
    escape_value: f64,
) {
    for i in 0..img.pixels.len() {
        let x = i as u32 % img.width;
        let y = i as u32 / img.width;
        let c = complex::Complex::new(
            -0.5 + ((3. * x as f64 / img.width as f64) - 1.5) * scale + off_x,
            ((2.5 * y as f64 / img.height as f64) - 1.25) * scale + off_y,
        );
        let mut z = complex::Complex::new(0., 0.);
        img.set_color(x, y, image::Color::new(0, 0, 0));
        for n in 0..max_iter {
            if z.abs2() > escape_value {
                img.set_color(x, y, get_color(n, z));
                break;
            }
            z = z * z + c;
        }
    }
}
