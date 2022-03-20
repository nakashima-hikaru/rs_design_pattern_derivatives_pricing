//! Bridgeパターンを利用する。
use std::{convert::From, rc::Rc};

pub trait ParametersInner {
    fn value_at(&self, x: f64) -> f64;
    fn integral(&self, time1: f64, time2: f64) -> f64;
    fn integral_square(&self, time1: f64, time2: f64) -> f64;
}

#[derive(Clone)]
pub struct Parameters {
    inner_object_ptr: Rc<dyn ParametersInner>,
}

impl Parameters {
    fn new(inner_object: Rc<dyn ParametersInner>) -> Parameters {
        Parameters {
            inner_object_ptr: inner_object,
        }
    }

    #[inline]
    pub fn integral(&self, time1: f64, time2: f64) -> f64 {
        self.inner_object_ptr.integral(time1, time2)
    }

    #[inline]
    pub fn integral_square(&self, time1: f64, time2: f64) -> f64 {
        self.inner_object_ptr.integral_square(time1, time2)
    }

    pub fn mean(&self, time1: f64, time2: f64) -> f64 {
        let total = self.integral(time1, time2);
        total / (time2 - time1)
    }

    pub fn root_mean_square(&self, time1: f64, time2: f64) -> f64 {
        let total = self.integral_square(time1, time2);
        total / (time2 - time1)
    }
}
#[derive(Clone)]
pub struct ParametersConstant {
    constant: f64,
    constant_square: f64,
}

impl ParametersConstant {
    pub fn new(constant: f64) -> Self {
        Self {
            constant,
            constant_square: constant * constant,
        }
    }
}

impl ParametersInner for ParametersConstant {
    fn value_at(&self, _x: f64) -> f64 {
        self.constant
    }
    fn integral(&self, time1: f64, time2: f64) -> f64 {
        (time2 - time1) * self.constant
    }

    fn integral_square(&self, time1: f64, time2: f64) -> f64 {
        (time2 - time1) * self.constant_square
    }
}

impl From<f64> for Parameters {
    fn from(x: f64) -> Self {
        let inner_object = Rc::new(ParametersConstant::new(x));
        Parameters::new(inner_object)
    }
}

use superslice::Ext;
pub struct ParametersRightContinuousPiecewiseConstant {
    constants: Vec<f64>,
    discontinuous_points: Vec<f64>,

    cached_integrals: Vec<f64>,
    cached_square_integrals: Vec<f64>,
}

impl ParametersRightContinuousPiecewiseConstant {
    pub fn new(constants: &[f64], discontinuous_points: &[f64]) -> Self {
        // validation
        assert!(constants.len() == discontinuous_points.len() + 1);
        assert!(discontinuous_points.is_sorted());

        let mut cached_integrals = Vec::<f64>::default();
        let mut cached_square_integrals = Vec::<f64>::default();
        let mut tmp = 0.0;
        let mut tmp2 = 0.0;

        // cache \int_d[0]^d[i] f(t) dt for all i > 0
        for i in 1..discontinuous_points.len() {
            let tmp3 = (discontinuous_points[i] - discontinuous_points[i - 1]) * constants[i];
            tmp += tmp3;
            tmp2 += tmp3 * constants[i];
            cached_integrals.push(tmp);
            cached_square_integrals.push(tmp2);
        }

        Self {
            constants: constants.to_vec(),
            discontinuous_points: discontinuous_points.to_vec(),
            cached_integrals,
            cached_square_integrals,
        }
    }
}

impl ParametersInner for ParametersRightContinuousPiecewiseConstant {
    fn value_at(&self, x: f64) -> f64 {
        let idx = self
            .discontinuous_points
            .upper_bound_by(|a| a.partial_cmp(&x).unwrap());
        self.constants[idx]
    }
    fn integral(&self, time1: f64, time2: f64) -> f64 {
        let idx1 = self
            .discontinuous_points
            .upper_bound_by(|a| a.partial_cmp(&time1).unwrap());
        let idx2 = self
            .discontinuous_points
            .upper_bound_by(|a| a.partial_cmp(&time2).unwrap());
        self.cached_integrals[idx2] - self.cached_integrals[idx1]
            + (self.cached_integrals[idx1] - time1) * self.constants[idx1]
            - (time2 - self.cached_integrals[idx2]) * self.constants[idx2]
    }

    fn integral_square(&self, time1: f64, time2: f64) -> f64 {
        let idx1 = self
            .discontinuous_points
            .upper_bound_by(|a| a.partial_cmp(&time1).unwrap());
        let idx2 = self
            .discontinuous_points
            .upper_bound_by(|a| a.partial_cmp(&time2).unwrap());
        self.cached_square_integrals[idx2] - self.cached_square_integrals[idx1]
            + (self.cached_square_integrals[idx1] - time1) * self.constants[idx1]
            - (time2 - self.cached_square_integrals[idx2]) * self.constants[idx2]
    }
}

pub struct ParametersLeftContinuousPiecewiseConstant {
    constants: Vec<f64>,
    discontinuous_points: Vec<f64>,

    cached_integrals: Vec<f64>,
    cached_square_integrals: Vec<f64>,
}

