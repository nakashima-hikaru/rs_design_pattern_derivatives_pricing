const ONE_OVER_ROOT_TWO_PI: f64 = 0.398942280401433;

pub fn normal_density(x: f64) -> f64 {
    ONE_OVER_ROOT_TWO_PI * (-x * x / 2.0).exp()
}

#[inline(always)]
pub fn inverse_cumulative_normal(u: f64) -> f64 {
    if !(-1.0..=1.0).contains(&u) {
        panic!("Parameters of inverse_cumulative must be within the interval [-1, 1].");
    }
    if (0.5..=0.92).contains(&u) {
        const A: [f64; 4] = [
            2.50662823884,
            -18.61500062529,
            41.39119773534,
            -25.44106049637,
        ];
        const B: [f64; 4] = [
            -8.47351093090,
            23.08336743743,
            -21.06224101826,
            3.13082909833,
        ];
        let y = u - 0.5;
        let r = y * y;
        y * (((A[3] * r + A[2]) * r + A[1]) * r + A[0])
            / ((((B[3] * r + B[2]) * r + B[1]) * r + B[0]) * r + 1.0)
    } else if 0.92 < u && u <= 1.0 {
        const C: [f64; 9] = [
            0.3374754822726147,
            0.9761690190917186,
            0.1607979714918209,
            0.0276438810333863,
            0.0038405729373609,
            0.0003951896511919,
            0.0000321767881768,
            0.0000002888167364,
            0.0000003960315187,
        ];
        let r = (-(1.0 - u).ln()).ln();
        C[0] + r
            * (C[1]
                + r * (C[2]
                    + r * (C[3] + r * (C[4] + r * (C[5] + r * (C[6] + r * C[7] + r * C[8]))))))
    } else {
        -inverse_cumulative_normal(1.0 - u)
    }
}

pub fn cumulative_normal(x: f64) -> f64 {
    const A: [f64; 5] = [
        0.319381530,
        -0.356563782,
        1.781477937,
        -1.821255978,
        1.330274429,
    ];
    let mut result: f64;
    if x < -7.0 {
        result = normal_density(x) / (1.0 + x * x).sqrt();
    } else if x > 7.0 {
        result = 1.0 - cumulative_normal(-x);
    } else {
        let tmp = 1.0 / (1.0 + 0.2316419 * x.abs());
        result = 1.0
            - normal_density(x)
                * (tmp * (A[0] + tmp * (A[1] + tmp * (A[2] + tmp * (A[3] + tmp * A[4])))));
        if x <= 0.0 {
            result = 1.0 - result;
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_density() {
        assert_eq!(normal_density(0.0), 0.398942280401433);
        assert_eq!(normal_density(1.0), 0.24197072451914353);
        assert_eq!(normal_density(-1.0), 0.24197072451914353);
    }

    #[test]
    fn test_inverse_cumulative_normal() {
        assert_eq!(inverse_cumulative_normal(0.5), 0.0);
        assert_eq!(inverse_cumulative_normal(0.95), 1.644853553323174);
        assert_eq!(inverse_cumulative_normal(0.999), 3.090195109132865);
    }

    #[test]
    fn test_cumulative_normal() {
        assert_eq!(cumulative_normal(0.0), 0.4999999994751917);
        assert_eq!(cumulative_normal(1.0), 0.8413447404368684);
        assert_eq!(cumulative_normal(-1.0), 0.15865525956313165);
    }
}
