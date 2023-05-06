//! ## 自前で乱数クラスを実装する理由
//! 1. 標準ライブラリのrandはコンパイラごとの実装依存
//!     - 再現性がなくテストが困難
//!     - Seedをグローバル変数とすることで再現性を保証できるようになるが、他の操作を挟むと乱数の値が変化してしまう。
//!     - 乱数の性能の測定が困難
//!
//! 1. クラスを修飾することができる
//!     - e.g. このクラスを利用してAntiTheticサンプリングを実装できる
//! 1. 疑似乱数ではなく、Loq Discrepancy Numberを使用することもできる。
//!
//! 累積関数の逆関数を通して一様乱数を正規乱数に変換するため、[[0,1]]区間から0,1は除いてサンプリングする。

use crate::chapter6::normals::inverse_cumulative_normal;
// 11.89s -> 10.42s
pub trait Random: Send + Sync {
    fn get_dimensionality(&self) -> usize;
    fn get_uniforms(&mut self, variates: &mut [f64]);
    fn skip(&mut self, number_of_paths: u64);
    fn set_seed(&mut self, seed: u64);
    fn reset(&mut self);
    fn get_gaussians(&mut self, variates: &mut [f64]) {
        self.get_uniforms(variates);
        for variate in variates {
            *variate = inverse_cumulative_normal(*variate);
        }
    }
    fn reset_dimensionality(&mut self, new_dimensionality: usize);
}
