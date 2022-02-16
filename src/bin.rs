use anyhow::{Error, Result};

use franklin::{
    crossover::{
        ArithmeticAverageCrossover, CrossoverFunction, EqualHalfsCrossover,
        LeftOrRightCloneCrossover,
    },
    fitness::{AbsoluteDistance, FitnessFunction, SquareDistance},
    mutators::{Mutator, RectangleMutator, TriangleMutator},
    ArgParser, ColorMode, DisplayCondition, EnvironmentBuilder, ImageReader, SaveCondition,
};

fn get_color_mode_from_name(name: &str) -> Result<ColorMode> {
    match name {
        "Rgb" => Ok(ColorMode::Rgb),
        "Grayscale" => Ok(ColorMode::Grayscale),
        _ => Err(Error::msg("Unknown mode.")),
    }
}

fn get_mutator_from_name(name: &str) -> Result<Box<dyn Mutator + Send + Sync + 'static>> {
    match name {
        "Rectangle" => Ok(Box::new(RectangleMutator::default())),
        "Triangle" => Ok(Box::new(TriangleMutator::default())),
        _ => Err(Error::msg("Unknown mutator.")),
    }
}

fn get_fitness_from_name(name: &str) -> Result<Box<dyn FitnessFunction + Send + Sync + 'static>> {
    match name {
        "SquareDistance" => Ok(Box::new(SquareDistance::default())),
        "AbsoluteDistance" => Ok(Box::new(AbsoluteDistance::default())),
        _ => Err(Error::msg("Unknown fitness function.")),
    }
}

fn get_crossover_from_name(name: &str) -> Result<Box<dyn CrossoverFunction + Send + 'static>> {
    match name {
        "LeftOrRight" => Ok(Box::new(LeftOrRightCloneCrossover::default())),
        "EqualHalfs" => Ok(Box::new(EqualHalfsCrossover::default())),
        "ArithmeticAverage" => Ok(Box::new(ArithmeticAverageCrossover::default())),
        _ => Err(Error::msg("Unknown crossover function.")),
    }
}

fn main() -> Result<()> {
    let args = ArgParser::default();

    // Safety: it's safe to unwrap because this argument is required, i.e. it cannot be empty or None.
    let image = ImageReader::load(args.get_value("image").unwrap())?;

    let mut environment_builder = EnvironmentBuilder::default();
    environment_builder.set_image(image);

    // Safety: it's safe to unwrap because this argument has a default value, i.e. it cannot be empty or None.
    environment_builder.set_color_mode(get_color_mode_from_name(
        args.get_value("color_mode").unwrap(),
    )?);

    // Safety: it's safe to unwrap because this argument has a default value, i.e. it cannot be empty or None.
    environment_builder.set_mutator(get_mutator_from_name(args.get_value("mutator").unwrap())?);

    // Safety: it's safe to unwrap because this argument has a default value, i.e. it cannot be empty or None.
    environment_builder
        .set_fitness_function(get_fitness_from_name(args.get_value("fitness").unwrap())?);

    // Safety: it's safe to unwrap because this argument has a default value, i.e. it cannot be empty or None.
    environment_builder.set_crossover_function(get_crossover_from_name(
        args.get_value("crossover").unwrap(),
    )?);

    // Safety: it's safe to unwrap because this argument has a validator which checks if the value can be parsed to a
    // usize.
    let generation_size: usize = args.get_value_t("generation_size")?;
    environment_builder.set_generation_size(generation_size);

    // Safety: it's safe to unwrap because this argument has a validator which checks if the value can be parsed to a
    // usize.
    let threads: usize = args.get_value_t("threads")?;
    environment_builder.set_threads(threads);

    if args.is_present("display") {
        if args.is_present("display_all") {
            environment_builder.set_display_condition(DisplayCondition::All);
        } else if let Ok(per) = args.get_value_t::<u32>("display_every") {
            // If "display_every" has been passed we should always end up here. This argument has a validator which
            // checks if the value can be parsed to a u32.
            environment_builder.set_display_condition(DisplayCondition::Every(per));
        }
    }

    if let Some(output_directory) = args.get_value("output_directory") {
        // Safety: it's safe to unwrap because this argument has a default value, i.e. it cannot be empty or None.
        let filename_prefix = args.get_value("filename_prefix").unwrap();
        environment_builder.set_filename_prefix(filename_prefix);

        let save_condition = if args.is_present("save_all") {
            SaveCondition::All
        } else if let Ok(per) = args.get_value_t::<u32>("save_every") {
            // If "display_every" has been passed we should always end up here. This argument has a validator which
            // checks if the value can be parsed to a u32.
            SaveCondition::Each(per)
        } else {
            SaveCondition::Never
        };

        environment_builder.set_output_directory(output_directory, save_condition)?;
    }

    let environment = environment_builder.build()?;
    environment.run()?;

    Ok(())
}
