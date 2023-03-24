use crate::{models::Image, util::Random};

use super::Mutator;

struct RandomRectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

/// This mutator mutates the given specimen by generating a random rectangle with random background color.
#[derive(Debug, Default)]
pub struct RectangleMutator;

impl RectangleMutator {
    fn get_random_rectangle(&self, random: &mut Random, image: &Image) -> RandomRectangle {
        let image_width = image.width();
        let image_height = image.height();

        let x = random.get_random(0usize, image_width);
        let y = random.get_random(0usize, image_height);

        let width = random.get_random(0usize, image_width - x) + 1;
        let height = random.get_random(0usize, image_height - y) + 1;

        RandomRectangle {
            x,
            y,
            width,
            height,
        }
    }
}

impl Mutator for RectangleMutator {
    fn mutate_rgb(&self, image: &mut Image) {
        let mut random = Random::default();

        let rect = self.get_random_rectangle(&mut random, image);

        let image_width = image.width();

        let r = random.get_random(0u8, 255);
        let g = random.get_random(0u8, 255);
        let b = random.get_random(0u8, 255);

        for i in rect.x..(rect.width + rect.x) {
            for j in rect.y..(rect.height + rect.y) {
                let pixel = &mut image[j * image_width + i];
                pixel.r(r);
                pixel.g(g);
                pixel.b(b);
            }
        }
    }

    fn mutate_grayscale(&self, image: &mut Image) {
        let mut random = Random::default();

        let rect = self.get_random_rectangle(&mut random, image);

        let image_width = image.width();

        let grayscale = random.get_random(0u8, 255);

        for i in rect.x..(rect.width + rect.x) {
            for j in rect.y..(rect.height + rect.y) {
                image[j * image_width + i].set_grayscale(grayscale);
            }
        }
    }
}
