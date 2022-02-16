use std::{fmt::Display, str::FromStr};

use anyhow::Result;

use clap::{
    crate_authors, crate_description, crate_name, crate_version, App, Arg, ArgMatches, ValueHint,
};

const MODE_INFO: &str =
    "Color mode by which specimens will be mutated. \"rgb\" indicates that each color channel should be mutated \
    separately. \"grayscale\" indicates that all color channels should be mutated together, resulting in a grayscale \
    image.";

const THREADS_INFO: &str =
    "Number of working threads used to mutate specimens and calculate their fitness. This number must be a positive \
    integer.";

const DISPLAY_ALL_INFO: &str =
    "Displays best specimen from every generation. This option conflicts with display-every. Only one of them can be \
    used at the same time.";

const DISPLAY_EVERY_INFO: &str =
    "Displays best specimen once per N generations. This option conflicts with display-all. Only one of them can be \
    used at the same time.";

const OUTPUT_DIRECTORY_INFO: &str =
    "Path to the output directory in which generated images will be saved. Given path must exist and point to a \
    directory. The argument has no effect if neither save-all, nor save-every has been given too.";

const FILENAME_PREFIX_INFO: &str =
    "Filename prefix for output images. The prefix is concatenated with current generation number and PNG extension. \
    The argument has no effect if neither save-all, nor save-every has been given too.";

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

