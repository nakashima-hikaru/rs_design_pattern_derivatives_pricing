//! Bridgeパターンを利用する。
use std::{convert::From, sync::Arc};

pub trait Parameters: Send + Sync {
    fn value_at(&self, x: f64) -> f64;
    fn integral(&self, time1: f64, time2: f64) -> f64;
    fn integral_square(&self, time1: f64, time2: f64) -> f64;
    fn mean(&self, time1: f64, time2: f64) -> f64 {
        let total = self.integral(time1, time2);
        total / (time2 - time1)
    }
    fn root_mean_square(&self, time1: f64, time2: f64) -> f64 {
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

impl Parameters for ParametersConstant {
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

impl From<f64> for ParametersConstant {
    fn from(x: f64) -> Self {
        ParametersConstant::new(x)
    }
}

use superslice::Ext;

pub trait ParametersPiecewiseConstantInner: Send + Sync {
    fn get_discontinuous_points(&self) -> &Vec<f64>;
    fn get_constants(&self) -> &Vec<f64>;
    fn get_cached_integrals(&self) -> &Vec<f64>;
    fn get_cached_square_integrals(&self) -> &Vec<f64>;
    fn value_at(&self, x: f64) -> f64;
}

pub struct ParametersPiecewiseConstant {
    inner_obj_ptr: Arc<dyn ParametersPiecewiseConstantInner>,
}

impl ParametersPiecewiseConstant {
    pub fn new(inner_obj_ptr: Arc<dyn ParametersPiecewiseConstantInner>) -> Self {
        Self { inner_obj_ptr }
    }
}

impl Parameters for ParametersPiecewiseConstant {
    fn value_at(&self, x: f64) -> f64 {
        self.inner_obj_ptr.value_at(x)
    }
    fn integral(&self, time1: f64, time2: f64) -> f64 {
        if time1 == time2 {
            return 0.0;
        }
        if time2 < time1 {
            return -self.integral(time2, time1);
        }
        let constants = self.inner_obj_ptr.get_constants();
        let discontinuous_points = self.inner_obj_ptr.get_discontinuous_points();
        let cached_integrals = self.inner_obj_ptr.get_cached_integrals();
        let idx1 = discontinuous_points.upper_bound_by(|a| a.partial_cmp(&time1).unwrap());
        let idx2 = discontinuous_points.lower_bound_by(|a| a.partial_cmp(&time2).unwrap());
        if idx1 == discontinuous_points.len() && idx2 == discontinuous_points.len() {
            return (time2 - time1) * constants[constants.len() - 1];
        } else if idx2 == discontinuous_points.len() {
            return cached_integrals[cached_integrals.len() - 1] - cached_integrals[idx1]
                + (discontinuous_points[idx1] - time1) * constants[idx1]
                + constants[constants.len() - 1]
                    * (time2 - discontinuous_points[discontinuous_points.len() - 1]);
        } else {
            return cached_integrals[idx2] - cached_integrals[idx1]
                + (discontinuous_points[idx1] - time1) * constants[idx1]
                + (time2 - discontinuous_points[idx2]) * constants[idx2];
        }
    }
    fn integral_square(&self, time1: f64, time2: f64) -> f64 {
        if time1 == time2 {
            return 0.0;
        }
        if time2 < time1 {
            return -self.integral(time2, time1);
        }
        let constants = self.inner_obj_ptr.get_constants();
        let discontinuous_points = self.inner_obj_ptr.get_discontinuous_points();
        let cached_square_integrals = self.inner_obj_ptr.get_cached_square_integrals();
        let idx1 = discontinuous_points.upper_bound_by(|a| a.partial_cmp(&time1).unwrap());
        let idx2 = discontinuous_points.lower_bound_by(|a| a.partial_cmp(&time2).unwrap());
        if idx1 == discontinuous_points.len() && idx2 == discontinuous_points.len() {
            return (time2 - time1)
                * constants[constants.len() - 1]
                * constants[constants.len() - 1];
        } else if idx2 == discontinuous_points.len() {
            return cached_square_integrals[cached_square_integrals.len() - 1]
                - cached_square_integrals[idx1]
                + (discontinuous_points[idx1] - time1) * constants[idx1] * constants[idx1]
                + constants[constants.len() - 1]
                    * constants[constants.len() - 1]
                    * (time2 - discontinuous_points[discontinuous_points.len() - 1]);
        } else {
            return cached_square_integrals[idx2] - cached_square_integrals[idx1]
                + (discontinuous_points[idx1] - time1) * constants[idx1] * constants[idx1]
                + (time2 - discontinuous_points[idx2]) * constants[idx2] * constants[idx2];
        }
    }
}
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

impl ParametersPiecewiseConstantInner for ParametersRightContinuousPiecewiseConstant {
    fn get_discontinuous_points(&self) -> &Vec<f64> {
        &self.discontinuous_points
    }
    fn get_constants(&self) -> &Vec<f64> {
        &self.constants
    }
    fn get_cached_integrals(&self) -> &Vec<f64> {
        &self.cached_integrals
    }
    fn get_cached_square_integrals(&self) -> &Vec<f64> {
        &self.cached_square_integrals
    }
    fn value_at(&self, x: f64) -> f64 {
        let idx = self
            .discontinuous_points
            .upper_bound_by(|a| a.partial_cmp(&x).unwrap());
        self.constants[idx]
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

        let mut cached_integrals = vec![0.0; 1];
        let mut cached_square_integrals = vec![0.0; 1];
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

impl ParametersPiecewiseConstantInner for ParametersLeftContinuousPiecewiseConstant {
    fn get_discontinuous_points(&self) -> &Vec<f64> {
        &self.discontinuous_points
    }
    fn get_constants(&self) -> &Vec<f64> {
        &self.constants
    }
    fn get_cached_integrals(&self) -> &Vec<f64> {
        &self.cached_integrals
    }
    fn get_cached_square_integrals(&self) -> &Vec<f64> {
        &self.cached_square_integrals
    }
    fn value_at(&self, x: f64) -> f64 {
        let idx = self
            .discontinuous_points
            .lower_bound_by(|a| a.partial_cmp(&x).unwrap());
        self.constants[idx]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_value_at() {
        let params = ParametersConstant::new(2.0);
        assert_eq!(params.value_at(1.0), 2.0);
    }

    #[test]
    fn test_integral() {
        let params = ParametersConstant::new(2.0);
        assert_eq!(params.integral(0.0, 1.0), 2.0);
        assert_eq!(params.integral(0.0, 2.0), 4.0);
    }

    #[test]
    fn test_integral_square() {
        let params = ParametersConstant::new(2.0);
        assert_eq!(params.integral_square(0.0, 1.0), 4.0);
        assert_eq!(params.integral_square(0.0, 2.0), 8.0);
    }

    #[test]
    fn test_mean() {
        let params = ParametersConstant::new(2.0);
        assert_eq!(params.mean(0.0, 1.0), 2.0);
        assert_eq!(params.mean(0.0, 2.0), 2.0);
    }

    #[test]
    fn test_root_mean_square() {
        let params = ParametersConstant::new(2.0);
        assert_eq!(params.root_mean_square(0.0, 1.0), 4.0);
        assert_eq!(params.root_mean_square(0.0, 2.0), 4.0);
    }

    #[test]
    fn test_from() {
        let params: ParametersConstant = 2.0.into();
        assert_eq!(params.constant, 2.0);
        assert_eq!(params.constant_square, 4.0);
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
    fn test_integrals() {
        let constants = [3.0, 2.0, 1.0, 5.0];
        let discontinuous_points = [-100.0, 200.0, 500.0];
        let f = ParametersPiecewiseConstant::new(Arc::new(
            ParametersLeftContinuousPiecewiseConstant::new(&constants, &discontinuous_points),
        ));
        assert_eq!(f.integral(-150.0, -120.0), 90.0);
        assert_eq!(f.integral(-120.0, -50.0), 160.0);
        assert_eq!(f.integral(-120.0, 300.0), 760.0);
        assert_eq!(f.integral(-120.0, 1000.0), 3460.0);
        assert_eq!(f.integral(700.0, 1000.0), 1500.0);
        assert_eq!(
            f.integral(discontinuous_points[0], discontinuous_points[1]),
            600.0
        );
        // case time1 > time2
        assert_eq!(f.integral(-120.0, -150.0), -90.0);
        assert_eq!(f.integral(-50.0, -120.0), -160.0);
        assert_eq!(f.integral(300.0, -120.0), -760.0);
        assert_eq!(f.integral(1000.0, -120.0), -3460.0);
        assert_eq!(f.integral(1000.0, 700.0), -1500.0);
        assert_eq!(
            f.integral(discontinuous_points[0], discontinuous_points[1]),
            600.0
        );
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
}
