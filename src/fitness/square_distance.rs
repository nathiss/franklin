use super::FitnessFunction;

#[derive(Default)]
pub struct SquareDistance;

impl FitnessFunction for SquareDistance {
    fn calculate_fitness(
        &self,
        first_image: &crate::models::Image,
        second_image: &crate::models::Image,
    ) -> usize {
        assert_eq!(first_image.pixels().len(), second_image.pixels().len(), "Images must be of the same size.");

        first_image
            .pixels()
            .iter()
            .zip(second_image.pixels().iter())
            .fold(0usize, |mut sum, (p1, p2)| {
                let diff_r = isize::from(p1.get_r()) - isize::from(p2.get_r());
                let diff_g = isize::from(p1.get_g()) - isize::from(p2.get_g());
                let diff_b = isize::from(p1.get_b()) - isize::from(p2.get_b());

                sum += diff_r.pow(2) as usize;
                sum += diff_g.pow(2) as usize;
                sum += diff_b.pow(2) as usize;

                sum
            })
    }
}
