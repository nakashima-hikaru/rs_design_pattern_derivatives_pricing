use crate::chapter10::payoff_constructible::PayoffHelper;
use crate::chapter4::payoff3::{PayoffCall, PayoffPut};
use once_cell::sync::OnceCell;

static REGISTER_CALL: OnceCell<PayoffHelper<PayoffCall>> = OnceCell::new();

static REGISTER_PUT: OnceCell<PayoffHelper<PayoffPut>> = OnceCell::new();

pub fn register_all_payoffs() {
    REGISTER_CALL.get_or_init(|| PayoffHelper::new("call".to_string()));
    REGISTER_PUT.get_or_init(|| PayoffHelper::new("put".to_string()));
}
