use crate::models::Image;

pub trait Mutator {
    fn mutate_rgb(&mut self, image: &mut Image);

    fn mutate_grayscale(&mut self, image: &mut Image);
}
