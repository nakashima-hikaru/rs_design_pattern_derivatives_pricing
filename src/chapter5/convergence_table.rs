use crate::chapter5::mc_statistics::StatisticsMC;

pub struct ConvergenceTable<'a, T: StatisticsMC> {
    inner: &'a mut T,
    results_so_far: Vec<Vec<f64>>,
    stopping_point: u64,
    paths_done: u64,
}

impl<'a, T: StatisticsMC> ConvergenceTable<'a, T> {
    pub fn new(inner: &'a mut T) -> ConvergenceTable<'a, T> {
        ConvergenceTable {
            inner,
            results_so_far: Vec::<Vec<f64>>::default(),
            stopping_point: 2,
            paths_done: 0,
        }
    }
}

impl<'a, T: StatisticsMC> StatisticsMC for ConvergenceTable<'a, T> {
    fn dump_one_result(&mut self, result: f64) {
        self.inner.dump_one_result(result);
        self.paths_done += 1;
        if self.paths_done == self.stopping_point {
            self.stopping_point *= 2;
            let this_result = self.inner.get_results_so_far();
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

#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;

    struct MockStats {
        results: Vec<f64>,
    }

    impl StatisticsMC for MockStats {
        fn dump_one_result(&mut self, result: f64) {
            self.results.push(result);
        }

        fn get_results_so_far(&self) -> Vec<Vec<f64>> {
            vec![vec![self.results.len() as f64 * 0.5]]
        }
    }

    #[test]
    fn test_convergence_table() {
        let mut stats = MockStats {
            results: Vec::new(),
        };
        let mut conv_table = ConvergenceTable::new(&mut stats);

        let num_paths = 1024u32;
        let expected_results = (2..=num_paths)
            .filter(|n| n.is_power_of_two())
            .map(|n| vec![n as f64 * 0.5, n as f64])
            .collect::<Vec<Vec<f64>>>();

        for i in 1..=num_paths {
            conv_table.dump_one_result(i as f64);
        }

        let results = conv_table.get_results_so_far();
        assert_eq!(results.len(), expected_results.len());
        for (actual, expected) in results.iter().zip(expected_results.iter()) {
            assert_relative_eq!(actual[0], expected[0]);
            assert_relative_eq!(actual[1], expected[1]);
        }
    }
}
