use std::ops::{Add, Mul, Sub};

struct Image {
    width: u32,
    height: u32,
    pixels: Vec<(u8, u8, u8)>,
}

impl Image {
    fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![(0, 0, 0); (width * height) as usize],
        }
    }
    fn set_color(&mut self, x: u32, y: u32, color: (u8, u8, u8)) {
        self.pixels[(y * self.width + x) as usize] = color
    }
    fn write_to_ppm(&self) {
        let mut data = String::new();
        data.push_str("P3\n");
        data.push_str(&format!("{} {} 255\n", self.width, self.height));
        for (r, g, b) in self.pixels.iter() {
            data.push_str(&format!("{r} {g} {b}\n"));
        }
        std::fs::write("test.ppm", &data).expect("Unable to write to file")
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

fn mandelbrot(img: &mut Image, max_iter: usize) {
    for i in 0..img.pixels.len() {
        let x = i as u32 % img.width;
        let y = i as u32 / img.width;
        let c = Complex::new(
            (2.5 * x as f64 / img.width as f64) - 2.,
            (2.24 * y as f64 / img.height as f64) - 1.12,
        );
        let mut z = Complex::new(0., 0.);
        img.set_color(x, y, (0, 0, 0));
        for n in 0..max_iter {
            if z.real * z.real + z.imaginary * z.imaginary > 4. {
                img.set_color(x, y, (255, 255, 255));
                break;
            }
            z = z * z + c;
        }
    }
}

fn main() {
    let mut img = Image::new(1920, 1080);
    mandelbrot(&mut img, 100);
    img.write_to_ppm();
}
