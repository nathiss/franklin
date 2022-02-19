#![warn(missing_docs)]

//! A utility for generating images through evolution.
//!
//! This crate provides utilities, as well as CLI tool called `franklin-cli` for generating evolutionary art.
//! Generation of evolutionary art can be separated into three steps:
//! - mutation,
//! - calculating fitness and selection,
//! - breeding the specimens.
//!
//! # Usage
//!
//! This example shows the minimal code for setting up the generation. All settings are set to default. Only one
//! property it required: source image.
//!
//! ```
//! use franklin::{EnvironmentBuilder, ImageReader};
//!
//! fn run_simulation() {
//!     let mut environment_builder = EnvironmentBuilder::default();
//!
//!     let image = ImageReader::load("./example.png").unwrap();
//!     environment_builder.set_image(image);
//!
//!     let environment = environment_builder.build().unwrap();
//!     environment.run().unwrap();
//! }
//! ```

mod arg_parser;
pub use arg_parser::ArgParser;

mod color_mode;
pub use color_mode::ColorMode;

mod display;
pub use display::DisplayCondition;

mod environment;

mod environment_builder;
pub use environment_builder::EnvironmentBuilder;

/// This module contains utilities used to breed specimens.
pub mod crossover;

/// This module contains utilities used to calculate the fitness of specimens.
pub mod fitness;

pub(crate) mod job_context;

mod models;

/// This module contains utilities used to mutate specimens.
pub mod mutators;

mod util;

mod fs;
pub use fs::*;
