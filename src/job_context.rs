use std::sync::Arc;

use crate::{fitness::FitnessFunction, models::Image, mutators::Mutator, ColorMode};

#[derive(Clone)]
pub(crate) struct JobContext {
    image: Arc<Image>,
    mutator: Arc<Box<dyn Mutator + Send + Sync>>,
    fitness: Arc<Box<dyn FitnessFunction + Send + Sync>>,
    color_mode: ColorMode,
}

impl JobContext {
    pub fn new(
        image: Image,
        mutator: Box<dyn Mutator + Send + Sync>,
        fitness: Box<dyn FitnessFunction + Send + Sync>,
        color_mode: ColorMode,
    ) -> Self {
        Self {
            image: Arc::new(image),
            mutator: Arc::new(mutator),
            fitness: Arc::new(fitness),
            color_mode,
        }
    }

    pub fn get_image(&self) -> &Arc<Image> {
        &self.image
    }

    pub fn get_mutator(&self) -> &Arc<Box<dyn Mutator + Send + Sync>> {
        &self.mutator
    }

    pub fn get_fitness(&self) -> &Arc<Box<dyn FitnessFunction + Send + Sync>> {
        &self.fitness
    }

    pub fn get_color_mode(&self) -> ColorMode {
        self.color_mode
    }
}
