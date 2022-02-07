use crate::models::Image;

pub trait CrossoverFunction {
    fn crossover(&mut self, first_image: &Image, second_image: &Image) -> Image;
}
