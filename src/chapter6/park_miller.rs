/// minimal standard generatorと呼ばれ、保証される統計的な正確性が最低限のもの。
/// generatorは二つの部分に分かれており、
/// ParkMiller-structは乱数を生成し、
/// RandomParkMiller-structはParkMiller-structの出力した乱数を一様乱数のベクトルに変換する。
use crate::chapter6::random2::RandomBase;
use crate::chapter6::random2::RandomBaseField;

const A: u32 = 16807;
const M: u32 = 2147483647;
const Q: u32 = 127773;
const R: u32 = 2836;

#[derive(Clone, Copy)]
struct ParkMiller {
    seed: u32,
}

impl ParkMiller {
    pub fn new(mut seed: u32) -> ParkMiller {
        if seed == 0 {
            seed = 1;
        }
        ParkMiller { seed }
    }
    pub fn set_seed(&mut self, seed: u32) {
        self.seed = seed;
        if self.seed == 0 {
            self.seed = 1;
        }
    }
    pub fn max(&self) -> u32 {
        M - 1
    }
    pub fn min(&self) -> u32 {
        1
    }
    pub fn get_one_random_integer(&mut self) -> u32 {
        let k: u32 = self.seed / Q;
        self.seed = A * (self.seed - k + Q) - R * k;
        self.seed
    }
}

#[derive(Clone)]
struct RandomParkMiller {
    random_base: RandomBaseField,
    inner_generator: ParkMiller,
    initial_seed: u32,
    reciprocal: f64,
}

impl RandomParkMiller {
    pub fn new(dimensionality: u32, seed: u32) -> RandomParkMiller {
        let inner_generator = ParkMiller::new(seed);
        RandomParkMiller {
            random_base: RandomBaseField::new(dimensionality),
            inner_generator,
            initial_seed: seed,
            reciprocal: 1.0 / (1.0 + inner_generator.max() as f64),
        }
    }
}

impl RandomBase for RandomParkMiller {
    fn get_dimensionality(&self) -> u32 {
        self.random_base.dimensionality
    }
    fn get_uniforms(&mut self, variates: &mut [f64]) {
        for j in 0..self.get_dimensionality() {
            // let y = *variate;
            variates[j as usize] =
                (self.inner_generator.get_one_random_integer() as f64) * self.reciprocal;
        }
    }
    fn skip(&mut self, number_of_paths: u32) {
        let mut tmp = Vec::<f64>::with_capacity(self.get_dimensionality() as usize);
        for j in 0..number_of_paths {
            self.get_uniforms(tmp.as_mut_slice());
        }
    }
    fn set_seed(&mut self, seed: u32) {
        self.initial_seed = seed;
        self.inner_generator.set_seed(seed);
    }
    fn reset(&mut self) {
        self.inner_generator.set_seed(self.initial_seed);
    }
    fn reset_dimensionality(&mut self, new_dimensionality: u32) {
        self.random_base.dimensionality = new_dimensionality;
        self.inner_generator.set_seed(self.initial_seed);
    }
}
