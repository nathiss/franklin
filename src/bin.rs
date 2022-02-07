use anyhow::Result;

use franklin::{
    crossover::RandomCrossover, fitness::SquareDistance, mutators::RectangleMutator,
    DisplayCondition, EnvironmentBuilder, ImageReader, SaveCondition,
};

fn main() -> Result<()> {
    let image = ImageReader::load("C:\\Users\\kamil\\Downloads\\Lenna.png")?;
    println!("Dimensions: h: {}, w: {}", image.height(), image.width());
    println!("Pixels #: {}", image.pixels().len());

    let mut environment_builder = EnvironmentBuilder::default();
    environment_builder.set_image(image);
    environment_builder.set_mutator(Box::new(RectangleMutator::default()));
    environment_builder.set_fitness_function(Box::new(SquareDistance::default()));
    environment_builder.set_crossover_function(Box::new(RandomCrossover::default()));
    environment_builder.set_display_condition(DisplayCondition::Every(50));
    environment_builder.set_output_directory("", SaveCondition::Never);
    environment_builder.set_generation_size(100);

    let environment = environment_builder.build()?;

    environment.run();

    Ok(())
}
