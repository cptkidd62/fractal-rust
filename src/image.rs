#[derive(Copy, Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }
}

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub pixels: Vec<Color>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            pixels: vec![Color::new(0, 0, 0); (width * height) as usize],
        }
    }
    pub fn set_color(&mut self, x: u32, y: u32, color: Color) {
        self.pixels[(y * self.width + x) as usize] = color
    }
    pub fn write_to_ppm(&self, fname: &str) {
        let mut data = String::new();
        data.push_str("P3\n");
        data.push_str(&format!("{} {} 255\n", self.width, self.height));
        for Color { r, g, b } in self.pixels.iter() {
            data.push_str(&format!("{r} {g} {b}\n"));
        }
        std::fs::write(fname, &data).expect("Unable to write to file")
    }
}