impl ParametersLeftContinuousPiecewiseConstant {
    pub fn new(constants: &[f64], discontinuous_points: &[f64]) -> Self {
        // validation
        assert!(constants.len() == discontinuous_points.len() + 1);
        assert!(discontinuous_points.is_sorted());

        let mut cached_integrals = Vec::<f64>::default();
        let mut cached_square_integrals = Vec::<f64>::default();
        let mut tmp = 0.0;
        let mut tmp2 = 0.0;

        // cache \int_d[0]^d[i] f(t) dt for all i > 0
        for i in 1..discontinuous_points.len() {
            let tmp3 = (discontinuous_points[i] - discontinuous_points[i - 1]) * constants[i];
            tmp += tmp3;
            tmp2 += tmp3 * constants[i];
            cached_integrals.push(tmp);
            cached_square_integrals.push(tmp2);
        }

        Self {
            constants: constants.to_vec(),
            discontinuous_points: discontinuous_points.to_vec(),
            cached_integrals,
            cached_square_integrals,
        }
    }
}

impl ParametersInner for ParametersLeftContinuousPiecewiseConstant {
    fn value_at(&self, x: f64) -> f64 {
        let idx = self
            .discontinuous_points
            .lower_bound_by(|a| a.partial_cmp(&x).unwrap());
        self.constants[idx]
    }
    fn integral(&self, time1: f64, time2: f64) -> f64 {
        let idx1 = self
            .discontinuous_points
            .upper_bound_by(|a| a.partial_cmp(&time1).unwrap());
        let idx2 = self
            .discontinuous_points
            .upper_bound_by(|a| a.partial_cmp(&time2).unwrap());
        self.cached_integrals[idx2] - self.cached_integrals[idx1]
            + (self.cached_integrals[idx1] - time1) * self.constants[idx1]
            - (time2 - self.cached_integrals[idx2]) * self.constants[idx2]
    }

    fn integral_square(&self, time1: f64, time2: f64) -> f64 {
        let idx1 = self
            .discontinuous_points
            .upper_bound_by(|a| a.partial_cmp(&time1).unwrap());
        let idx2 = self
            .discontinuous_points
            .upper_bound_by(|a| a.partial_cmp(&time2).unwrap());
        self.cached_square_integrals[idx2] - self.cached_square_integrals[idx1]
            + (self.cached_square_integrals[idx1] - time1) * self.constants[idx1]
            - (time2 - self.cached_square_integrals[idx2]) * self.constants[idx2]
    }
}

#[test]
fn test_left_continuous() {
    // When StepType is LeftContinuous,
    // f(x) = `constants[0]` if x <=`discontinuous_points[0]`,
    // f(x) = `constants[i]` if `discontinuous_points[i-1]` < x <=`discontinuous_points[i]` for all i >= 1 and
    // f(x) = `constants[n]` if `discontinuous_points[n]` < x.
    let constants = [3.0, 2.0, 1.0, 5.0];
    let discontinuous_points = [-100.0, 200.0, 500.0];
    let f = ParametersLeftContinuousPiecewiseConstant::new(&constants, &discontinuous_points);
    assert_eq!(f.value_at(discontinuous_points[0] - 100.0), constants[0]);
    assert_eq!(f.value_at(discontinuous_points[0]), constants[0]);
    assert_eq!(
        f.value_at((discontinuous_points[0] + discontinuous_points[1]) * 0.5),
        constants[1]
    );
    assert_eq!(f.value_at(discontinuous_points[1]), constants[1]);
    assert_eq!(
        f.value_at((discontinuous_points[1] + discontinuous_points[2]) * 0.5),
        constants[2]
    );
    assert_eq!(f.value_at(discontinuous_points[2]), constants[2]);
    assert_eq!(f.value_at(discontinuous_points[2] + 100.0), constants[3]);
}

#[test]
fn test_right_continuous() {
    // When StepType is RightContinuous,
    // f(x) = `constants[0]` if x <`discontinuous_times[0]`,
    // f(x) = `constants[i]` if `discontinuous_times[i-1]` <= x <`discontinuous_times[i]` for all i >= 1 and
    // f(x) = `constants[n]` if `discontinuous_points[n]` <= x.
    let constants = [3.0, 2.0, 1.0, 5.0];
    let discontinuous_points = [-100.0, 200.0, 500.0];
    let f = ParametersRightContinuousPiecewiseConstant::new(&constants, &discontinuous_points);
    assert_eq!(f.value_at(discontinuous_points[0] - 100.0), constants[0]);
    assert_eq!(f.value_at(discontinuous_points[0]), constants[1]);
    assert_eq!(
        f.value_at((discontinuous_points[0] + discontinuous_points[1]) * 0.5),
        constants[1]
    );
    assert_eq!(f.value_at(discontinuous_points[1]), constants[2]);
    assert_eq!(
        f.value_at((discontinuous_points[1] + discontinuous_points[2]) * 0.5),
        constants[2]
    );
    assert_eq!(f.value_at(discontinuous_points[2]), constants[3]);
    assert_eq!(f.value_at(discontinuous_points[2] + 100.0), constants[3]);
}
