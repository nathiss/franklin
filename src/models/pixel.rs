#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub const fn white() -> Self {
        Pixel::new(255, 255, 255)
    }

    pub const fn grayscale(grayscale: u8) -> Self {
        Pixel::new(grayscale, grayscale, grayscale)
    }

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel { r, g, b }
    }

    pub fn r(&mut self, r: u8) {
        self.r = r;
    }

    pub fn g(&mut self, g: u8) {
        self.g = g;
    }

    pub fn b(&mut self, b: u8) {
        self.b = b;
    }

    pub fn get_r(&self) -> u8 {
        self.r
    }

    pub fn get_g(&self) -> u8 {
        self.g
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }

    pub fn set_grayscale(&mut self, grayscale: u8) {
        self.r(grayscale);
        self.g(grayscale);
        self.b(grayscale);
    }

    pub fn as_slice(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}
