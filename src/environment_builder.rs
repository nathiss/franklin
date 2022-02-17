use std::{fs, path::Path};

use anyhow::{Error, Result};
use rayon::ThreadPoolBuilder;

use crate::{
    crossover::{CrossoverFunction, EqualHalfsCrossover},
    environment::Environment,
    fitness::{FitnessFunction, SquareDistance},
    job_context::JobContext,
    models::Image,
    mutators::{Mutator, RectangleMutator},
    ColorMode, DisplayCondition, ImageWriter, SaveCondition,
};

/// This builder provides an interface to set up the environment for generating images.
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
    filename_prefix: String,
    save_condition: SaveCondition,
}

impl EnvironmentBuilder {
    /// Sets the image.
    ///
    /// Image is always required. [EnvironmentBuilder#build()] will return an error if the image has not been set.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use franklin::EnvironmentBuilder;
    /// use franklin::ImageReader;
    ///
    /// let mut environment_builder = EnvironmentBuilder::default();
    ///
    /// let image = ImageReader::load("path/to/image.png").unwrap();
    /// environment_builder.set_image(image);
    /// ```
    pub fn set_image(&mut self, image: Image) {
        self.image = Some(image);
    }

    /// Sets the color mode.
    ///
    /// This property is optional. The default value is: Rgb.
    ///
    /// # Examples
    ///
    /// ```
    /// use franklin::{ColorMode, EnvironmentBuilder};
    ///
    /// let mut environment_builder = EnvironmentBuilder::default();
    ///
    /// environment_builder.set_color_mode(ColorMode::Rgb);
    /// ```
    pub fn set_color_mode(&mut self, color_mode: ColorMode) {
        self.color_mode = color_mode;
    }

    /// Sets the mutator.
    ///
    /// This property is optional. The default value is: Rectangle.
    ///
    /// # Examples
    ///
    /// ```
    /// use franklin::EnvironmentBuilder;
    /// use franklin::mutators::TriangleMutator;
    ///
    /// let mut environment_builder = EnvironmentBuilder::default();
    ///
    /// environment_builder.set_mutator(Box::new(TriangleMutator::default()));
    /// ```
    pub fn set_mutator(&mut self, mutator: Box<dyn Mutator + Send + Sync>) {
        self.mutator = mutator;
    }

    /// Sets the fitness function.
    ///
    /// This property is optional. The default value is: SquareDistance.
    ///
    /// # Examples
    ///
    /// ```
    /// use franklin::EnvironmentBuilder;
    /// use franklin::fitness::SquareDistance;
    ///
    /// let mut environment_builder = EnvironmentBuilder::default();
    ///
    /// environment_builder.set_fitness_function(Box::new(SquareDistance::default()));
    /// ```
    pub fn set_fitness_function(&mut self, fitness: Box<dyn FitnessFunction + Send + Sync>) {
        self.fitness = fitness;
    }

    /// Sets the crossover function.
    ///
    /// This property is optional. The default value is: LeftOfRightClone.
    ///
    /// # Examples
    ///
    /// ```
    /// use franklin::EnvironmentBuilder;
    /// use franklin::crossover::ArithmeticAverageCrossover;
    ///
    /// let mut environment_builder = EnvironmentBuilder::default();
    ///
    /// environment_builder.set_crossover_function(Box::new(ArithmeticAverageCrossover::default()));
    /// ```
    pub fn set_crossover_function(&mut self, crossover: Box<dyn CrossoverFunction + Send>) {
        self.crossover = crossover;
    }

    /// Sets the generation size.
    ///
    /// This property is optional. The default value is: 100.
    ///
    /// # Examples
    ///
    /// ```
    /// use franklin::EnvironmentBuilder;
    ///
    /// let mut environment_builder = EnvironmentBuilder::default();
    ///
    /// environment_builder.set_generation_size(150);
    /// ```
    pub fn set_generation_size(&mut self, generation_size: usize) {
        self.generation_size = generation_size;
    }

    /// Sets the number of worker threads.
    ///
    /// This property is optional. The default value is: 1.
    ///
    /// # Examples
    ///
    /// ```
    /// use franklin::EnvironmentBuilder;
    ///
    /// let mut environment_builder = EnvironmentBuilder::default();
    ///
    /// environment_builder.set_threads(5);
    /// ```
    pub fn set_threads(&mut self, threads: usize) {
        self.threads = threads;
    }

    /// Sets the display condition.
    ///
    /// This property is optional. The default value is: [DisplayCondition::None].
    ///
    /// # Examples
    ///
    /// ```
    /// use franklin::{DisplayCondition, EnvironmentBuilder};
    ///
    /// let mut environment_builder = EnvironmentBuilder::default();
    ///
    /// environment_builder.set_display_condition(DisplayCondition::None);
    /// ```
    pub fn set_display_condition(&mut self, display_condition: DisplayCondition) {
        self.display_condition = display_condition;
    }

    /// Sets the output directory and the save condition.
    ///
    /// This method returns an error if `output_directory` does not exist or it's not a directory.
    ///
    /// Usage of this method is optional. The default values are:
    /// * for `output_directory` it's "" (empty path),
    /// * for `save_condition` it's [SaveCondition::Never].
    ///
    /// # Examples
    ///
    /// ```
    /// use franklin::EnvironmentBuilder;
    /// use franklin::SaveCondition;
    ///
    /// let mut environment_builder = EnvironmentBuilder::default();
    ///
    /// environment_builder.set_output_directory("PATH", SaveCondition::All);
    /// ```
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

    /// Sets the filename prefix.
    ///
    /// This property is optional. The default value is: "output_".
    ///
    /// # Examples
    ///
    /// ```
    /// use franklin::EnvironmentBuilder;
    ///
    /// let mut environment_builder = EnvironmentBuilder::default();
    ///
    /// environment_builder.set_filename_prefix("result_");
    /// ```
    pub fn set_filename_prefix(&mut self, filename_prefix: &str) {
        self.filename_prefix = filename_prefix.to_owned();
    }

    /// This method build the environment and returns it.
    ///
    /// Returns an error if the builder has been ill-formed.
    ///
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use franklin::EnvironmentBuilder;
    ///
    /// let mut environment_builder = EnvironmentBuilder::default();
    ///
    /// // Sets up all required properties.
    ///
    /// let environment = environment_builder.build().unwrap();
    /// ```
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

                // Safety: it's safe to unwrap the image because None-case is captured in the first arm.
                let job_context = JobContext::new(
                    self.image.unwrap(),
                    self.mutator,
                    self.fitness,
                    self.color_mode,
                );

                let image_writer =
                    ImageWriter::new(self.output_directory, self.filename_prefix.to_owned());

                Ok(Environment::new(
                    job_context,
                    self.generation_size,
                    self.crossover,
                    self.display_condition,
                    image_writer,
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
            filename_prefix: String::new(),
            save_condition: SaveCondition::Never,
        }
    }
}
