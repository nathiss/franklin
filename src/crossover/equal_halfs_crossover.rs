use crate::models::{Image, Pixel};

use super::CrossoverFunction;

#[derive(Default)]
pub struct EqualHalfsCrossover;

impl CrossoverFunction for EqualHalfsCrossover {
    fn crossover(&mut self, first_image: &Image, second_image: &Image) -> Image {
        let size = first_image.pixels().len();

        let number_of_pixels_from_first_image = if size % 2 == 0 {
            size / 2
        } else {
            size / 2 + 1
        };

        let number_of_pixels_from_second_image = size / 2;

        let pixels = first_image
            .pixels()
            .iter()
            .take(number_of_pixels_from_first_image)
            .chain(
                second_image
                    .pixels()
                    .iter()
                    .take(number_of_pixels_from_second_image),
            )
            .cloned()
            .collect::<Vec<Pixel>>();

        Image::new(first_image.height(), first_image.width(), pixels)
    }
}
