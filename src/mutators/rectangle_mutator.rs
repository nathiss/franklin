use crate::{models::Image, util::Random};

use super::Mutator;

struct RandomRectangle {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[derive(Default)]
pub struct RectangleMutator {
    random: Random,
}

impl RectangleMutator {
    fn get_random_rectangle(&mut self, image: &Image) -> RandomRectangle {
        let image_width = image.width();
        let image_height = image.height();

        let x = self.random.get_random(0usize, image_width - 1);
        let y = self.random.get_random(0usize, image_height - 1);

        let width = self.random.get_random(0usize, image_width - x) + 1;
        let height = self.random.get_random(0usize, image_height - y) + 1;

        RandomRectangle {
            x,
            y,
            width,
            height,
        }
    }
}

impl Mutator for RectangleMutator {
    fn mutate_rgb(&mut self, image: &mut Image) {
        let rect = self.get_random_rectangle(image);

        let image_width = image.width();

        let r = self.random.get_random(0u8, 255);
        let g = self.random.get_random(0u8, 255);
        let b = self.random.get_random(0u8, 255);

        for i in rect.x..(rect.width + rect.x) {
            for j in rect.y..(rect.height + rect.y) {
                image[j * image_width + i].r(r);
                image[j * image_width + i].g(g);
                image[j * image_width + i].b(b);
            }
        }
    }

    fn mutate_grayscale(&mut self, image: &mut Image) {
        let rect = self.get_random_rectangle(image);

        let image_width = image.width();

        let grayscale = self.random.get_random(0u8, 255);

        for i in rect.x..(rect.width + rect.x) {
            for j in rect.y..(rect.height + rect.y) {
                image[j * image_width + i].set_grayscale(grayscale);
            }
        }
    }
}
