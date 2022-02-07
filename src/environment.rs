use anyhow::Result;
use rand::prelude::SliceRandom;

use crate::{
    crossover::CrossoverFunction,
    display::Window,
    fitness::FitnessFunction,
    models::{Image, Pixel},
    mutators::Mutator,
    util::Random,
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

    random: Random,
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
            random: Random::default(),
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

    fn run_single_generation(&mut self) -> Result<()> {
        self.generation
            .iter_mut()
            // We skip the first element to make sure we always make progress or stay with the same image
            .skip(1)
            .for_each(|mut entry| {
                // Mutate
                self.mutator.mutate(&mut entry.0);

                // Calculate fitness
                entry.1 = self.fitness.calculate_fitness(&self.image, &entry.0);
            });

        // Sort
        self.generation.sort_by(|a, b| a.1.cmp(&b.1));

        // Dump worst
        let best_size = self.get_best_size();
        self.generation.truncate(best_size);
        // for i in best_size..self.generation_size {
        //     let entry = self.generation[i % 2].clone();
        //     self.generation.push(entry);
        // }

        // Crossover
        for _ in 0..self.generation_size - best_size {
            let parents = self
                .generation
                .choose_multiple(self.random.get_rng(), 2)
                .map(|entry| &entry.0)
                .collect::<Vec<&Image>>();

            let new_image = self.crossover.crossover(parents[0], parents[1]);
            self.generation.push((new_image, usize::MAX));
        }

        self.current_generation_number += 1;
        println!(
            "Current generation: {} ({})",
            self.current_generation_number, self.generation[0].1
        );

        Ok(())
    }

    pub fn run(mut self) -> Result<()> {
        self.prepare_first_generation();

        match &self.display_condition {
            DisplayCondition::All | DisplayCondition::Every(_) => self.run_with_window(),
            DisplayCondition::None => self.run_without_window(),
        }
    }

    fn run_with_window(mut self) -> Result<()> {
        let dimensions = (self.image.height(), self.image.width());
        Window::run_with_context(dimensions, move |mut window| -> Result<()> {
            while !window.should_exit() {
                self.run_single_generation()?;

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

        Ok(())
    }

    fn run_without_window(mut self) -> Result<()> {
        loop {
            self.run_single_generation()?;
        }
    }

    fn get_best_size(&self) -> usize {
        let size = self.generation_size / 50;

        if size == 0 {
            1
        } else {
            size
        }
    }
}
