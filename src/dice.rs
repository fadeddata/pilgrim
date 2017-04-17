use rand::{SeedableRng, StdRng};
use rand::distributions::{IndependentSample, Range};

pub struct Die {
    pub sides: u8,
    rng: StdRng,
}

impl<'a> Die {
    pub fn roll(&mut self, n: u8) -> Vec<u8> {
        let between = Range::new(1, self.sides + 1);
        let mut result = Vec::new();

        for _ in 0..n {
            let a = between.ind_sample(&mut self.rng);
            result.push(a);
        }

        result
    }

    pub fn add_roll(&mut self, n: u8) -> u8 {
        self.roll(n).iter().sum()
    }

    pub fn new(sides: u8, seed: &'a [usize]) -> Die {
        let rng: StdRng = SeedableRng::from_seed(seed);

        Die {
            sides: sides,
            rng: rng,
        }
    }
}