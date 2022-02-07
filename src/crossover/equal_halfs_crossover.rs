use crate::models::Image;

use super::CrossoverFunction;

#[derive(Default)]
pub struct EqualHalfsCrossover;

impl CrossoverFunction for EqualHalfsCrossover {
    fn crossover(&self, first_image: &Image, second_image: &Image) -> Image {
        let size = first_image.pixels().len();
        let iter = first_image
            .pixels()
            .iter()
            .take(size / 2 + 1)
            .chain(second_image.pixels().iter().take(size / 2));

        Image::from_iter(first_image.height(), first_image.width(), iter)
    }
}
