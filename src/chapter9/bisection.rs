//! クラスごとにメソッド関数としてbisectionを定義するとvtableを参照する分遅いなど、いろいろ不便なのでテンプレートによって実装する。

pub fn bisection<T: Fn(f64) -> f64>(
    target: f64,
    low: f64,
    high: f64,
    tolerance: f64,
    the_function: T,
) -> f64 {
    let mut low = low;
    let mut high = high;
    let mut x = 0.5 * (low + high);
    let mut y = the_function(x);
    loop {
        if y < target {
            low = x;
        }
        if y > target {
            high = x;
        }
        x = 0.5 * (low + high);
        y = the_function(x);
        if (y - target).abs() <= tolerance {
            break;
        }
    }
    x
}
