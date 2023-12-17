//! キャッシュフローを格納するVectorを1回のシミュレーションごとに作るのはコンストラクタとデストラクタの呼び出しに時間がかかるので、
//! mutableなメンバ変数にしている。
use crate::chapter4::parameters::Parameters;
use crate::chapter5::mc_statistics::StatisticsMC;
use crate::chapter7::path_dependent::CashFlow;
use crate::chapter7::path_dependent::PathDependent;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex};

pub struct ExoticEngineData<'a, T: PathDependent + ?Sized, S: Parameters> {
    /// A path dependent product such as Asian option
    the_product: &'a T,
    /// Interest rates
    r: &'a S,
    /// Discount factors
    discounts: Vec<f64>,
    /// Cash flows simulated on paths
    these_cash_flows: Vec<CashFlow>,
}

impl<'a, T: PathDependent + ?Sized, S: Parameters> ExoticEngineData<'a, T, S> {
    pub fn new(the_product: &'a T, r: &'a S) -> ExoticEngineData<'a, T, S> {
        let these_cash_flows = vec![CashFlow::default(); the_product.max_number_of_cash_flows()];
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

    fn do_one_path(&mut self, spot_values: &[f64]) -> f64 {
        self.these_cash_flows.clear();
        self.these_cash_flows.resize_with(
            self.the_product.max_number_of_cash_flows(),
            CashFlow::default,
        );
        self.the_product
            .cash_flows(spot_values, &mut self.these_cash_flows);
        let discounts = &self.discounts;
        self.these_cash_flows
            .iter()
            .map(|cash_flow| cash_flow.amount * discounts[cash_flow.time_index])
            .sum()
    }
}

pub trait ExoticEngine<T: PathDependent + ?Sized, S: Parameters> {
    /// Returns the pointer of `self.exotic_engine_data`.

    fn get_one_path(&mut self, variates: &mut [f64]);

    fn do_simulation(
        &mut self,
        data: &mut ExoticEngineData<T, S>,
        the_gatherer: &mut impl StatisticsMC,
        number_of_paths: usize,
    ) where
        Self: Sync,
        Self: Send,
    {
        let spot_values = vec![0.0; data.the_product.get_look_at_times().len()];
        let self_ptr = Arc::new(Mutex::new(self));
        let the_gatherer_ptr = Arc::new(Mutex::new(the_gatherer));
        let spot_values_ptr = Arc::new(Mutex::new(spot_values));
        let data_ptr = Arc::new(Mutex::new(data));
        (0..number_of_paths).into_par_iter().for_each(|_| {
            let mut locked_self_ptr = self_ptr.lock().unwrap();
            let mut locked_the_gatherer_ptr = the_gatherer_ptr.lock().unwrap();
            let mut locked_spot_values_ptr = spot_values_ptr.lock().unwrap();
            let mut locked_data_ptr = data_ptr.lock().unwrap();
            (*locked_self_ptr).get_one_path(&mut locked_spot_values_ptr);
            let this_value = (*locked_data_ptr).do_one_path(&locked_spot_values_ptr);
            (*locked_the_gatherer_ptr).dump_one_result(this_value);
        });
    }
}
