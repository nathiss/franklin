use crate::models::Image;

/// This trait defines functionality used to mutate a single specimen.
///
/// You can use this trait to implement your own mutator, but take a look at already implemented mutators:
/// ([RectangleMutator], [TriangleMutator]).
///
/// [RectangleMutator]: crate::mutators::RectangleMutator
/// [TriangleMutator]: crate::mutators::TriangleMutator
pub trait Mutator {
    /// This method mutates the given image.
    ///
    /// This method performs the mutation with respect to all three color channels.
    fn mutate_rgb(&self, image: &mut Image);

    /// This method mutates the given image.
    ///
    /// This method performs the mutation with respect to only one color channel.
    fn mutate_grayscale(&self, image: &mut Image);
}
