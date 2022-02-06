use super::FitnessFunction;

#[derive(Default)]
pub struct SquareDistance;

impl FitnessFunction for SquareDistance {
    fn calculate_fitness(
        &self,
        _first_image: &crate::models::Image,
        _second_image: &crate::models::Image,
    ) -> usize {
        todo!()
    }
}
