use crate::models::{Image, Pixel};

pub trait FitnessFunction {
    fn calculate_fitness_rgb(&self, first_image: &Image, second_image: &Image) -> usize;

    fn calculate_fitness_grayscale(&self, first_image: &Image, second_image: &Image) -> usize;
}

pub(crate) fn fold_images<F>(
    first_image: &crate::models::Image,
    second_image: &crate::models::Image,
    fold_pixels: F,
) -> usize
where
    F: FnMut(usize, (&Pixel, &Pixel)) -> usize,
{
    assert_eq!(
        first_image.pixels().len(),
        second_image.pixels().len(),
        "Images must be of the same size."
    );

    first_image
        .pixels()
        .iter()
        .zip(second_image.pixels().iter())
        .fold(0usize, fold_pixels)
}
