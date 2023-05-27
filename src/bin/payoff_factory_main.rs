use rust_design_pattern_derivative_pricing::chapter10::payoff_factory::PayoffFactory;
use std::io;

fn main() {
    let mut strike = String::new();
    io::stdin().read_line(&mut strike).unwrap();
    let strike: f64 = strike.trim().parse().unwrap();

    let mut name = String::new();
    io::stdin().read_line(&mut name).unwrap();
    let name = name.trim();

    let payoff_factory = PayoffFactory::instance();
    let payoff = payoff_factory.create_payoff(name, strike);

    if let Some(payoff) = payoff {
        let mut spot = String::new();
        io::stdin().read_line(&mut spot).unwrap();
        let spot: f64 = spot.trim().parse().unwrap();

        println!("{}", payoff.calculate(spot));
    }

    let mut tmp = String::new();
    io::stdin().read_line(&mut tmp).unwrap();
}
