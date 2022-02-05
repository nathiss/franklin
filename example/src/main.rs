use franklin::Image;

fn main() {
    let image = Image::load("C:\\Users\\kamil\\Downloads\\Lenna.png").unwrap();
    println!("Dimensions: h: {}, w: {}", image.height(), image.width());
    println!("Pixels #: {}", image.pixels().len());

    // let mut environment_builder = EnvironmentBuilder::default();
    // environment_builder.set_image(image);
    // environment_builder.set_mutator(RectangleMutator::default());
    // environment_builder.set_fitness_mode(SquareDistance::default());
    // environment_builder.set_crossover_mode(RandomCrossover::default());
    // environment_builder.set_color_mode(ColorMode::Grayscale);
    // environment_builder.set_condition(Condition::GenerationNumber(10_000));

    // let environment = environment_builder.build();

    // environment.run();
}
