use std::{fmt::Display, str::FromStr};

use anyhow::Result;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches, ValueHint,
};

const DISPLAY_ALL_INFO: &str =
    "Displays best specimen from every generation. This option conflicts with \
display-every. Only one of them can be used at the same time.";

const DISPLAY_EVERY_INFO: &str =
    "Displays best specimen once per N generations. This option conflicts with \
display-all. Only one of them can be used at the same time.";

const OUTPUT_DIRECTORY_INFO: &str = "Path to the output directory in which generated images will be saved. Given path \
must exist and point to a directory. The argument has no effect if neither save-all, nor save-every have been given \
too.";

const SAVE_ALL_INFO: &str =
    "Saves best specimen from every generation. This option conflicts with save-every. Only one of them can be used at \
    the same time. If output-dir has not been specified this argument has no effect.";

const SAVE_EVERY_INFO: &str =
    "Saves best specimen once per N generations. This option conflicts with save-all. Only one of them can be used at \
    the same time. If output-dir has not been specified this argument has no effect.";

fn validate_generation_size(s: &str) -> Result<(), String> {
    match s.parse::<usize>() {
        Ok(size) => {
            if size > 2 {
                Ok(())
            } else {
                Err(String::from("Generation size cannot be smaller than 3."))
            }
        }
        Err(msg) => Err(msg.to_string()),
    }
}

fn validate_every(s: &str) -> Result<(), String> {
    match s.parse::<u32>() {
        Ok(size) => {
            if size > 0 {
                Ok(())
            } else {
                Err(String::from("Generation gap must be a positive integer."))
            }
        }
        Err(msg) => Err(msg.to_string()),
    }
}

#[derive(Debug)]
pub struct ArgParser {
    arg_matches: ArgMatches,
}

impl ArgParser {
    pub fn get_value(&self, key: &str) -> Option<&str> {
        self.arg_matches.value_of(key)
    }

    pub fn get_value_t<T>(&self, key: &str) -> Result<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Display,
    {
        Ok(self.arg_matches.value_of_t::<T>(key)?)
    }

    pub fn is_present(&self, key: &str) -> bool {
        self.arg_matches.is_present(key)
    }
}

impl Default for ArgParser {
    fn default() -> Self {
        let arg_matches = App::new(crate_name!())
            .author(crate_authors!("\n"))
            .about(crate_description!())
            .version(crate_version!())
            .arg(
                Arg::new("image")
                    .short('i')
                    .long("image")
                    .help("Path to the source image")
                    .takes_value(true)
                    .required(true)
                    .forbid_empty_values(true)
                    .value_hint(ValueHint::FilePath)
                    .display_order(10),
            )
            .arg(
                Arg::new("mutator")
                    .short('m')
                    .long("mutator")
                    .help("Mutator used to mutate specimens")
                    .takes_value(true)
                    .possible_values(["Rectangle"])
                    .default_value("Rectangle")
                    .display_order(20),
            )
            .arg(
                Arg::new("fitness")
                    .short('f')
                    .long("fitness")
                    .help("Fitness function used to calculate fitness of specimens")
                    .takes_value(true)
                    .possible_values(["SquareDistance"])
                    .default_value("SquareDistance")
                    .display_order(30),
            )
            .arg(
                Arg::new("crossover")
                    .short('c')
                    .long("crossover")
                    .help("Crossover function used to breed specimens")
                    .takes_value(true)
                    .possible_values(["LeftOrRight", "EqualHalfs"])
                    .default_value("LeftOrRight")
                    .display_order(40),
            )
            .arg(
                Arg::new("generation_size")
                    .short('g')
                    .long("generation")
                    .help("Number of specimens in each generation")
                    .takes_value(true)
                    .default_value("100")
                    .forbid_empty_values(true)
                    .value_name("N")
                    .validator(validate_generation_size)
                    .display_order(50),
            )
            .arg(
                Arg::new("display_all")
                    .long("display-all")
                    .long_help(DISPLAY_ALL_INFO)
                    .takes_value(false)
                    .group("display")
                    .display_order(60),
            )
            .arg(
                Arg::new("display_every")
                    .long("display-every")
                    .long_help(DISPLAY_EVERY_INFO)
                    .takes_value(true)
                    .forbid_empty_values(true)
                    .value_name("N")
                    .group("display")
                    .validator(validate_every)
                    .display_order(70),
            )
            .arg(
                Arg::new("output_directory")
                    .long("output-dir")
                    .long_help(OUTPUT_DIRECTORY_INFO)
                    .takes_value(true)
                    .forbid_empty_values(true)
                    .value_name("DIR")
                    .value_hint(ValueHint::DirPath)
                    .display_order(80),
            )
            .arg(
                Arg::new("save_all")
                    .long("save-all")
                    .long_help(SAVE_ALL_INFO)
                    .takes_value(false)
                    .group("output")
                    .display_order(90),
            )
            .arg(
                Arg::new("save_every")
                    .long("save-every")
                    .long_help(SAVE_EVERY_INFO)
                    .takes_value(true)
                    .forbid_empty_values(true)
                    .value_name("N")
                    .group("output")
                    .validator(validate_every)
                    .display_order(100),
            )
            .get_matches();

        Self { arg_matches }
    }
}
