use crate::models::pixel::Pixel;

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

    pub fn as_raw_bytes(&self) -> Vec<u8> {
        self.pixels.iter().fold(
            Vec::with_capacity(self.pixels().len() * 3),
            |mut vec, pixel| {
                vec.extend_from_slice(&pixel.as_slice());

                vec
            },
        )
    }
}
