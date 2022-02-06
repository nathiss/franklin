use anyhow::Result;

use crate::{
    crossover::{CrossoverFunction, RandomCrossover},
    environment::Environment,
    fitness::{FitnessFunction, SquareDistance},
    models::Image,
    mutators::{Mutator, RectangleMutator},
    DisplayCondition,
};

pub struct EnvironmentBuilder {
    image: Option<Image>,
    mutator: Box<dyn Mutator>,
    fitness: Box<dyn FitnessFunction>,
    crossover: Box<dyn CrossoverFunction>,
    display_condition: DisplayCondition,
}

impl EnvironmentBuilder {
    pub fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }

    pub fn set_mutator(&mut self, mutator: Box<dyn Mutator>) {
        self.mutator = mutator;
    }

    pub fn set_fitness_function(&mut self, fitness: Box<dyn FitnessFunction>) {
        self.fitness = fitness;
    }

    pub fn set_crossover_function(&mut self, crossover: Box<dyn CrossoverFunction>) {
        self.crossover = crossover;
    }

    pub fn set_display_condition(&mut self, display_condition: DisplayCondition) {
        self.display_condition = display_condition;
    }

    pub fn build(self) -> Result<Environment> {
        match self.image {
            Some(image) => Ok(Environment::for_image(image)),
            None => Err(anyhow::Error::msg("Message must be set.")),
        }
    }
}

impl Default for EnvironmentBuilder {
    fn default() -> Self {
        Self {
            image: None,
            mutator: Box::new(RectangleMutator::default()),
            fitness: Box::new(SquareDistance::default()),
            crossover: Box::new(RandomCrossover::default()),
            display_condition: DisplayCondition::None,
        }
    }
}
