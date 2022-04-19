use crate::chapter4::parameters::Parameters;
use crate::chapter5::mc_statistics::StatisticsMC;
use crate::chapter7::path_dependent::CashFlow;
use crate::chapter7::path_dependent::PathDependent;
use rayon::prelude::*;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ExoticEngineField {
    /// A path dependent product such as Asian option
    the_product: Arc<dyn PathDependent>,
    /// Interest rates
    r: Box<dyn Parameters>,
    /// Discount factors
    discounts: Vec<f64>,
    /// Cash flows simulated on paths
    these_cash_flows: Mutex<Vec<CashFlow>>,
}

impl ExoticEngineField {
    pub fn new(the_product: Arc<dyn PathDependent>, r: Box<dyn Parameters>) -> ExoticEngineField {
        let these_cash_flows = Mutex::new(vec![
            CashFlow::default();
            the_product.max_number_of_cash_flows() as usize
        ]);
        let discounts = the_product
            .possible_cash_flow_times()
            .iter_mut()
            .map(|discount| (-r.integral(0.0, *discount)).exp())
            .collect();
        ExoticEngineField {
            the_product,
            r,
            discounts,
            these_cash_flows,
        }
    }
    /// Returns the pointer of `self.the_product`.
    pub fn get_the_product(&self) -> &Arc<dyn PathDependent> {
        &self.the_product
    }
    /// Returns the pointer of `self.r`.
    pub fn get_r(&self) -> &Box<dyn Parameters> {
        &self.r
    }
}

pub trait ExoticEngine {
    /// Returns the pointer of `self.exotic_engine_field`.
    fn as_exotic_engine_field(&self) -> &ExoticEngineField;

    fn get_one_path(&mut self) -> Vec<f64>;

    fn do_simulation(&mut self, the_gatherer: &mut dyn StatisticsMC, number_of_paths: u64)
    where
        Self: Sync,
        Self: Send,
    {
        let self_ptr = Arc::new(Mutex::new(self));
        let the_gatherer_ptr = Arc::new(Mutex::new(the_gatherer));
        let _: Vec<_> = (0..number_of_paths)
            .into_par_iter()
            .map(|_| {
                let self_ptr = Arc::clone(&self_ptr);
                let the_gatherer_ptr = Arc::clone(&the_gatherer_ptr);
                let mut locked_self_ptr = self_ptr.lock().unwrap();
                let mut locked_the_gatherer_ptr = the_gatherer_ptr.lock().unwrap();
                let spot_values = (*locked_self_ptr).get_one_path();
                let this_value = (*locked_self_ptr).do_one_path(&spot_values);
                (*locked_the_gatherer_ptr).dump_one_result(this_value);
            })
            .collect();
    }

    fn do_one_path(&self, spot_values: &[f64]) -> f64 {
        self.as_exotic_engine_field().the_product.cash_flows(
            spot_values,
            &mut self
                .as_exotic_engine_field()
                .these_cash_flows
                .lock()
                .as_mut()
                .unwrap(),
        );
        let discounts = &self.as_exotic_engine_field().discounts;
        self.as_exotic_engine_field()
            .these_cash_flows
            .lock()
            .as_ref()
            .unwrap()
            .iter()
            .map(|cash_flow| cash_flow.amount * discounts[cash_flow.time_index as usize])
            .sum()
    }
}
