#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Self {
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

    pub fn set_grayscale(&mut self, grayscale: u8) {
        self.r(grayscale);
        self.g(grayscale);
        self.b(grayscale);
    }
}
