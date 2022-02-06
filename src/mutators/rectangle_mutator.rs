use rand::{
    distributions::{uniform::SampleUniform, Uniform},
    prelude::{Distribution, StdRng},
    SeedableRng,
};

use crate::models::Image;

use super::Mutator;

pub struct RectangleMutator {
    rng: StdRng,
}

impl RectangleMutator {
    fn get_random<T>(&mut self, min: T, max: T) -> T
    where
        T: SampleUniform,
    {
        Uniform::new(min, max).sample(&mut self.rng)
    }
}

impl Mutator for RectangleMutator {
    fn mutate(&mut self, image: &mut Image) {
        let x = self.get_random(0usize, image.width() - 1);
        let y = self.get_random(0usize, image.height() - 1);

        let width = self.get_random(0usize, image.width() - x) + 1;
        let height = self.get_random(0usize, image.height() - y) + 1;

        let image_width = image.width();

        let r = self.get_random(0u8, 255);
        let g = self.get_random(0u8, 255);
        let b = self.get_random(0u8, 255);

        for i in x..width + x {
            for j in y..height + y {
                image[j * image_width + i].r(r);
                image[j * image_width + i].g(g);
                image[j * image_width + i].b(b);
            }
        }
    }
}

impl Default for RectangleMutator {
    fn default() -> Self {
        Self {
            rng: rand::rngs::StdRng::from_entropy(),
        }
    }
}
