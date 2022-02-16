use std::sync::mpsc::channel;

use anyhow::Result;
use rand::prelude::SliceRandom;
use rayon::spawn;

use crate::{
    crossover::CrossoverFunction,
    display::Window,
    job_context::JobContext,
    models::{Image, Pixel},
    util::Random,
    ColorMode, DisplayCondition, ImageWriter,
};

fn get_best_size(generation_size: usize) -> usize {
    // This should always be true. arg_parser::validate_generation_size ensures valid generation size.
    assert!(
        generation_size > 2,
        "Generation size must be grater than 2."
    );

    if generation_size >= 100 {
        generation_size / 50
    } else {
        2
    }
}

fn get_first_generation(
    vec_len: usize,
    image_height: usize,
    image_width: usize,
) -> Vec<(Image, usize)> {
    let mut vec = Vec::with_capacity(vec_len);

    let pixel = Pixel::white();

    vec.resize_with(vec_len, || {
        (Image::blank(image_height, image_width, &pixel), usize::MAX)
    });

    vec
}

pub struct Environment {
    job_context: JobContext,
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
    #[must_use]
    pub(crate) fn new(
        job_context: JobContext,
        generation_size: usize,
        crossover: Box<dyn CrossoverFunction + Send>,
        display_condition: DisplayCondition,
        image_writer: ImageWriter,
        should_save_specimen: Box<dyn Fn(u32) -> bool + Send>,
    ) -> Self {
        let generation = get_first_generation(
            generation_size,
            job_context.get_image().height(),
            job_context.get_image().width(),
        );

        Self {
            job_context,
            crossover,
            display_condition,
            should_save_specimen,
            generation,
            best_from_generation_size: get_best_size(generation_size),
            current_generation_number: 0,
            random: Random::default(),
            image_writer,
        }
    }

    #[must_use]
    fn mutate_generation(mut self) -> Self {
        let mut new_generation = Vec::with_capacity(self.generation.len());
        let mut old_generation = self.generation.into_iter();

        // Safety: it's safe to unwrap here because generation always has fixed number of specimens and it cannot be
        // less than 3.
        new_generation.push(old_generation.next().unwrap());

        let (tx, rx) = channel();

        old_generation.for_each(|mut entry| {
            let tx = tx.clone();
            let context = self.job_context.clone();

            spawn(move || {
                match context.get_color_mode() {
                    ColorMode::Rgb => {
                        context.get_mutator().mutate_rgb(&mut entry.0);
                        entry.1 = context
                            .get_fitness()
                            .calculate_fitness_rgb(&*context.get_image(), &entry.0);
                    }
                    ColorMode::Grayscale => {
                        context.get_mutator().mutate_grayscale(&mut entry.0);
                        entry.1 = context
                            .get_fitness()
                            .calculate_fitness_grayscale(&*context.get_image(), &entry.0);
                    }
                }

                // Safety: it's ok to unwrap here because the flow of the program guarantees that the Receiver<T> (rx)
                // will outlive all Senders<T> (tx).
                // See: https://doc.rust-lang.org/std/sync/mpsc/struct.Sender.html#method.send
                tx.send(entry).unwrap();
            });
        });

        drop(tx);
        new_generation.extend(rx.iter());
        self.generation = new_generation;

        self
    }

    fn run_single_generation(mut self) -> Result<Self> {
        self = self.mutate_generation();

        // Sort
        self.generation.sort_by(|a, b| a.1.cmp(&b.1));

        // Dump worst
        let generation_size = self.generation.len();
        self.generation.truncate(self.best_from_generation_size);

        // Crossover
        for _ in 0..generation_size - self.best_from_generation_size {
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

        Ok(self)
    }

    pub fn run(self) -> Result<()> {
        match &self.display_condition {
            DisplayCondition::All | DisplayCondition::Every(_) => self.run_with_window(),
            DisplayCondition::None => self.run_without_window(),
        }
    }

    fn run_with_window(mut self) -> Result<()> {
        let dimensions = (
            self.job_context.get_image().height(),
            self.job_context.get_image().width(),
        );
        Window::run_with_context(dimensions, move |mut window| -> Result<()> {
            while !window.should_exit() {
                self = self.run_single_generation()?;

                let should_display_window = match self.display_condition {
                    DisplayCondition::All => true,
                    DisplayCondition::Every(per) => self.current_generation_number % per == 0,
                    DisplayCondition::None => false,
                };

                if should_display_window {
                    window.show_image(
                        &format!("Generation #{}", self.current_generation_number),
                        &self.generation[0].0,
                    )?;
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
            self = self.run_single_generation()?;

            if (self.should_save_specimen)(self.current_generation_number) {
                self.save_best_specimen()?;
            }
        }
    }

    fn save_best_specimen(&self) -> Result<()> {
        self.image_writer
            .write(self.current_generation_number, &self.generation[0].0)?;

        Ok(())
    }
}
