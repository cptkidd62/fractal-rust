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

fn main() {
    let mut img = Image::new(512, 256);
    for i in 0..img.height {
        if i < 1000000 {
            img.set_color(i, i, (255, 0, 0));
        }
    }
    img.write_to_ppm();
}
