use anyhow::{Error, Result};

use franklin::{
    crossover::{CrossoverFunction, EqualHalfsCrossover, LeftOrRightCloneCrossover},
    fitness::{FitnessFunction, SquareDistance},
    mutators::{Mutator, RectangleMutator},
    ArgParser, DisplayCondition, EnvironmentBuilder, ImageReader, SaveCondition,
};

fn get_mutator_from_name(name: &str) -> Result<Box<dyn Mutator + Send + 'static>> {
    match name {
        "Rectangle" => Ok(Box::new(RectangleMutator::default())),
        _ => Err(Error::msg("Unknown mutator.")),
    }
}

fn get_fitness_from_name(name: &str) -> Result<Box<dyn FitnessFunction + Send + 'static>> {
    match name {
        "SquareDistance" => Ok(Box::new(SquareDistance::default())),
        _ => Err(Error::msg("Unknown fitness function.")),
    }
}

fn get_crossover_from_name(name: &str) -> Result<Box<dyn CrossoverFunction + Send + 'static>> {
    match name {
        "LeftOrRight" => Ok(Box::new(LeftOrRightCloneCrossover::default())),
        "EqualHalfs" => Ok(Box::new(EqualHalfsCrossover::default())),
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
