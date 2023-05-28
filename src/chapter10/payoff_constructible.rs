use crate::chapter10::payoff_factory::PayoffFactory;
use crate::chapter4::payoff3::Payoff;
use std::marker::PhantomData;
use std::sync::{Arc, RwLock};

pub struct PayoffHelper<T: 'static + Payoff> {
    _phantom: PhantomData<T>,
}

impl<T: 'static + Payoff> PayoffHelper<T> {
    fn create(strike: f64) -> Box<dyn Payoff> {
        Box::new(T::new(strike))
    }

    pub fn new(payoff_id: String) -> Self {
        // let the_payoff_factory = &mut PayoffFactory::instance();

        // the_payoff_factory
        let mut t = PayoffFactory::instance().lock().unwrap();

        t.register_payoff(payoff_id, Arc::new(RwLock::new(PayoffHelper::<T>::create)));
        Self {
            _phantom: PhantomData,
        }
    }
}
