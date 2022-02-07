use rand::{
    distributions::{uniform::SampleUniform, Uniform},
    prelude::{Distribution, StdRng},
    SeedableRng,
};

pub struct Random {
    rng: StdRng,
}

impl Random {
    pub fn get_random<T>(&mut self, min: T, max: T) -> T
    where
        T: SampleUniform,
    {
        Uniform::new(min, max).sample(&mut self.rng)
    }

    pub fn get_rng(&mut self) -> &mut StdRng {
        &mut self.rng
    }
}

impl Default for Random {
    fn default() -> Self {
        Self {
            rng: rand::rngs::StdRng::from_entropy(),
        }
    }
}
