mod complex;
mod fractal;
mod image;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 7 {
        println!("Please provide the following arguments: FILENAME, WIDTH, HEIGHT, SCALE, X_AXIS_OFFSET, Y_AXIS_OFFSET")
    } else {
        let fname = args[1].as_str();
        let width = args[2].parse::<u32>().unwrap();
        let height = args[3].parse::<u32>().unwrap();
        let scale = args[4].parse::<f64>().unwrap();
        let off_x = args[5].parse::<f64>().unwrap();
        let off_y = args[6].parse::<f64>().unwrap();
        let mut img = image::Image::new(width, height);
        fractal::mandelbrot(&mut img, scale, off_x, off_y, 100, 256.);
        img.write_to_ppm(fname);
    }
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
