use crate::models::Image;

/// This trait defines functionality used to breed two specimens.
///
/// You can use this trait to implement your own crossover function, but take a look at already implemented crossover
/// functions: ([ArithmeticAverageCrossover], [EqualHalfsCrossover], [LeftOrRightCloneCrossover]).
///
/// [ArithmeticAverageCrossover]: crate::crossover::ArithmeticAverageCrossover
/// [EqualHalfsCrossover]: crate::crossover::EqualHalfsCrossover
/// [LeftOrRightCloneCrossover]: crate::crossover::LeftOrRightCloneCrossover
pub trait CrossoverFunction {
    /// This method create a new image which is a product of breeding two images.
    ///
    /// The product does not necessarily has to be a product of crossing two images. Some implementation may ignore
    /// either image or both of them.
    fn crossover(&mut self, first_image: &Image, second_image: &Image) -> Image;
}
