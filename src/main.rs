use std::ops::{Add, Mul, Sub};

#[derive(Copy, Clone)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

struct Image {
    width: u32,
    height: u32,
    pixels: Vec<Color>,
}

impl Image {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0, 0, 0); (width * height) as usize],
        }
    }
    fn set_color(&mut self, x: u32, y: u32, color: Color) {
        self.pixels[(y * self.width + x) as usize] = color
    }
    fn write_to_ppm(&self, fname: &str) {
        let mut data = String::new();
        data.push_str("P3\n");
        data.push_str(&format!("{} {} 255\n", self.width, self.height));
        for Color { r, g, b } in self.pixels.iter() {
            data.push_str(&format!("{r} {g} {b}\n"));
        }
        std::fs::write(fname, &data).expect("Unable to write to file")
    }
}

#[derive(Copy, Clone)]
struct Complex {
    real: f64,
    imaginary: f64,
}

impl Complex {
    fn new(real: f64, imaginary: f64) -> Self {
        Self { real, imaginary }
    }
}

impl Add for Complex {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real + rhs.real,
            imaginary: self.imaginary + rhs.imaginary,
        }
    }
}

impl Sub for Complex {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real - rhs.real,
            imaginary: self.imaginary - rhs.imaginary,
        }
    }
}

impl Mul for Complex {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            real: self.real * rhs.real - self.imaginary * rhs.imaginary,
            imaginary: self.real * rhs.imaginary + self.imaginary * rhs.real,
        }
    }
}

static COLOR_PALETTE1: [Color; 4] = [
    Color {
        r: 96,
        g: 76,
        b: 195,
    },
    Color {
        r: 143,
        g: 209,
        b: 79,
    },
    Color {
        r: 245,
        g: 245,
        b: 245,
    },
    Color {
        r: 255,
        g: 102,
        b: 0,
    },
];

fn get_color(n: usize, z: Complex) -> Color {
    let v = (n as f64 + 1.
        - (z.real * z.real + z.imaginary * z.imaginary)
            .sqrt()
            .log10()
            .log2())
    .ln_1p() * 3.;
    let col1 = COLOR_PALETTE1[v.floor() as usize % 4];
    let col2 = COLOR_PALETTE1[v.ceil() as usize % 4];
    Color::new(
        (col1.r as f64 + ((v - v.floor()) * (col2.r as f64 - col1.r as f64))) as u8,
        (col1.g as f64 + ((v - v.floor()) * (col2.g as f64 - col1.g as f64))) as u8,
        (col1.b as f64 + ((v - v.floor()) * (col2.b as f64 - col1.b as f64))) as u8,
    )
}

fn mandelbrot(
    img: &mut Image,
    scale: f64,
    off_x: f64,
    off_y: f64,
    max_iter: usize,
    escape_value: f64,
) {
    for i in 0..img.pixels.len() {
        let x = i as u32 % img.width;
        let y = i as u32 / img.width;
        let c = Complex::new(
            -0.5 + ((3. * x as f64 / img.width as f64) - 1.5) * scale + off_x,
            ((2.5 * y as f64 / img.height as f64) - 1.25) * scale + off_y,
        );
        let mut z = Complex::new(0., 0.);
        img.set_color(x, y, Color::new(0, 0, 0));
        for n in 0..max_iter {
            if z.real * z.real + z.imaginary * z.imaginary > escape_value {
                img.set_color(x, y, get_color(n, z));
                break;
            }
            z = z * z + c;
        }
    }
}

fn main() {
    let mut img = Image::new(1920, 1080);
    mandelbrot(&mut img, 1., 0., 0., 100, 256.);
    img.write_to_ppm("test.ppm");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_case_ppm() {
        let mut img = Image::new(1920, 1080);
        mandelbrot(&mut img, 1., 0., 0., 100, 256.);
        img.write_to_ppm("basic.ppm");
    }
    #[test]
    fn small1_case_ppm() {
        let mut img = Image::new(2560, 1440);
        mandelbrot(&mut img, 0.00015, -1.0752, 0.006, 100, 256.);
        img.write_to_ppm("small1.ppm");
    }
    #[test]
    fn small2_case_ppm() {
        let mut img = Image::new(800, 450);
        mandelbrot(&mut img, 0.02, -0.06, -0.56, 100, 256.);
        img.write_to_ppm("small2.ppm");
    }
    #[test]
    fn small3_case_ppm() {
        let mut img = Image::new(2560, 1440);
        mandelbrot(&mut img, 0.002, -0.06, -0.555, 100, 256.);
        img.write_to_ppm("small3.ppm");
    }
}
