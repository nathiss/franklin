use crate::models::Image;

pub trait FitnessFunction {
    fn calculate_fitness(&self, first_image: &Image, second_image: &Image) -> usize;
}
