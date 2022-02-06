use crate::models::Image;

pub trait Mutator {
    fn mutate(&mut self, image: &mut Image);
}
