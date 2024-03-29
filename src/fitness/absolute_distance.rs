use crate::models::Pixel;

use super::{fitness_function::fold_images, FitnessFunction};

/// This fitness function calculates the fitness of specimens by calculating the absolute distances between colors of
/// all respective pixels.
#[derive(Debug, Default)]
pub struct AbsoluteDistance;

impl FitnessFunction for AbsoluteDistance {
    fn calculate_fitness_rgb(
        &self,
        first_image: &crate::models::Image,
        second_image: &crate::models::Image,
    ) -> usize {
        let fold_rgb_pixels = |mut sum: usize, (p1, p2): (&Pixel, &Pixel)| {
            let diff_r = isize::from(p1.get_r()) - isize::from(p2.get_r());
            let diff_g = isize::from(p1.get_g()) - isize::from(p2.get_g());
            let diff_b = isize::from(p1.get_b()) - isize::from(p2.get_b());

            sum += diff_r.unsigned_abs();
            sum += diff_g.unsigned_abs();
            sum += diff_b.unsigned_abs();

            sum
        };

        fold_images(first_image, second_image, fold_rgb_pixels)
    }

    fn calculate_fitness_grayscale(
        &self,
        first_image: &crate::models::Image,
        second_image: &crate::models::Image,
    ) -> usize {
        let fold_grayscale_pixels = |mut sum: usize, (p1, p2): (&Pixel, &Pixel)| {
            let diff_red = isize::from(p1.get_r()) - isize::from(p2.get_r());
            let diff_grayscale = diff_red * 3;

            sum += diff_grayscale.unsigned_abs();

            sum
        };

        fold_images(first_image, second_image, fold_grayscale_pixels)
    }
}
