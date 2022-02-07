use std::ops::{Index, IndexMut};

use crate::models::pixel::Pixel;

#[derive(Clone)]
pub struct Image {
    height: usize,
    width: usize,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(height: usize, width: usize, pixels: Vec<Pixel>) -> Self {
        Self {
            height,
            width,
            pixels,
        }
    }

    pub fn from_iter<'a, I>(height: usize, width: usize, iter: I) -> Self
    where
        I: Iterator<Item = &'a Pixel>,
    {
        let size = height * width;
        let pixels = iter.take(size).cloned().collect::<Vec<Pixel>>();
        assert_eq!(size, pixels.len(), "Iterator yielded not enough pixels.");

        Self::new(height, width, pixels)
    }

    pub fn blank(height: usize, width: usize, pixel: &Pixel) -> Self {
        let size = height * width;
        let mut pixels = Vec::with_capacity(size as usize);

        for _ in 0..size {
            pixels.push(pixel.clone());
        }

        Self {
            height,
            width,
            pixels,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
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

impl Index<usize> for Image {
    type Output = Pixel;

    fn index(&self, index: usize) -> &Self::Output {
        &self.pixels[index]
    }
}

impl IndexMut<usize> for Image {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.pixels[index]
    }
}
