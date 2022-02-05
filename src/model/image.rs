use anyhow::{Ok, Result};
use image::{io::Reader as ImageReader, Pixel as RgbPixel};

use super::pixel::Pixel;

pub struct Image {
    height: u32,
    width: u32,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn load(path: &str) -> Result<Self> {
        let image = ImageReader::open(path)?;
        let image = image.decode()?;

        let image_buffer = match image.as_rgb8() {
            Some(image_buffer) => Ok(image_buffer),
            None => Err(anyhow::Error::msg(
                "Cannot convert the image to RGB8 representation.",
            )),
        }?;

        let pixels = image_buffer
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

    pub fn pixels(&self) -> &Vec<Pixel> {
        &self.pixels
    }
}
