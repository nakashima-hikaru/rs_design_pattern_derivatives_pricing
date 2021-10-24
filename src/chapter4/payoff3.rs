/// difference from payoff2:
/// define clone-method for deep copy implemention.
pub trait Payoff {
    fn value(&self, spot: f64) -> f64;
}

pub struct PayoffCall {
    strike: f64,
}

impl PayoffCall {
    pub fn new(strike: f64) -> Self {
        Self { strike }
    }
}

impl Payoff for PayoffCall {
    fn value(&self, spot: f64) -> f64 {
        (spot - self.strike).max(0.0)
    }
}

impl Clone for PayoffCall {
    fn clone(&self) -> Self {
        Self::new(self.strike)
    }
}
pub struct PayoffPut {
    strike: f64,
}

impl PayoffPut {
    pub fn new(strike: f64) -> Self {
        Self { strike }
    }
}

impl Payoff for PayoffPut {
    fn value(&self, spot: f64) -> f64 {
        (self.strike - spot).max(0.0)
    }
}

impl Clone for PayoffPut {
    fn clone(&self) -> Self {
        Self::new(self.strike)
    }
}
