//! キャッシュフローを格納するVectorを1回のシミュレーションごとに作るのはコンストラクタとデストラクタの呼び出しに時間がかかるので、
//! mutableなメンバ変数にしている。
use crate::chapter4::parameters::Parameters;
use crate::chapter5::mc_statistics::StatisticsMC;
use crate::chapter7::path_dependent::CashFlow;
use crate::chapter7::path_dependent::PathDependent;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, RwLock};

pub struct ExoticEngineData<'a, T: PathDependent, S: Parameters> {
    /// A path dependent product such as Asian option
    the_product: &'a T,
    /// Interest rates
    r: &'a S,
    /// Discount factors
    discounts: Vec<f64>,
    /// Cash flows simulated on paths
    these_cash_flows: Arc<RwLock<Vec<CashFlow>>>,
}

impl<'a, T: PathDependent, S: Parameters> ExoticEngineData<'a, T, S> {
    pub fn new(the_product: &'a T, r: &'a S) -> ExoticEngineData<'a, T, S> {
        let these_cash_flows = Arc::new(RwLock::new(vec![
            CashFlow::default();
            the_product.max_number_of_cash_flows()
        ]));
        let discounts = the_product
            .possible_cash_flow_times()
            .iter_mut()
            .map(|discount| (-r.integral(0.0, *discount)).exp())
            .collect();
        ExoticEngineData {
            the_product,
            r,
            discounts,
            these_cash_flows,
        }
    }
    /// Returns the pointer of `self.the_product`.
    pub fn get_the_product(&self) -> &'a T {
        self.the_product
    }
    /// Returns the pointer of `self.r`.
    pub fn get_r(&self) -> &'a S {
        self.r
    }
}

pub trait ExoticEngine<T: PathDependent, S: Parameters> {
    /// Returns the pointer of `self.exotic_engine_data`.
    fn get_exotic_engine_data(&self) -> &ExoticEngineData<T, S>;

    fn get_one_path(&mut self, variates: &mut [f64]);

    fn do_simulation(&mut self, the_gatherer: &mut impl StatisticsMC, number_of_paths: usize)
    where
        Self: Sync,
        Self: Send,
    {
        let spot_values = vec![
            0.0;
            self.get_exotic_engine_data()
                .the_product
                .get_look_at_times()
                .len()
        ];
        let self_ptr = Arc::new(RwLock::new(self));
        let the_gatherer_ptr = Arc::new(RwLock::new(the_gatherer));
        let spot_values_ptr = Arc::new(RwLock::new(spot_values));
        (0..number_of_paths).into_par_iter().for_each(|_| {
            let mut locked_self_ptr = self_ptr.write().unwrap();
            let mut locked_the_gatherer_ptr = the_gatherer_ptr.write().unwrap();
            let mut locked_spot_values_ptr = spot_values_ptr.write().unwrap();
            (*locked_self_ptr).get_one_path(&mut locked_spot_values_ptr);
            let this_value = (*locked_self_ptr).do_one_path(&locked_spot_values_ptr);
            (*locked_the_gatherer_ptr).dump_one_result(this_value);
        });
    }

    fn do_one_path(&self, spot_values: &[f64]) -> f64 {
        let these_cash_flows = &mut *self
            .get_exotic_engine_data()
            .these_cash_flows
            .write()
            .unwrap();
        these_cash_flows.clear();
        these_cash_flows.resize_with(
            self.get_exotic_engine_data()
                .the_product
                .max_number_of_cash_flows(),
            CashFlow::default,
        );
        self.get_exotic_engine_data()
            .the_product
            .cash_flows(spot_values, these_cash_flows);
        let discounts = &self.get_exotic_engine_data().discounts;
        these_cash_flows
            .iter()
            .map(|cash_flow| cash_flow.amount * discounts[cash_flow.time_index])
            .sum()
    }
}
