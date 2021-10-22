pub trait Payoff {
    fn forward_value(&self, spot: f64) -> f64;
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
    fn forward_value(&self, spot: f64) -> f64 {
        (spot - self.strike).max(0.0)
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
    fn forward_value(&self, spot: f64) -> f64 {
        (self.strike - spot).max(0.0)
    }
}

impl Clone for PayoffCall {
    fn clone(&self) -> Self {
        Self::new(self.strike)
    }
}

impl Clone for PayoffPut {
    fn clone(&self) -> Self {
        Self::new(self.strike)
    }
}
