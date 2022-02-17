/// Bridgeパターンを利用する。
use std::convert::From;
trait ParametersInner {
    fn box_clone(&self) -> Box<dyn ParametersInner>;
    fn integral(&self, time1: f64, time2: f64) -> f64;
    fn integral_square(&self, time1: f64, time2: f64) -> f64;
}

impl Clone for Box<dyn ParametersInner> {
    fn clone(&self) -> Box<dyn ParametersInner> {
        self.box_clone()
    }
}

#[derive(Clone)]
pub struct Parameters {
    inner_object_ptr: Box<dyn ParametersInner>,
}

impl Parameters {
    fn new(inner_object: Box<dyn ParametersInner>) -> Parameters {
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
#[derive(Copy, Clone)]
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
    fn box_clone(&self) -> Box<dyn ParametersInner> {
        Box::new((*self).clone())
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
        let inner_object = Box::new(ParametersConstant::new(x));
        Parameters::new(inner_object)
    }
}
