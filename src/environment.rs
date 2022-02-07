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

    generation: Vec<(Image, usize)>,
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
            self.generation
                .push((Image::blank(height, width, &pixel), usize::MAX));
        }
    }

    fn run_single_generation(&mut self) {
        self.generation
            .iter_mut()
            .skip(1) // We skip the first element to make sure we always make progress or stay with the same image
            .for_each(|mut entry| {
                // Mutate
                self.mutator.mutate(&mut entry.0);

                // Calculate fitness
                entry.1 = self.fitness.calculate_fitness(&self.image, &entry.0);
            });

        self.generation.sort_by(|a, b| a.1.cmp(&b.1));

        // Dump worst, save best
        self.generation.truncate(2);
        for i in 2..self.generation_size {
            let entry = self.generation[i % 2].clone();
            self.generation.push(entry);
        }

        // Crossover

        self.current_generation_number += 1;
        println!(
            "Current generation: {} ({})",
            self.current_generation_number, self.generation[0].1
        );
    }

    pub fn run(mut self) {
        self.prepare_first_generation();

        match &self.display_condition {
            DisplayCondition::All | DisplayCondition::Every(_) => self.run_with_window(),
            DisplayCondition::None => self.run_without_window(),
        }
    }

    fn run_with_window(mut self) {
        let dimensions = (self.image.height(), self.image.width());
        Window::run_with_context(dimensions, move |mut window| -> Result<()> {
            while !window.should_exit() {
                self.run_single_generation();

                let should_display_window = match self.display_condition {
                    DisplayCondition::All => true,
                    DisplayCondition::Every(per) => self.current_generation_number % per == 0,
                    DisplayCondition::None => false,
                };

                if should_display_window {
                    window.show_image("Lorem ipsum", &self.generation[0].0)?;
                }
            }

            Ok(())
        });
    }

    fn run_without_window(mut self) {
        loop {
            self.run_single_generation();
        }
    }
}
