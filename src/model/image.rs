use std::io::Error;

use image::{io::Reader as ImageReader, Pixel as RgbPixel};

use super::pixel::Pixel;

pub struct Image {
    height: u32,
    width: u32,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn load(path: &str) -> Result<Self, Error> {
        let image = ImageReader::open(path)?;
        let image = image.decode().unwrap();

        let pixels = image.as_rgb8().unwrap()
            .pixels()
            .map(|p| p.channels())
            .map(|p| Pixel::new(p[0], p[1], p[2]))
            .collect::<Vec<Pixel>>();


        Ok(Self {
            height: image.height(),
            width: image.width(),
            pixels,
        })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}
