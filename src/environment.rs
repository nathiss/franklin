use anyhow::Result;
use rand::prelude::SliceRandom;

use crate::{
    crossover::CrossoverFunction,
    display::Window,
    fitness::FitnessFunction,
    models::{Image, Pixel},
    mutators::Mutator,
    util::Random,
    ColorMode, DisplayCondition, ImageWriter,
};

fn get_best_size(generation_size: usize) -> usize {
    // This should always be true. arg_parser::validate_generation_size ensures valid generation size.
    assert!(generation_size > 2, "Generation size must be grater than 2.");

    if generation_size >= 100 {
        generation_size / 50
    } else {
        2
    }
}

pub struct Environment {
    image: Image,
    color_mode: ColorMode,
    generation_size: usize,
    mutator: Box<dyn Mutator + Send>,
    fitness: Box<dyn FitnessFunction + Send>,
    crossover: Box<dyn CrossoverFunction + Send>,
    display_condition: DisplayCondition,
    should_save_specimen: Box<dyn Fn(u32) -> bool + Send>,

    generation: Vec<(Image, usize)>,
    best_from_generation_size: usize,
    current_generation_number: u32,

    random: Random,
    image_writer: ImageWriter,
}

impl Environment {
    pub fn new(
        image: Image,
        color_mode: ColorMode,
        generation_size: usize,
        mutator: Box<dyn Mutator + Send>,
        fitness: Box<dyn FitnessFunction + Send>,
        crossover: Box<dyn CrossoverFunction + Send>,
        display_condition: DisplayCondition,
        output_directory: &str,
        should_save_specimen: Box<dyn Fn(u32) -> bool + Send>,
    ) -> Self {
        Self {
            image,
            color_mode,
            generation_size,
            mutator,
            fitness,
            crossover,
            display_condition,
            should_save_specimen,
            generation: Vec::with_capacity(generation_size),
            best_from_generation_size: get_best_size(generation_size),
            current_generation_number: 0,
            random: Random::default(),
            image_writer: ImageWriter::to_dir(output_directory),
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
                // Mutate & calculate fitness
                match self.color_mode {
                    ColorMode::Rgb => {
                        self.mutator.mutate_rgb(&mut entry.0);
                        entry.1 = self.fitness.calculate_fitness_rgb(&self.image, &entry.0);
                    }
                    ColorMode::Grayscale => {
                        self.mutator.mutate_grayscale(&mut entry.0);
                        entry.1 = self
                            .fitness
                            .calculate_fitness_grayscale(&self.image, &entry.0);
                    }
                }
            });

        // Sort
        self.generation.sort_by(|a, b| a.1.cmp(&b.1));

        // Dump worst
        self.generation.truncate(self.best_from_generation_size);

        // Crossover
        for _ in 0..self.generation_size - self.best_from_generation_size {
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
                    // TODO: change this title
                    window.show_image("Lorem ipsum", &self.generation[0].0)?;
                }

                if (self.should_save_specimen)(self.current_generation_number) {
                    self.save_best_specimen()?;
                }
            }

            Ok(())
        });

        Ok(())
    }

    fn run_without_window(mut self) -> Result<()> {
        loop {
            self.run_single_generation()?;

            if (self.should_save_specimen)(self.current_generation_number) {
                self.save_best_specimen()?;
            }
        }
    }

    fn save_best_specimen(&self) -> Result<()> {
        let filename = format!("output_{:0>6}.png", self.current_generation_number);

        self.image_writer.write(&filename, &self.generation[0].0)?;

        Ok(())
    }
}
