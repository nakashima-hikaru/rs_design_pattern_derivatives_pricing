//! minimal standard generatorと呼ばれ、保証される統計的な正確性が最低限のもの。
//! generatorは二つの部分に分かれており、
//! ParkMiller-structは乱数を生成し、
//! RandomParkMiller-structはParkMiller-structの出力した乱数を一様乱数のベクトルに変換する。
use crate::chapter6::random2::RandomBase;

/// A linear congruential generator.
/// See \[ParkMiller\] p.1196.
///
/// \[ParkMiler\] Park, S. K. and Keith W. Miller. “Random number generators: good ones are hard to find.” Commun. ACM 31 (1988): 1192-1201.
struct ParkMiller {
    seed: i64,
}

impl ParkMiller {
    const A: i64 = 16807;
    const M: i64 = 2147483647;
    const Q: i64 = 127773;
    const R: i64 = 2836;
    pub fn new(mut seed: i64) -> ParkMiller {
        if seed == 0 {
            seed = 1;
        }
        ParkMiller { seed }
    }
    fn set_seed(&mut self, seed: i64) {
        self.seed = seed;
        if self.seed == 0 {
            self.seed = 1;
        }
    }

    /// The maximum number of generated random integers.
    fn max(&self) -> u64 {
        (ParkMiller::M - 1) as u64
    }

    /// Get a random integer in the interval \[0, M\].
    pub fn get_one_random_integer(&mut self) -> i64 {
        let k = self.seed / ParkMiller::Q;
        self.seed = ParkMiller::A * (self.seed - k * ParkMiller::Q) - k * ParkMiller::R;
        if self.seed < 0 {
            self.seed += ParkMiller::M;
        }
        self.seed
    }
}

pub struct RandomParkMiller {
    dimensionality: u64,
    inner_generator: ParkMiller,
    initial_seed: u64,
    /// Converts random integers to random number in \[0,1\].
    reciprocal: f64,
}

impl RandomParkMiller {
    pub fn new(dimensionality: u64, seed: u64) -> RandomParkMiller {
        let inner_generator = ParkMiller::new(seed as i64);
        let reciprocal = 1.0 / (1.0 + inner_generator.max() as f64);
        RandomParkMiller {
            dimensionality,
            inner_generator,
            initial_seed: seed,
            reciprocal,
        }
    }
}

impl RandomBase for RandomParkMiller {
    fn get_dimensionality(&self) -> u64 {
        self.dimensionality
    }

    /// Set uniform variables to `variates`.
    fn get_uniforms(&mut self, variates: &mut [f64]) {
        variates.copy_from_slice(
            &(0..self.get_dimensionality())
                .map(|_j| (self.inner_generator.get_one_random_integer() as f64) * self.reciprocal)
                .collect::<Vec<f64>>(),
        );
    }

    /// Skips random number generating
    ///
    /// # Arguments
    ///
    /// * `number_of_paths` - The number of paths to skip.
    fn skip(&mut self, number_of_paths: u64) {
        let mut tmp = vec![0.0; self.get_dimensionality() as usize];
        for _j in 0..number_of_paths {
            self.get_uniforms(&mut tmp);
        }
    }

    /// Set an initial seed.
    fn set_seed(&mut self, seed: u64) {
        self.initial_seed = seed;
        self.inner_generator.set_seed(seed as i64);
    }

    fn reset(&mut self) {
        self.inner_generator.set_seed(self.initial_seed as i64);
    }

    /// Updates dimensionality of generated random numbers.
    fn reset_dimensionality(&mut self, new_dimensionality: u64) {
        self.dimensionality = new_dimensionality;
        self.inner_generator.set_seed(self.initial_seed as i64);
    }
}

#[test]
fn test_distribution() {
    let n = 100000;
    let mut x = RandomParkMiller::new(n, 0);
    let mut v = vec![0.0; n as usize];

    x.get_gaussians(&mut v);
    let mut mean = 0.0;
    let mut variant = 0.0;
    for u in v {
        mean += u;
        variant += u * u;
    }
    mean /= n as f64;
    variant /= n as f64;
    assert_eq!(mean, 0.00047708248676497185);
    assert_eq!(variant, 0.9987128274353647);
}
