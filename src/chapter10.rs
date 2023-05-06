use crate::chapter4::payoff3::Payoff;
use std::collections::HashMap;

#[derive(Clone)]
struct PayoffFactory<'a, CreatePayoffFunction>
where
    CreatePayoffFunction: Fn(f64) -> Box<dyn Payoff + 'a>,
{
    the_creator_functions: HashMap<&'a str, CreatePayoffFunction>,
}

impl<'a, CreatePayoffFunction> PayoffFactory<'a, CreatePayoffFunction>
where
    CreatePayoffFunction: Fn(f64) -> Box<dyn Payoff + 'a>,
{
    pub fn register_payoff(
        &'a mut self,
        payoff_id: &'a str,
        creator_function: CreatePayoffFunction,
    ) {
        self.the_creator_functions
            .insert(payoff_id, creator_function);
    }
    pub fn create_payoff(&'a self, payoff_id: &'a str, strike: f64) -> Box<dyn Payoff + 'a> {
        self.the_creator_functions
            .get_key_value(payoff_id)
            .expect("{payoff_id} is an unknown payoff")
            .1(strike)
    }
}

// impl<CreatePayoffFunction> PayoffFactory<'static, CreatePayoffFunction>
// where
//     CreatePayoffFunction: Fn(f64) -> Box<dyn Payoff + 'static>,
// {
//     pub fn instance() -> PayoffFactory<'static, CreatePayoffFunction> {
//         static the_factory: SyncOnceCell<Mutex<PayoffFactory<CreatePayoffFunction>>> = SyncOnceCell::new();
//         the_factory
//     }
// }
