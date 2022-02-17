use crate::models::Image;

use super::CrossoverFunction;

/// This crossover function breeds specimens by copying either `first_image` or `second_image`.
///
/// Selection of the source switches with each call of [LeftOrRightCloneCrossover#crossover] method.
#[derive(Debug, Default)]
pub struct LeftOrRightCloneCrossover {
    counter: usize,
}

impl CrossoverFunction for LeftOrRightCloneCrossover {
    fn crossover(&mut self, first_image: &Image, second_image: &Image) -> Image {
        self.counter += 1;

        if self.counter % 2 == 0 {
            first_image.clone()
        } else {
            second_image.clone()
        }
    }
}