fn validate_threads(s: &str) -> Result<(), String> {
    match s.parse::<usize>() {
        Ok(num) => {
            if num > 0 {
                Ok(())
            } else {
                Err(String::from("TNumber of workers cannot be 0."))
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

fn get_app() -> App<'static> {
    App::new(crate_name!())
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
            Arg::new("color_mode")
                .long("mode")
                .long_help(MODE_INFO)
                .takes_value(true)
                .possible_values(["Rgb", "Grayscale"])
                .default_value("Rgb")
                .display_order(20),
        )
        .arg(
            Arg::new("mutator")
                .short('m')
                .long("mutator")
                .help("Mutator used to mutate specimens")
                .takes_value(true)
                .possible_values(["Rectangle", "Triangle"])
                .default_value("Rectangle")
                .display_order(30),
        )
        .arg(
            Arg::new("fitness")
                .short('f')
                .long("fitness")
                .help("Fitness function used to calculate fitness of specimens")
                .takes_value(true)
                .possible_values(["SquareDistance", "AbsoluteDistance"])
                .default_value("SquareDistance")
                .display_order(40),
        )
        .arg(
            Arg::new("crossover")
                .short('c')
                .long("crossover")
                .help("Crossover function used to breed specimens")
                .takes_value(true)
                .possible_values(["LeftOrRight", "EqualHalfs", "ArithmeticAverage"])
                .default_value("LeftOrRight")
                .display_order(50),
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
                .display_order(60),
        )
        .arg(
            Arg::new("threads")
                .short('t')
                .long("threads")
                .long_help(THREADS_INFO)
                .takes_value(true)
                .forbid_empty_values(true)
                .default_value("1")
                .value_name("N")
                .validator(validate_threads)
                .display_order(70),
        )
        .arg(
            Arg::new("display_all")
                .long("display-all")
                .long_help(DISPLAY_ALL_INFO)
                .takes_value(false)
                .group("display")
                .display_order(80),
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
                .display_order(90),
        )
        .arg(
            Arg::new("output_directory")
                .long("output-dir")
                .long_help(OUTPUT_DIRECTORY_INFO)
                .takes_value(true)
                .forbid_empty_values(true)
                .value_name("DIR")
                .value_hint(ValueHint::DirPath)
                .display_order(100),
        )
        .arg(
            Arg::new("filename_prefix")
                .long("filename-prefix")
                .long_help(FILENAME_PREFIX_INFO)
                .takes_value(true)
                .forbid_empty_values(true)
                .default_value("output_")
                .value_name("PREFIX")
                .display_order(110),
        )
        .arg(
            Arg::new("save_all")
                .long("save-all")
                .long_help(SAVE_ALL_INFO)
                .takes_value(false)
                .group("output")
                .display_order(120),
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
                .display_order(130),
        )
}

#[derive(Debug)]
pub struct ArgParser {
    arg_matches: ArgMatches,
}

impl ArgParser {
    #[must_use]
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

    #[must_use]
    pub fn is_present(&self, key: &str) -> bool {
        self.arg_matches.is_present(key)
    }
}

impl Default for ArgParser {
    #[must_use]
    fn default() -> Self {
        Self {
            arg_matches: get_app().get_matches(),
        }
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;

    #[test]
    fn image_wasNotPassed_validationFailed() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli"]);

        assert!(result.is_err());
    }

    #[test]
    fn image_valueIsEmpty_validationFailed() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli", "--image", ""]);

        assert!(result.is_err());
    }

    #[test]
    fn image_valueIsNotEmpty_validationPassed() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH"]);

        assert!(result.is_ok());
    }

    #[test]
    fn image_shortNameGiven_validationPassed() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli", "-i", "PATH"]);

        assert!(result.is_ok());
    }

    #[test]
    fn image_valueGiven_correctValueReturned() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli", "-i", "PATH"]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!("PATH", result.value_of("image").unwrap_or_default());
    }

    #[test]
    fn mode_modeIsNotSpecified_defaultValueSet() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH"]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!("Rgb", result.value_of("color_mode").unwrap_or_default());
    }

    #[test]
    fn mode_modeIsSpecified_validationPassed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--mode",
            "Grayscale",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            "Grayscale",
            result.value_of("color_mode").unwrap_or_default()
        );
    }

    #[test]
    fn mode_valueDoesNotExistInPossibleValues_validationFailed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--mode",
            "FOO",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn mutator_mutatorIsNotSpecified_defaultValueSet() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH"]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!("Rectangle", result.value_of("mutator").unwrap_or_default());
    }

    #[test]
    fn mutator_mutatorIsSpecified_validationPassed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--mutator",
            "Rectangle",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!("Rectangle", result.value_of("mutator").unwrap_or_default());
    }

    #[test]
    fn mutator_shortNameGiven_validationPassed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "-m",
            "Triangle",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!("Triangle", result.value_of("mutator").unwrap_or_default());
    }

    #[test]
    fn mutator_valueDoesNotExistInPossibleValues_validationFailed() {
        let result =
            get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH", "-m", "FOO"]);

        assert!(result.is_err());
    }

    #[test]
    fn fitness_fitnessIsNotSpecified_defaultValueSet() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH"]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            "SquareDistance",
            result.value_of("fitness").unwrap_or_default()
        );
    }

    #[test]
    fn fitness_fitnessIsSpecified_validationPassed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--fitness",
            "AbsoluteDistance",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            "AbsoluteDistance",
            result.value_of("fitness").unwrap_or_default()
        );
    }

    #[test]
    fn fitness_shortNameGiven_validationPassed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "-f",
            "SquareDistance",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            "SquareDistance",
            result.value_of("fitness").unwrap_or_default()
        );
    }

    #[test]
    fn fitness_valueDoesNotExistInPossibleValues_validationFailed() {
        let result =
            get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH", "-f", "FOO"]);

        assert!(result.is_err());
    }

    #[test]
    fn crossover_crossoverIsNotSpecified_defaultValueSet() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH"]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            "LeftOrRight",
            result.value_of("crossover").unwrap_or_default()
        );
    }

    #[test]
    fn crossover_crossoverIsSpecified_validationPassed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--crossover",
            "EqualHalfs",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            "EqualHalfs",
            result.value_of("crossover").unwrap_or_default()
        );
    }

    #[test]
    fn crossover_shortNameGiven_validationPassed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "-c",
            "EqualHalfs",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            "EqualHalfs",
            result.value_of("crossover").unwrap_or_default()
        );
    }

    #[test]
    fn crossover_valueDoesNotExistInPossibleValues_validationFailed() {
        let result =
            get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH", "-c", "FOO"]);

        assert!(result.is_err());
    }

    #[test]
    fn crossover_arithmeticAverageExistsInPossibleValues_validationPassed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "-c",
            "ArithmeticAverage",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            "ArithmeticAverage",
            result.value_of("crossover").unwrap_or_default()
        );
    }

    #[test]
    fn generationSize_generationSizeIsNotSpecified_defaultValueSet() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH"]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            100usize,
            result.value_of_t("generation_size").unwrap_or_default()
        );
    }

    #[test]
    fn generationSize_generationSizeIsSpecified_validationPassed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--generation",
            "150",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            150usize,
            result.value_of_t("generation_size").unwrap_or_default()
        );
    }

    #[test]
    fn generationSize_shortNameGiven_validationPassed() {
        let result =
            get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH", "-g", "200"]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            200usize,
            result.value_of_t("generation_size").unwrap_or_default()
        );
    }

    #[test]
    fn generationSize_valueSmallerThan3_validationFailed() {
        let result =
            get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH", "-g", "2"]);

        assert!(result.is_err());
    }

    #[test]
    fn generationSize_valueIsNotNumber_validationFailed() {
        let result =
            get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH", "-g", "FOO"]);

        assert!(result.is_err());
    }

    #[test]
    fn threads_threadsIsNotSpecified_defaultValueSet() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH"]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(1usize, result.value_of_t("threads").unwrap_or_default());
    }

    #[test]
    fn threads_threadsIsSpecified_validationPassed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--threads",
            "6",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(6usize, result.value_of_t("threads").unwrap_or_default());
    }

    #[test]
    fn threads_shortNameGiven_validationPassed() {
        let result =
            get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH", "-t", "15"]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(15usize, result.value_of_t("threads").unwrap_or_default());
    }

    #[test]
    fn threads_valueEqualTo0_validationFailed() {
        let result =
            get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH", "-t", "0"]);

        assert!(result.is_err());
    }

    #[test]
    fn threads_valueIsNotNumber_validationFailed() {
        let result =
            get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH", "-t", "FOO"]);

        assert!(result.is_err());
    }

    #[test]
    fn display_noneOfGroupArgsGiven_displayIsNotPresent() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH"]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_present("display"));
    }

    #[test]
    fn display_displayAllGiven_displayAllIsPresent() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--display-all",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_present("display_all"));
    }

    #[test]
    fn display_displayEveryGiven_displayEveryIsPresent() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--display-every",
            "50",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_present("display_every"));
    }

    #[test]
    fn display_displayEveryGiven_valuePropertyParsed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--display-every",
            "50",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(50, result.value_of_t("display_every").unwrap_or_default());
    }

    #[test]
    fn display_displayAllAndDisplayEveryGiven_validationFailed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--display-all",
            "--display-every",
            "50",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn outputDirectory_outputDirectoryIsNotSpecified_outputDirectoryIsNotPresent() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH"]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_present("output_directory"));
    }

    #[test]
    fn outputDirectory_outputDirectoryIsEmpty_validationFailed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--output-dir",
            "",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn outputDirectory_valueIsNotEmpty_validationPassed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--output-dir",
            "DIR_PATH",
        ]);

        assert!(result.is_ok());
    }

    #[test]
    fn outputDirectory_valueGiven_correctValueReturned() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--output-dir",
            "DIR_PATH",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            "DIR_PATH",
            result.value_of("output_directory").unwrap_or_default()
        );
    }

    #[test]
    fn filenamePrefix_filenamePrefixIsNotSpecified_defaultValue() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH"]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            "output_",
            result.value_of("filename_prefix").unwrap_or_default()
        );
    }

    #[test]
    fn filenamePrefix_filenamePrefixIsEmpty_validationFailed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--filename-prefix",
            "",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn filenamePrefix_valueIsNotEmpty_validationPassed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--filename-prefix",
            "PREFIX",
        ]);

        assert!(result.is_ok());
    }

    #[test]
    fn filenamePrefix_valueGiven_correctValueReturned() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--filename-prefix",
            "PREFIX",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(
            "PREFIX",
            result.value_of("filename_prefix").unwrap_or_default()
        );
    }

    #[test]
    fn output_noneOfGroupArgsGiven_displayIsNotPresent() {
        let result = get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH"]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(!result.is_present("output"));
    }

    #[test]
    fn output_saveAllGiven_saveAllIsPresent() {
        let result =
            get_app().try_get_matches_from(vec!["franklin-cli", "--image", "PATH", "--save-all"]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_present("save_all"));
    }

    #[test]
    fn output_saveEveryGiven_saveEveryIsPresent() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--save-every",
            "50",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert!(result.is_present("save_every"));
    }

    #[test]
    fn output_outputEveryGiven_valuePropertyParsed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--save-every",
            "50",
        ]);

        assert!(result.is_ok());
        let result = result.unwrap();
        assert_eq!(50, result.value_of_t("save_every").unwrap_or_default());
    }

    #[test]
    fn output_saveAllAndSaveEveryGiven_validationFailed() {
        let result = get_app().try_get_matches_from(vec![
            "franklin-cli",
            "--image",
            "PATH",
            "--save-all",
            "--save-every",
            "50",
        ]);

        assert!(result.is_err());
    }
}
