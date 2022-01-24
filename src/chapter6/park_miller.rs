/// minimal standard generatorと呼ばれ、保証される統計的な正確性が最低限のもの。
/// generatorは二つの部分に分かれており、
/// ParkMiller-structは乱数を生成し、
/// RandomParkMiller-structはParkMiller-structの出力した乱数を一様乱数のベクトルに変換する。
use crate::chapter6::random2::RandomBase;
use crate::chapter6::random2::RandomBaseField;

#[derive(Clone, Copy)]
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
    pub fn set_seed(&mut self, seed: i64) {
        self.seed = seed;
        if self.seed == 0 {
            self.seed = 1;
        }
    }
    pub fn max(&self) -> u64 {
        (ParkMiller::M - 1) as u64
    }
    pub fn min(&self) -> u64 {
        1
    }
    pub fn get_one_random_integer(&mut self) -> i64 {
        let k = self.seed / ParkMiller::Q;
        self.seed = ParkMiller::A * (self.seed - k * ParkMiller::Q) - k * ParkMiller::R;
        if self.seed < 0 {
            self.seed += ParkMiller::M;
        }
        self.seed
    }
}

#[derive(Clone)]
struct RandomParkMiller {
    random_base: RandomBaseField,
    inner_generator: ParkMiller,
    initial_seed: u64,
    reciprocal: f64,
}

impl RandomParkMiller {
    pub fn new(dimensionality: u64, seed: u64) -> RandomParkMiller {
        let inner_generator = ParkMiller::new(seed as i64);
        RandomParkMiller {
            random_base: RandomBaseField::new(dimensionality),
            inner_generator,
            initial_seed: seed,
            reciprocal: 1.0 / (1.0 + inner_generator.max() as f64),
        }
    }
}

impl RandomBase for RandomParkMiller {
    fn box_clone(&self) -> Box<dyn RandomBase> {
        Box::new((*self).clone())
    }
    fn get_dimensionality(&self) -> u64 {
        self.random_base.dimensionality
    }
    fn get_uniforms(&mut self, variates: &mut [f64]) {
        for j in 0..self.get_dimensionality() {
            variates[j as usize] =
                (self.inner_generator.get_one_random_integer() as f64) * self.reciprocal;
        }
    }
    fn skip(&mut self, number_of_paths: u64) {
        let mut tmp = Vec::<f64>::with_capacity(self.get_dimensionality() as usize);
        for _j in 0..number_of_paths {
            self.get_uniforms(tmp.as_mut_slice());
        }
    }
    fn set_seed(&mut self, seed: u64) {
        self.initial_seed = seed;
        self.inner_generator.set_seed(seed as i64);
    }
    fn reset(&mut self) {
        self.inner_generator.set_seed(self.initial_seed as i64);
    }
    fn reset_dimensionality(&mut self, new_dimensionality: u64) {
        self.random_base.dimensionality = new_dimensionality;
        self.inner_generator.set_seed(self.initial_seed as i64);
    }
}

#[test]
fn test_distribution() {
    let n = 100000;
    let mut x = RandomParkMiller::new(n, 25435344);
    let mut v = Vec::<f64>::with_capacity(n as usize);
    for _i in 0..n {
        v.push(0.0);
    }

    x.get_gaussians(&mut v.as_mut_slice());
    let mut mean = 0.0;
    let mut variant = 0.0;
    for u in v {
        mean += u;
        variant += u * u;
    }
    mean /= n as f64;
    variant /= n as f64;
    println!("{}, {}", mean, variant);
}
