# franklin

[![franklin-ci](https://github.com/nathiss/franklin/actions/workflows/franklin-ci.yml/badge.svg?branch=development)](https://github.com/nathiss/franklin/actions/workflows/franklin-ci.yml)
[![Crates.io](https://img.shields.io/crates/v/franklin)](https://crates.io/crates/franklin)
[![docs.rs](https://docs.rs/franklin/badge.svg)](https://docs.rs/franklin/)
![Crates.io](https://img.shields.io/crates/l/franklin)

A utility for generating [evolutionary art](https://en.wikipedia.org/wiki/Evolutionary_art).

> What is art?  
> Something more than sum of its parts.

## Description

There are two utilities here: `franklin-cli` allowing you to generate images through your command line and `franklin`,
a rust library with some out-of-the-box utilities. It also allows you to define your own components to customize
generation even more.

The underlying process is my implementation of [Evolutionary algorithm](https://en.wikipedia.org/wiki/Evolutionary_algorithm).
The whole process can be split into three parts:

* **mutation** - each specimen of the current generation is being randomly mutated.

* **scoring** - each specimen is being compared with the ideal image (the source) and delta score between those images
is calculated.

* **crossover** - after the scoring the best individual were selected from the generation for breeding. In this step we
mix genes of two parents to give birth to new offspring, until new generation with the same size is produced.

## Usage

If you want to start a simulation it's best to check all available options in `franklin-cli`:

```sh
franklin-cli --help
```

It's also possible to implement your own components, such as mutators, fitness functions or crossover functions.
Go to docs, to see more details.

## License

See [LICENSE.txt](LICENSE.txt) file.
