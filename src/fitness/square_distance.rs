use crate::models::Pixel;

use super::FitnessFunction;

fn fold_images<F>(
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

#[derive(Default)]
pub struct SquareDistance;

impl FitnessFunction for SquareDistance {
    fn calculate_fitness_rgb(
        &self,
        first_image: &crate::models::Image,
        second_image: &crate::models::Image,
    ) -> usize {
        let fold_rgb_pixels = |mut sum: usize, (p1, p2): (&Pixel, &Pixel)| {
            let diff_r = isize::from(p1.get_r()) - isize::from(p2.get_r());
            let diff_g = isize::from(p1.get_g()) - isize::from(p2.get_g());
            let diff_b = isize::from(p1.get_b()) - isize::from(p2.get_b());

            sum += diff_r.pow(2) as usize;
            sum += diff_g.pow(2) as usize;
            sum += diff_b.pow(2) as usize;

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

            sum += diff_grayscale.pow(2) as usize;

            sum
        };

        fold_images(first_image, second_image, fold_grayscale_pixels)
    }
}
