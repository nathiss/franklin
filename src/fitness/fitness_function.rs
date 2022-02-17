use crate::models::{Image, Pixel};

/// This trait defines functionality used to calculate fitness of specimens.
///
/// You can use this trait to implement your own fitness function, but take a look at already implemented fitness
/// functions: ([AbsoluteDistance], [SquareDistance]).
///
/// [AbsoluteDistance]: crate::fitness::AbsoluteDistance
/// [SquareDistance]: crate::fitness::SquareDistance
pub trait FitnessFunction {
    /// This method calculates the fitness of `second_image` relative to `first_image`.
    ///
    /// In other words, it returns a value describing difference between those two images. The higher the value, the
    /// more those images are different from each other. This method calculates the fitness with respect to all three
    /// color channels.
    fn calculate_fitness_rgb(&self, first_image: &Image, second_image: &Image) -> usize;

    /// This method calculates the fitness of `second_image` relative to `first_image`.
    ///
    /// In other words, it returns a value describing difference between those two images. The higher the value, the
    /// more those images are different from each other. This method calculates the fitness with respect to only one
    /// color channel.
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
