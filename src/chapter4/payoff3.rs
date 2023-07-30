//! payoff2.rsとの違い: Clone traitを実装することで値渡しにした。

pub trait Payoff: Send + Sync {
    fn name() -> &'static str
    where
        Self: Sized;
    fn new(strike: f64) -> Self
    where
        Self: Sized;
    fn calculate(&self, spot: f64) -> f64;
}

#[derive(Clone, Debug)]
pub struct PayoffCall {
    strike: f64,
}

impl Payoff for PayoffCall {
    fn name() -> &'static str {
        "call"
    }
    fn new(strike: f64) -> Self {
        Self { strike }
    }
    fn calculate(&self, spot: f64) -> f64 {
        (spot - self.strike).max(0.0)
    }
}

#[derive(Clone, Debug)]
pub struct PayoffPut {
    strike: f64,
}

impl Payoff for PayoffPut {
    fn name() -> &'static str {
        "put"
    }
    fn new(strike: f64) -> Self {
        Self { strike }
    }
    fn calculate(&self, spot: f64) -> f64 {
        (self.strike - spot).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_payoff_call() {
        let strike = 100.0;
        let payoff_call = PayoffCall::new(strike);

        // Test in the money
        let spot = 110.0;
        let expected = spot - strike;
        let result = payoff_call.calculate(spot);
        assert_eq!(result, expected);

        // Test out of the money
        let spot = 90.0;
        let expected = 0.0;
        let result = payoff_call.calculate(spot);
        assert_eq!(result, expected);

        // Test at the money
        let spot = 100.0;
        let expected = 0.0;
        let result = payoff_call.calculate(spot);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_payoff_put() {
        let strike = 100.0;
        let payoff_put = PayoffPut::new(strike);

        // Test in the money
        let spot = 90.0;
        let expected = strike - spot;
        let result = payoff_put.calculate(spot);
        assert_eq!(result, expected);

        // Test out of the money
        let spot = 110.0;
        let expected = 0.0;
        let result = payoff_put.calculate(spot);
        assert_eq!(result, expected);

        // Test at the money
        let spot = 100.0;
        let expected = 0.0;
        let result = payoff_put.calculate(spot);
        assert_eq!(result, expected);
    }
}
