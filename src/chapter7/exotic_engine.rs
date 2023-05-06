//! キャッシュフローを格納するVectorを1回のシミュレーションごとに作るのはコンストラクタとデストラクタの呼び出しに時間がかかるので、
//! mutableなメンバ変数にしている。

use crate::chapter4::parameters::Parameters;
use crate::chapter5::mc_statistics::StatisticsMC;
use crate::chapter7::path_dependent::CashFlow;
use crate::chapter7::path_dependent::PathDependent;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::sync::Arc;
use std::sync::Mutex;

pub struct ExoticEngineField<'a> {
    /// A path dependent product such as Asian option
    the_product: &'a dyn PathDependent,
    /// Interest rates
    r: &'a dyn Parameters,
    /// Discount factors
    discounts: Vec<f64>,
    /// Cash flows simulated on paths
    these_cash_flows: Mutex<Vec<CashFlow>>,
}

impl<'a> ExoticEngineField<'a> {
    pub fn new(
        the_product: &'a impl PathDependent,
        r: &'a (impl Parameters + 'a),
    ) -> ExoticEngineField<'a> {
        let these_cash_flows = Mutex::new(vec![
            CashFlow::default();
            the_product.max_number_of_cash_flows()
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
    pub fn get_the_product(&self) -> &'a dyn PathDependent {
        self.the_product
    }
    /// Returns the pointer of `self.r`.
    pub fn get_r(&self) -> &'a dyn Parameters {
        self.r
    }
}

pub trait ExoticEngine {
    /// Returns the pointer of `self.exotic_engine_field`.
    fn as_exotic_engine_field(&self) -> &ExoticEngineField;

    fn get_one_path(&mut self, variates: &mut [f64]);

    fn do_simulation(&mut self, the_gatherer: &mut impl StatisticsMC, number_of_paths: usize)
    where
        Self: Sync,
        Self: Send,
    {
        let spot_values = vec![
            0.0;
            self.as_exotic_engine_field()
                .the_product
                .get_look_at_times()
                .len()
        ];
        let self_ptr = Arc::new(Mutex::new(self));
        let the_gatherer_ptr = Arc::new(Mutex::new(the_gatherer));
        let spot_values_ptr = Arc::new(Mutex::new(spot_values));
        let _: Vec<_> = (0..number_of_paths)
            .into_par_iter()
            .map(|_| {
                let mut locked_self_ptr = self_ptr.lock().unwrap();
                let mut locked_the_gatherer_ptr = the_gatherer_ptr.lock().unwrap();
                let mut locked_spot_values_ptr = spot_values_ptr.lock().unwrap();
                (*locked_self_ptr).get_one_path(&mut *locked_spot_values_ptr);
                let this_value = (*locked_self_ptr).do_one_path(&*locked_spot_values_ptr);
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
            .map(|cash_flow| cash_flow.amount * discounts[cash_flow.time_index])
            .sum()
    }
}
