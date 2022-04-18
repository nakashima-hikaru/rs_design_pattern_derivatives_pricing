//! モンテカルロ収束指標を取得するために、統計収集オブジェクトを作りたい。
//! モンテカルロはエキゾチックオプションプライサーなどの色々なプライサーで利用されたり、
//! リスクシステムの開発に利用されたりするため、再利用可能な形でオブジェクトを定義したい。
//!
//! オブジェクトには以下の機能が必要:
//! *各パスごとのデータを取り込む(指定した一つの統計データを取得するようにすることで無駄な統計データの計算を省く)
//! *統計データを出力する(Vec<Vec<T>>を戻り値とすることでテーブルを返すことができる)
//!
//! PayoffやParameterとは違って統計データはコピーをすることが滅多にないため、
//! 参照を利用することで、わざわざBridgeパターンを使う必要がない。

/// Statistics used in the Monte Carlo method.
pub trait StatisticsMC: Send + Sync {
    /// Updates the internal information required to obtain the relevant statistic results.
    ///
    /// # Arguments
    ///
    /// * result - A result on a path.
    fn dump_one_result(&mut self, result: f64);

    /// Gets statistic results at the moment.
    fn get_results_so_far(&self) -> Vec<Vec<f64>>;
}

#[derive(Default)]
/// For obtaining mean values which equals to `running_sum`/`paths_done`.
pub struct StatisticsMean {
    /// The sum of all result so far.
    running_sum: f64,
    /// The number of paths so far.
    paths_done: u64,
}

impl StatisticsMC for StatisticsMean {
    fn dump_one_result(&mut self, result: f64) {
        self.paths_done += 1;
        self.running_sum += result;
    }
    fn get_results_so_far(&self) -> Vec<Vec<f64>> {
        let mut results = vec![vec![0.0; 1]; 1];
        results[0][0] = self.running_sum / self.paths_done as f64;
        results
    }
}
