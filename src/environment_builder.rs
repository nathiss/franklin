use std::{fs, path::Path};

use anyhow::{Error, Result};
use rayon::ThreadPoolBuilder;

use crate::{
    crossover::{CrossoverFunction, EqualHalfsCrossover},
    environment::Environment,
    fitness::{FitnessFunction, SquareDistance},
    models::Image,
    mutators::{Mutator, RectangleMutator},
    ColorMode, DisplayCondition, SaveCondition,
};

pub struct EnvironmentBuilder {
    image: Option<Image>,
    color_mode: ColorMode,
    mutator: Box<dyn Mutator + Send + Sync>,
    fitness: Box<dyn FitnessFunction + Send + Sync>,
    crossover: Box<dyn CrossoverFunction + Send>,
    generation_size: usize,
    threads: usize,
    display_condition: DisplayCondition,
    output_directory: String,
    save_condition: SaveCondition,
}

impl EnvironmentBuilder {
    pub fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }

    pub fn set_color_mode(&mut self, color_mode: ColorMode) {
        self.color_mode = color_mode;
    }

    pub fn set_mutator(&mut self, mutator: Box<dyn Mutator + Send + Sync>) {
        self.mutator = mutator;
    }

    pub fn set_fitness_function(&mut self, fitness: Box<dyn FitnessFunction + Send + Sync>) {
        self.fitness = fitness;
    }

    pub fn set_crossover_function(&mut self, crossover: Box<dyn CrossoverFunction + Send>) {
        self.crossover = crossover;
    }

    pub fn set_generation_size(&mut self, generation_size: usize) {
        self.generation_size = generation_size;
    }

    pub fn set_threads(&mut self, threads: usize) {
        self.threads = threads;
    }

    pub fn set_display_condition(&mut self, display_condition: DisplayCondition) {
        self.display_condition = display_condition;
    }

    pub fn set_output_directory(
        &mut self,
        output_directory: &str,
        save_condition: SaveCondition,
    ) -> Result<()> {
        match save_condition {
            SaveCondition::Never => {
                self.save_condition = save_condition;
                Ok(())
            }
            SaveCondition::Each(0) => {
                Err(Error::msg("SaveCondition::Each must be greater than zero."))
            }
            SaveCondition::All | SaveCondition::Each(_) => {
                let abs_path = fs::canonicalize(Path::new(output_directory))?;
                let attr = fs::metadata(abs_path)?;

                if attr.is_dir() {
                    self.output_directory = output_directory.to_string();
                    self.save_condition = save_condition;
                    Ok(())
                } else {
                    Err(Error::msg("The path does not point to a directory"))
                }
            }
        }
    }

    pub fn build(self) -> Result<Environment> {
        match self {
            Self { image: None, .. } => Err(Error::msg("Image must be set.")),
            Self {
                generation_size: 0, ..
            } => Err(Error::msg("Generation size cannot be zero")),
            _ => {
                ThreadPoolBuilder::new()
                    .num_threads(self.threads)
                    .build_global()?;

                let should_save_specimen: Box<dyn Fn(u32) -> bool + Send> =
                    match self.save_condition {
                        SaveCondition::All => Box::new(|_| true),
                        SaveCondition::Each(per) => {
                            Box::new(move |gen_number: u32| gen_number % per == 0)
                        }
                        SaveCondition::Never => Box::new(|_| false),
                    };

                Ok(Environment::new(
                    self.image.unwrap(),
                    self.color_mode,
                    self.generation_size,
                    self.mutator,
                    self.fitness,
                    self.crossover,
                    self.display_condition,
                    &self.output_directory,
                    should_save_specimen,
                ))
            }
        }
    }
}

impl Default for EnvironmentBuilder {
    #[must_use]
    fn default() -> Self {
        Self {
            image: None,
            color_mode: ColorMode::Rgb,
            mutator: Box::new(RectangleMutator::default()),
            fitness: Box::new(SquareDistance::default()),
            crossover: Box::new(EqualHalfsCrossover::default()),
            generation_size: 100,
            threads: 1,
            display_condition: DisplayCondition::None,
            output_directory: String::new(),
            save_condition: SaveCondition::Never,
        }
    }
}
