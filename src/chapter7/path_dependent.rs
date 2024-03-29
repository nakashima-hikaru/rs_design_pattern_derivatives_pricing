//! 離散型の経路依存型のオプションを取り扱えるようにする。
//! 今回はアメリカンオプションやバミューダンオプションを取り扱わない。
//! ・モンテカルロ法による時価評価のロジック
//! パスごとに割引されたペイオフを生成し、これらの平均を取る。
//! パスごとのペイオフを決定するためには、
//! パスを生成して、そのパスからペイオフの変数となっている時点の株価を得る。
//! この株価たちからキャッシュフローを求め、現時点まで割り引く。
//! したがって以下のステップが必要。
//! 1.パスを生成
//!     パスはメインクラスへの入力なので、クラス階層から定義 or 基底クラスに抽象メソッドとして定義 or テンプレートパラメータとして定義
//!     ここでは抽象メソッドとして定義する。
//! 2. 1.で生成したパスについての全てのキャッシュフローを生成
//!     金利などの情報は何も持たない、単なるデリバティブの定義としてオプションクラスを定義する。
//! 3. 2.の全てのキャッシュフローを割引して足し上げることで時価を得る
//!     キャッシュフロー計算はメインエンジンクラスに組み込む
//! 4.全てのパスで時価の平均を取る。
//!     以前作成したgathererを利用する
//! ・これらのステップの接続
//! input: 時価評価に必要な時間における株価のスポット値
//! output:キャッシュフロー
//! よって、
//! (i)「どの時間の」株価スポット値が必要かという情報を伝える仕組み
//! (ii)キャッシュフローオブジェクトの定義
//! を考える必要がある。
//! (i) 必要な時間の配列を返すget_look_at_timesメソッドを定義する。
//! (ii)単純にキャッシュフローの量と時間をタプルで保持すると、複雑な金利の期間構造を持つ場合にdiscount factorを求めるのに時間がかかってしまう。
//!     よってあらかじめ割引率を参照する必要のある時間の配列をpossible_cashflow_timesというメソッドで求めておく。
//! 時間を配列によって保持するので、キャッシュフローオブジェクトはインデックスと金額のペアで定義する。
//! cash_flowメソッドによって、スポットの配列からキャッシュフローを返す。
#[derive(Default, Clone, Copy)]
/// A cash flow simulated on a path.
pub struct CashFlow {
    /// The forward value of the cash flow
    pub amount: f64,
    /// The time the cash flow arises
    pub time_index: usize,
}

impl CashFlow {
    pub fn new(time_index: usize, amount: f64) -> Self {
        CashFlow { time_index, amount }
    }
}

/// A path-dependent product.
/// The product's payoff depends on the entire path of underlying asset prices.
pub trait PathDependent: Send + Sync {
    /// Returns a reference to a `Vec<f64>` that represents the look at times.
    fn get_look_at_times(&self) -> &Vec<f64>;
    /// Returns the maximum number of cash flows.
    fn max_number_of_cash_flows(&self) -> usize;

    /// Returns times of cash flows to calculate its discount factor.
    fn possible_cash_flow_times(&self) -> Vec<f64>;

    /// Calculates cash flows based on given spot values and updates the provided `generated_flows` with the results.
    ///
    /// # Arguments
    ///
    /// * `spot_values` - A slice of `f64` values representing the spot values.
    /// * `generated_flows` - A mutable slice of `CashFlow` objects where the generated cash flows will be updated.
    fn cash_flows(&self, spot_values: &[f64], generated_flows: &mut [CashFlow]) -> u64;
}
