use crate::models::{Image, Pixel};

use super::CrossoverFunction;

/// This crossover function breeds specimens by calculating arithmetic average of each pixel from both parents and
/// producing new pixels for the new specimen.
#[derive(Debug, Default)]
pub struct ArithmeticAverageCrossover;

impl CrossoverFunction for ArithmeticAverageCrossover {
    fn crossover(&mut self, first_image: &Image, second_image: &Image) -> Image {
        let pixels = first_image
            .pixels()
            .iter()
            .zip(second_image.pixels().iter())
            .map(|(p1, p2)| {
                let red_average = p1.get_r() / 2 + p2.get_r() / 2;
                let green_average = p1.get_g() / 2 + p2.get_g() / 2;
                let blue_average = p1.get_b() / 2 + p2.get_b() / 2;

                Pixel::new(red_average, green_average, blue_average)
            })
            .collect::<Vec<Pixel>>();

        Image::new(first_image.height(), first_image.width(), pixels)
    }
}
