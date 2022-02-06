use anyhow::Result;

use crate::{
    crossover::CrossoverFunction,
    display::Window,
    fitness::FitnessFunction,
    models::{Image, Pixel},
    mutators::Mutator,
    DisplayCondition, SaveCondition,
};

pub struct Environment {
    image: Image,
    generation_size: usize,
    mutator: Box<dyn Mutator + Send>,
    fitness: Box<dyn FitnessFunction + Send>,
    crossover: Box<dyn CrossoverFunction + Send>,
    display_condition: DisplayCondition,
    output_directory: String,
    save_condition: SaveCondition,

    generation: Vec<Image>,
    current_generation_number: u32,
}

impl Environment {
    pub fn new(
        image: Image,
        generation_size: usize,
        mutator: Box<dyn Mutator + Send>,
        fitness: Box<dyn FitnessFunction + Send>,
        crossover: Box<dyn CrossoverFunction + Send>,
        display_condition: DisplayCondition,
        output_directory: String,
        save_condition: SaveCondition,
    ) -> Self {
        Self {
            image,
            generation_size,
            mutator,
            fitness,
            crossover,
            display_condition,
            output_directory,
            save_condition,
            generation: Vec::with_capacity(generation_size),
            current_generation_number: 0,
        }
    }

    fn prepare_first_generation(&mut self) {
        let height = self.image.height();
        let width = self.image.width();
        let pixel = Pixel::white();

        for _ in 0..self.generation_size {
            self.generation.push(Image::blank(height, width, &pixel));
        }
    }

    fn run_single_generation(&mut self) {
        // Mutate
        self.generation
            .iter_mut()
            .for_each(|mut specimen| self.mutator.mutate(&mut specimen));

        // Calculate fitness

        // Crossover

        // Dump worst, save best

        self.current_generation_number += 1;
        println!("Current generation: {}", self.current_generation_number);
    }

    pub fn run(mut self) {
        self.prepare_first_generation();

        let dimensions = (self.image.height(), self.image.width());
        Window::run_with_context(dimensions, move |mut window| -> Result<()> {
            while !window.should_exit() {
                self.run_single_generation();

                window.show_image("Lorem ipsum", &self.generation[0])?;
            }

            Ok(())
        });
    }
}
