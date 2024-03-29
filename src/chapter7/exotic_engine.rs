//! キャッシュフローを格納するVectorを1回のシミュレーションごとに作るのはコンストラクタとデストラクタの呼び出しに時間がかかるので、
//! mutableなメンバ変数にしている。
use crate::chapter4::parameters::Parameters;
use crate::chapter5::mc_statistics::StatisticsMC;
use crate::chapter7::path_dependent::CashFlow;
use crate::chapter7::path_dependent::PathDependent;
use rayon::current_thread_index;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use std::sync::{Arc, Mutex};

pub struct ExoticEngineData<'a, T: PathDependent + ?Sized> {
    /// A path dependent product such as Asian option
    the_product: &'a T,
    /// Discount factors
    discount_factors: Vec<f64>,
}

impl<'a, T: PathDependent + ?Sized> ExoticEngineData<'a, T> {
    pub fn new(the_product: &'a T, r: &impl Parameters) -> Self {
        let discounts = the_product
            .possible_cash_flow_times()
            .iter_mut()
            .map(|discount| (-r.integral(0.0, *discount)).exp())
            .collect();
        ExoticEngineData {
            the_product,
            discount_factors: discounts,
        }
    }
    fn do_one_path(&self, spot_values: &[f64], these_cash_flows: &mut Vec<CashFlow>) -> f64 {
        these_cash_flows.resize_with(
            self.the_product.max_number_of_cash_flows(),
            CashFlow::default,
        );
        self.the_product.cash_flows(spot_values, these_cash_flows);
        these_cash_flows
            .iter()
            .map(|cash_flow| cash_flow.amount * self.discount_factors[cash_flow.time_index])
            .sum()
    }
}

pub trait ExoticEngine<T: PathDependent + ?Sized>: Clone {
    fn get_one_path(&mut self, variates: &mut [f64]);

    fn set_seed(&mut self, seed: u64);

    fn skip(&mut self, number_of_paths: usize);

    fn do_simulation(
        &mut self,
        data: &ExoticEngineData<T>,
        the_gatherer: &mut impl StatisticsMC,
        number_of_paths: usize,
    ) where
        Self: Sync,
        Self: Send,
    {
        let length_of_times = data.the_product.get_look_at_times().len();
        let max_number_of_cash_flows = data.the_product.max_number_of_cash_flows();
        let the_gatherer_ptr = Arc::new(Mutex::new(the_gatherer));
        (0..number_of_paths).into_par_iter().for_each_init(
            || {
                (
                    vec![0.0; length_of_times],
                    vec![CashFlow::default(); max_number_of_cash_flows],
                    {
                        let num_skip = current_thread_index().unwrap() * length_of_times;
                        let mut ret = self.clone();
                        ret.skip(num_skip);
                        ret
                    },
                )
            },
            |(spot_values, these_cash_flows, cloned_self), _| {
                cloned_self.get_one_path(spot_values);
                let this_value = data.do_one_path(spot_values, these_cash_flows);
                (*the_gatherer_ptr.lock().unwrap()).dump_one_result(this_value);
            },
        );
    }
}
