use crate::{models::Image, util::Random};

use super::Mutator;

#[derive(Default)]
pub struct RectangleMutator {
    random: Random,
}

impl Mutator for RectangleMutator {
    fn mutate(&mut self, image: &mut Image) {
        let x = self.random.get_random(0usize, image.width() - 1);
        let y = self.random.get_random(0usize, image.height() - 1);

        let width = self.random.get_random(0usize, image.width() - x) + 1;
        let height = self.random.get_random(0usize, image.height() - y) + 1;

        let image_width = image.width();

        let r = self.random.get_random(0u8, 255);
        let g = self.random.get_random(0u8, 255);
        let b = self.random.get_random(0u8, 255);

        for i in x..width + x {
            for j in y..height + y {
                image[j * image_width + i].r(r);
                image[j * image_width + i].g(g);
                image[j * image_width + i].b(b);
            }
        }
    }
}
