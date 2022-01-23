/// モンテカルロ収束指標を取得するために、統計収集オブジェクトを作りたい。
/// モンテカルロはエキゾチックオプションプライサーなどの色々なプライサーで利用されたり、
/// リスクシステムの開発に利用されたりするため、再利用可能な形でオブジェクトを定義したい。
///
/// オブジェクトには以下の機能が必要:
/// *各パスごとのデータを取り込む(指定した一つの統計データを取得するようにすることで無駄な統計データの計算を省く)
/// *統計データを出力する(Vec<Vec<T>>を戻り値とすることでテーブルを返すことができる)
///
/// PayoffやParameterとは違って統計データはコピーをすることが滅多にないため、
/// 参照を利用することで、わざわざBridgeパターンを使う必要がない。

pub trait StatisticsMC {
    fn box_clone(&self) -> Box<dyn StatisticsMC>;
    fn dump_one_result(&mut self, result: f64);
    fn get_results_so_far(&self) -> Vec<Vec<f64>>;
}

impl Clone for Box<dyn StatisticsMC> {
    fn clone(&self) -> Box<dyn StatisticsMC> {
        self.box_clone()
    }
}

#[derive(Clone, Default)]
pub struct StatisticsMean {
    running_sum: f64,
    paths_done: u32,
}

impl StatisticsMC for StatisticsMean {
    fn box_clone(&self) -> Box<dyn StatisticsMC> {
        Box::new((*self).clone())
    }
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