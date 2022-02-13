use crate::models::Image;

pub trait FitnessFunction {
    fn calculate_fitness_rgb(&self, first_image: &Image, second_image: &Image) -> usize;

    fn calculate_fitness_grayscale(&self, first_image: &Image, second_image: &Image) -> usize;
}
