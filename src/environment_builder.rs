use anyhow::Result;

use crate::{
    crossover::{CrossoverFunction, EqualHalfsCrossover},
    environment::Environment,
    fitness::{FitnessFunction, SquareDistance},
    models::Image,
    mutators::{Mutator, RectangleMutator},
    DisplayCondition, SaveCondition,
};

pub struct EnvironmentBuilder {
    image: Option<Image>,
    mutator: Box<dyn Mutator + Send>,
    fitness: Box<dyn FitnessFunction + Send>,
    crossover: Box<dyn CrossoverFunction + Send>,
    generation_size: usize,
    display_condition: DisplayCondition,
    output_directory: String,
    save_condition: SaveCondition,
}

impl EnvironmentBuilder {
    pub fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }

    pub fn set_mutator(&mut self, mutator: Box<dyn Mutator + Send>) {
        self.mutator = mutator;
    }

    pub fn set_fitness_function(&mut self, fitness: Box<dyn FitnessFunction + Send>) {
        self.fitness = fitness;
    }

    pub fn set_crossover_function(&mut self, crossover: Box<dyn CrossoverFunction + Send>) {
        self.crossover = crossover;
    }

    pub fn set_generation_size(&mut self, generation_size: usize) {
        self.generation_size = generation_size;
    }

    pub fn set_display_condition(&mut self, display_condition: DisplayCondition) {
        self.display_condition = display_condition;
    }

    pub fn set_output_directory(&mut self, output_directory: &str, save_condition: SaveCondition) {
        match save_condition {
            SaveCondition::Never => self.save_condition = save_condition,
            SaveCondition::Each(0) => panic!("SaveCondition::Each must be greater than zero."),
            SaveCondition::All | SaveCondition::Each(_) => {
                match std::fs::read_dir(output_directory) {
                    Ok(_) => {
                        self.output_directory = output_directory.to_string();
                        self.save_condition = save_condition;
                    }
                    Err(e) => panic!("Error while accessing dir: {}", e),
                }
            }
        }
    }

    pub fn build(self) -> Result<Environment> {
        match self {
            Self { image: None, .. } => Err(anyhow::Error::msg("Image must be set.")),
            Self {
                generation_size: 0, ..
            } => Err(anyhow::Error::msg("Generation size cannot be zero")),
            _ => Ok(Environment::new(
                self.image.unwrap(),
                self.generation_size,
                self.mutator,
                self.fitness,
                self.crossover,
                self.display_condition,
                self.output_directory,
                self.save_condition,
            )),
        }
    }
}

impl Default for EnvironmentBuilder {
    fn default() -> Self {
        Self {
            image: None,
            mutator: Box::new(RectangleMutator::default()),
            fitness: Box::new(SquareDistance::default()),
            crossover: Box::new(EqualHalfsCrossover::default()),
            generation_size: 100,
            display_condition: DisplayCondition::None,
            output_directory: String::new(),
            save_condition: SaveCondition::Never,
        }
    }
}
