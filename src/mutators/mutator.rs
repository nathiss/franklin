use crate::models::Image;

pub trait Mutator {
    fn mutate(&self, image: &mut Image);
}
