mod complex;
mod fractal;
mod image;

fn main() {
    let mut img = image::Image::new(1920, 1080);
    fractal::mandelbrot(&mut img, 1., 0., 0., 100, 256.);
    img.write_to_ppm("test.ppm");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn basic_case_ppm() {
        let mut img = image::Image::new(1920, 1080);
        fractal::mandelbrot(&mut img, 1., 0., 0., 100, 256.);
        img.write_to_ppm("basic.ppm");
    }
    #[test]
    fn small1_case_ppm() {
        let mut img = image::Image::new(2560, 1440);
        fractal::mandelbrot(&mut img, 0.00015, -1.0752, 0.006, 100, 256.);
        img.write_to_ppm("small1.ppm");
    }
    #[test]
    fn small2_case_ppm() {
        let mut img = image::Image::new(800, 450);
        fractal::mandelbrot(&mut img, 0.02, -0.06, -0.56, 100, 256.);
        img.write_to_ppm("small2.ppm");
    }
    #[test]
    fn small3_case_ppm() {
        let mut img = image::Image::new(2560, 1440);
        fractal::mandelbrot(&mut img, 0.002, -0.06, -0.555, 100, 256.);
        img.write_to_ppm("small3.ppm");
    }
}
