use crate::chapter4::parameters::Parameters;
use crate::chapter5::mc_statistics::StatisticsMC;
use crate::chapter7::path_dependent::CashFlow;
use crate::chapter7::path_dependent::PathDependent;
use std::cell::RefCell;
pub struct ExoticEngineField {
    the_product: Box<dyn PathDependent>,
    r: Parameters,
    discounts: Vec<f64>,
    these_cash_flows: RefCell<Vec<CashFlow>>,
}

impl ExoticEngineField {
    fn new(the_product: Box<dyn PathDependent>, r: &Parameters) -> ExoticEngineField {
        let these_cash_flows = RefCell::new(vec![
            CashFlow::default();
            the_product.max_number_of_cash_flow() as usize
        ]);
        let mut discounts = the_product.possible_cash_flow_times();
        for i in 0..discounts.len() {
            discounts[i] = (-r.integral(0.0, discounts[i])).exp();
        }
        ExoticEngineField {
            the_product,
            r: r.clone(),
            discounts,
            these_cash_flows,
        }
    }
}
pub trait ExoticEngine {
    fn as_exotic_engine_filed(&self) -> &ExoticEngineField;
    // fn as_mut_exotic_engine_filed(&mut self) -> &mut ExoticEngineField;
    fn get_one_path(&mut self, spot_values: &mut [f64]);
    fn do_one_path(&self, spot_values: &[f64]) -> f64 {
        let number_flows = self.as_exotic_engine_filed().the_product.cash_flows(
            spot_values,
            &self.as_exotic_engine_filed().these_cash_flows.borrow(),
        );
        let mut value = 0.0;
        for i in 0..number_flows {
            value += self.as_exotic_engine_filed().these_cash_flows.borrow()[i as usize].amount;
        }
        value
    }
    fn do_simulation(&mut self, the_gatherer: &mut dyn StatisticsMC, number_of_paths: u64) {
        let mut spot_values = vec![
            0.0;
            self.as_exotic_engine_filed()
                .the_product
                .get_look_at_times()
                .len()
        ];
        let sz = self
            .as_exotic_engine_filed()
            .the_product
            .max_number_of_cash_flow() as usize;
        self.as_exotic_engine_filed()
            .these_cash_flows
            .borrow_mut()
            .resize(sz, CashFlow::default());
        for _i in 0..number_of_paths {
            self.get_one_path(&mut spot_values);
            let this_value = self.do_one_path(&spot_values);
            the_gatherer.dump_one_result(this_value);
        }
    }
}
