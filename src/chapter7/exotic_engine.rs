use crate::chapter4::parameters::Parameters;
use crate::chapter5::mc_statistics::StatisticsMC;
use crate::chapter7::path_dependent::CashFlow;
use crate::chapter7::path_dependent::PathDependent;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct ExoticEngineField {
    /// A path dependent product such as Asian option
    the_product: Rc<dyn PathDependent>,
    /// Interest rates
    r: Rc<Parameters>,
    /// Discount factors
    discounts: Vec<f64>,
    /// Cash flows simulated on paths
    these_cash_flows: RefCell<Vec<CashFlow>>,
}

impl ExoticEngineField {
    pub fn new(the_product: Rc<dyn PathDependent>, r: Rc<Parameters>) -> ExoticEngineField {
        let these_cash_flows = RefCell::new(vec![
            CashFlow::default();
            the_product.max_number_of_cash_flows() as usize
        ]);
        let mut discounts = the_product.possible_cash_flow_times();
        for discount in discounts.iter_mut() {
            *discount = (-r.integral(0.0, *discount)).exp();
        }
        ExoticEngineField {
            the_product,
            r,
            discounts,
            these_cash_flows,
        }
    }
    /// Returns the pointer of `self.the_product`.
    pub fn get_the_product(&self) -> &Rc<dyn PathDependent> {
        &self.the_product
    }
    /// Returns the pointer of `self.r`.
    pub fn get_r(&self) -> &Rc<Parameters> {
        &self.r
    }
}

pub trait ExoticEngine {
    /// Returns the pointer of `self.exotic_engine_field`.
    fn as_exotic_engine_field(&self) -> &ExoticEngineField;

    fn get_one_path(&mut self, spot_values: &mut [f64]);

    fn do_simulation(&mut self, the_gatherer: &mut dyn StatisticsMC, number_of_paths: u64) {
        let mut spot_values = vec![
            0.0;
            self.as_exotic_engine_field()
                .the_product
                .get_look_at_times()
                .len()
        ];
        let sz = self
            .as_exotic_engine_field()
            .the_product
            .max_number_of_cash_flows() as usize;

        self.as_exotic_engine_field()
            .these_cash_flows
            .borrow_mut()
            .resize(sz, CashFlow::default());
        for _i in 0..number_of_paths {
            self.get_one_path(&mut spot_values);
            let this_value = self.do_one_path(&spot_values);
            the_gatherer.dump_one_result(this_value);
        }
    }

    /// Returns the present value of the product on a path.
    ///
    /// # Arguments
    ///
    /// * `spot_values` - Spot values on a path.
    fn do_one_path(&self, spot_values: &[f64]) -> f64 {
        let number_flows = self.as_exotic_engine_field().the_product.cash_flows(
            spot_values,
            &mut self.as_exotic_engine_field().these_cash_flows.borrow_mut(),
        );
        let mut value = 0.0;
        for i in 0..number_flows {
            let field = self.as_exotic_engine_field();
            let cashflow = &field.these_cash_flows.borrow()[i as usize];
            value += cashflow.amount * field.discounts[cashflow.time_index as usize];
        }
        value
    }
}
