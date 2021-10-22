#[derive(Debug)]
#[allow(dead_code)]
pub enum OptionType {
    Call,
    Put,
}

#[allow(dead_code)]
pub struct Payoff {
    strike: f64,
    the_option_type: OptionType,
}

impl Payoff {
    #[allow(dead_code)]
    pub fn new(strike: f64, the_option_type: OptionType) -> Payoff {
        Payoff {
            strike,
            the_option_type,
        }
    }

    #[allow(dead_code)]
    pub fn forward_value(&self, spot: f64) -> f64 {
        #[allow(unreachable_patterns)]
        match self.the_option_type {
            OptionType::Call => (spot - self.strike).max(0.0),
            OptionType::Put => (self.strike - spot).max(0.0),
            _ => panic!("unknown option type found."),
        }
    }
}
