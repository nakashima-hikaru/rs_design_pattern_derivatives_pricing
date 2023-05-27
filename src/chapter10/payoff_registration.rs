use crate::chapter10::payoff_constructible::PayoffHelper;
use crate::chapter4::payoff3::{PayoffCall, PayoffPut};
use once_cell::sync::Lazy;

static REGISTER_CALL: Lazy<PayoffHelper<PayoffCall>> =
    Lazy::new(|| PayoffHelper::new("call".to_string()));
static REGISTER_PUT: Lazy<PayoffHelper<PayoffPut>> =
    Lazy::new(|| PayoffHelper::new("put".to_string()));

pub fn register_all_payoffs() {
    let _ = &*REGISTER_CALL;
    let _ = &*REGISTER_PUT;
}
