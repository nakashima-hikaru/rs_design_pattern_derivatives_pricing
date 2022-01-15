/// * テキスト内での注意事項
///
/// ** Payoffクラスの設計
/// 変更点
/// ・enumの代わりにtraitを用いてオプションの種類ごとにstructを定義した。
/// 改善点
/// ・新しいオプションの追加で既存のコードに影響が生じない。
/// ・(double digitalなどの)メンバ変数がstrikeのみでない場合であっても対応できる。
/// 課題
/// ・既存のファイルを変更することなく新しいオプションを追加したい。

pub trait Payoff {
    fn value(&self, spot: f64) -> f64;
}

pub struct PayoffCall {
    strike: f64,
}
pub struct PayoffPut {
    strike: f64,
}
impl PayoffCall {
    pub fn new(strike: f64) -> Self {
        Self { strike }
    }
}
impl PayoffPut {
    pub fn new(strike: f64) -> Self {
        Self { strike }
    }
}

impl Payoff for PayoffCall {
    fn value(&self, spot: f64) -> f64 {
        (spot - self.strike).max(0.0)
    }
}
impl Payoff for PayoffPut {
    fn value(&self, spot: f64) -> f64 {
        (self.strike - spot).max(0.0)
    }
}
