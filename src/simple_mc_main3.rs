mod chapter1;
mod chapter3;

fn main() {
    use crate::chapter3::payoff2;

    println!("\nEnter expiry\n");
    let expiry = text_io::read!();

    println!("\nEnter Strike\n");
    let strike = text_io::read!();

    println!("\nEnter spot\n");
    let spot = text_io::read!();

    println!("\nEnter vol\n");
    let vol = text_io::read!();

    println!("\nEnter r\n");
    let r = text_io::read!();

    println!("\nNumber of paths\n");
    let number_of_paths = text_io::read!();

    let call_payoff = payoff2::PayoffCall::new(strike);
    let put_payoff = payoff2::PayoffPut::new(strike);
    let result_call = <dyn payoff2::Payoff>::simple_montecarlo2(
        &call_payoff,
        expiry,
        spot,
        vol,
        r,
        number_of_paths,
    );

    let result_put = <dyn payoff2::Payoff>::simple_montecarlo2(
        &put_payoff,
        expiry,
        spot,
        vol,
        r,
        number_of_paths,
    );
    println!(
        "the prices are {} for the call and {} for the put\n",
        result_call, result_put
    );
}
