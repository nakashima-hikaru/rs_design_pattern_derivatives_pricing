use std::convert::From;

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

#[derive(Debug, Clone, Copy)]
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
}
