use crate::models::Image;

use super::Mutator;

#[derive(Default)]
pub struct RectangleMutator {}

impl Mutator for RectangleMutator {
    fn mutate(&self, _image: &mut Image) {}
}
