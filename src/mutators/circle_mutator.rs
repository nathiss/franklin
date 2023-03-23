use num_integer::Roots;

use crate::{models::Image, util::Random};

use super::Mutator;

struct RandomCircle {
    x: i64,
    y: i64,
    r: i64,
}

/// This mutator mutates the given specimen by generating a random filled circle with random background color.
#[derive(Debug, Default)]
pub struct CircleMutator;

impl CircleMutator {
    fn get_random_circle(&self, random: &mut Random, image: &Image) -> RandomCircle {
        let image_width = image.width() as i64;
        let image_height = image.height() as i64;

        let x = random.get_random(1, image_width);
        let y = random.get_random(1, image_height);

        let n = *[x, y, image_width - x, image_height - y]
            .iter()
            .min()
            .unwrap();

        let r = random.get_random(1, n + 1);

        RandomCircle { x, y, r }
    }
}

impl Mutator for CircleMutator {
    fn mutate_rgb(&self, image: &mut crate::models::Image) {
        let mut random = Random::default();

        let circle = self.get_random_circle(&mut random, image);

        let r_channel = random.get_random(0u8, 255);
        let g_channel = random.get_random(0u8, 255);
        let b_channel = random.get_random(0u8, 255);

        for i in -circle.r..circle.r {
            let height = (circle.r * circle.r - i * i).sqrt();

            for j in -height..height {
                let idx = (j + circle.y) * image.width() as i64 + (i + circle.x);
                let pixel = &mut image[idx as usize];
                pixel.r(r_channel);
                pixel.g(g_channel);
                pixel.b(b_channel);
            }
        }
    }

    fn mutate_grayscale(&self, image: &mut crate::models::Image) {
        let mut random = Random::default();

        let circle = self.get_random_circle(&mut random, image);

        let grayscale = random.get_random(0u8, 255);

        for i in -circle.r..circle.r {
            let height = (circle.r * circle.r - i * i).sqrt();

            for j in -height..height {
                let idx = (j + circle.y) * image.width() as i64 + (i + circle.x);
                let pixel = &mut image[idx as usize];
                pixel.set_grayscale(grayscale);
            }
        }
    }
}
