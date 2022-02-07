use crate::models::{Image, Pixel};

use super::CrossoverFunction;

#[derive(Default)]
pub struct EqualHalfsCrossover;

impl CrossoverFunction for EqualHalfsCrossover {
    fn crossover(&mut self, first_image: &Image, second_image: &Image) -> Image {
        let size = first_image.pixels().len();
        let mut pixels = first_image
            .pixels()
            .iter()
            .take(size / 2 + 1)
            .chain(second_image.pixels().iter().take(size / 2))
            .cloned()
            .collect::<Vec<Pixel>>();

        // TODO: should not be nesesery
        pixels.truncate(size);

        Image::new(first_image.height(), first_image.width(), pixels)
    }
}
