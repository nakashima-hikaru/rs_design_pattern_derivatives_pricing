//! * テキスト内での注意事項
//! ・メンバ変数は基本的にmut修飾詞を用いずに宣言する。
//! ・例えばstrikeの情報をこのクラスの外部で取得したいときは、
//!   get_strikeという名前のpublicではないメソッドを用意して、このメソッドを通じて取得する。
//!
//! ** Payoffクラスの設計
//! enumによってオプションの種類を表現する。
//! 課題
//! ・次に新しい種類のオプションを追加するときに、
//! OptionType enumなどでPayoffクラスに変更を加える必要がある。
//! というのは、「新しい種類のオプションの追加」という変更の影響を限定的にしたいからだ。

#[derive(Debug)]
pub enum OptionType {
    Call,
    Put,
}

pub struct Payoff {
    strike: f64,
    the_option_type: OptionType,
}

impl Payoff {
    pub fn new(strike: f64, the_option_type: OptionType) -> Payoff {
        Payoff {
            strike,
            the_option_type,
        }
    }

    pub fn forward_value(&self, spot: f64) -> f64 {
        match self.the_option_type {
            OptionType::Call => (spot - self.strike).max(0.0),
            OptionType::Put => (self.strike - spot).max(0.0),
        }
    }
}
