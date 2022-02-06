use super::CrossoverFunction;

#[derive(Default)]
pub struct RandomCrossover;

impl CrossoverFunction for RandomCrossover {
    fn crossover(
        &self,
        _first_image: &crate::models::Image,
        _second_image: &crate::models::Image,
    ) -> crate::models::Image {
        todo!()
    }
}
