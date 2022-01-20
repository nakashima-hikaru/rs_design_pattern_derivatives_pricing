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

trait RandomBase {
    fn get_dimensionality(&self) -> usize;
    fn get_uniforms(&self, variates: &[f64]);
    fn skip(&self, number_of_paths: u32);
    fn set_seed(&self, seed: u32);
    fn reset(&self);
    fn get_gaussians(&self, variates: &mut [f64]) {
        self.get_uniforms(variates);
        for i in 0..self.get_dimensionality() {
            let x = variates[i];
            variates[i] = inverse_cumulative_normal(x);
        }
    }
}
