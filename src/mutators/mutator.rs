use crate::models::Image;

pub trait Mutator {
    fn mutate_rgb(&self, image: &mut Image);

    fn mutate_grayscale(&self, image: &mut Image);
}
