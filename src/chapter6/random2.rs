use crate::chapter6::normals::inverse_cumulative_normal;

/// 標準ライブラリのrandはコンパイラごとの実装に依存する。
/// そのため再現性がなくテストが困難であり、実装された乱数の性能を知るのが難しい。
///
/// 「Standコマンドで」Seedを設定して再現性を持たせたい。
/// Seedはグローバル変数なので他のシミュレーションと競合することを防ぐことができる。
/// さらに、クラスを修飾することができる。
/// 疑似乱数ではなく、Loq Discrepancy Numberを使用することもできる。
///
/// 累積関数の逆関数を通して一様乱数を正規乱数に変換するため、[0,1]区間から0,1は除いてサンプリングする。

pub trait RandomBase {
    fn get_dimensionality(&self) -> u32;
    fn get_uniforms(&mut self, variates: &mut [f64]);
    fn skip(&mut self, number_of_paths: u32);
    fn set_seed(&mut self, seed: u32);
    fn reset(&mut self);
    fn get_gaussians(&mut self, variates: &mut [f64]) {
        self.get_uniforms(variates);
        for i in 0..self.get_dimensionality() {
            let x = variates[i as usize];
            variates[i as usize] = inverse_cumulative_normal(x);
        }
    }
    fn reset_dimensionality(&mut self, new_dimensionality: u32);
}

#[derive(Clone)]
pub struct RandomBaseField {
    pub dimensionality: u32,
}

impl RandomBaseField {
    pub fn new(dimensionality: u32) -> RandomBaseField {
        RandomBaseField { dimensionality }
    }
}
