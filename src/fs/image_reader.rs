use anyhow::{Ok, Result};
use image::{io::Reader, Pixel as RgbPixel};

use crate::models::{Image, Pixel};

/// This struct can be used to load an image from the filesystem.
#[derive(Debug)]
pub struct ImageReader;

impl ImageReader {
    /// This method load the image pointed by the given path.
    pub fn load(path: &str) -> Result<Image> {
        let image = Reader::open(path)?;
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

        Ok(Image::new(
            image.height() as usize,
            image.width() as usize,
            pixels,
        ))
    }
}
