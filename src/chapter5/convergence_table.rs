use crate::chapter5::mc_statistics::StatisticsMC;

pub struct ConvergenceTable<'a> {
    inner: &'a mut dyn StatisticsMC,
    results_so_far: Vec<Vec<f64>>,
    stopping_point: u64,
    paths_done: u64,
}

impl<'a> ConvergenceTable<'a> {
    pub fn new(inner: &'a mut impl StatisticsMC) -> ConvergenceTable<'a> {
        ConvergenceTable {
            inner,
            results_so_far: Vec::<Vec<f64>>::default(),
            stopping_point: 2,
            paths_done: 0,
        }
    }
}

impl<'a> StatisticsMC for ConvergenceTable<'a> {
    fn dump_one_result(&mut self, result: f64) {
        self.inner.dump_one_result(result);
        self.paths_done += 1;
        if self.paths_done == self.stopping_point {
            self.stopping_point *= 2;
            let this_result = self.inner.get_results_so_far().clone();
            for mut res in this_result {
                res.push(self.paths_done as f64);
                self.results_so_far.push(res);
            }
        }
    }
    fn get_results_so_far(&self) -> Vec<Vec<f64>> {
        let mut tmp = self.results_so_far.clone();
        if self.paths_done * 2 != self.stopping_point {
            let this_result = self.inner.get_results_so_far();
            for mut res in this_result {
                res.push(self.paths_done as f64);
                tmp.push(res);
            }
        }
        tmp
    }
}
