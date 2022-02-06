use super::pixel::Pixel;

pub struct Image {
    height: u32,
    width: u32,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(height: u32, width: u32, pixels: Vec<Pixel>) -> Self {
        Self {
            height,
            width,
            pixels,
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixels(&self) -> &Vec<Pixel> {
        &self.pixels
    }
}
