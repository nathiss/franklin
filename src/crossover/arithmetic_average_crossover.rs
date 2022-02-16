use crate::models::{Image, Pixel};

use super::CrossoverFunction;

#[derive(Debug, Default)]
pub struct ArithmeticAverageCrossover;

impl CrossoverFunction for ArithmeticAverageCrossover {
    fn crossover(&mut self, first_image: &Image, second_image: &Image) -> Image {
        let pixels = first_image
            .pixels()
            .iter()
            .zip(second_image.pixels().iter())
            .map(|(p1, p2)| {
                let red_average = ((u16::from(p1.get_r()) + u16::from(p2.get_r())) / 2) as u8;
                let green_average = ((u16::from(p1.get_g()) + u16::from(p2.get_g())) / 2) as u8;
                let blue_average = ((u16::from(p1.get_b()) + u16::from(p2.get_b())) / 2) as u8;

                Pixel::new(red_average, green_average, blue_average)
            })
            .collect::<Vec<Pixel>>();

        Image::new(first_image.height(), first_image.width(), pixels)
    }
}
