const ONE_OVER_ROOT_TWO_PI: f64 = 0.398942280401433;

pub fn normal_density(x: f64) -> f64 {
    ONE_OVER_ROOT_TWO_PI * (-x * x / 2.0).exp()
}

pub fn inverse_cumulative_normal(u: f64) -> f64 {
    if u < -1.0 || u > 1.0 {
        // println!("{}", u);
        panic!("Parameters of inverse_cumulative must be within the interval [-1, 1].");
    }
    if 0.5 <= u && u <= 0.92 {
        let a = [
            2.50662823884,
            -18.61500062529,
            41.39119773534,
            -25.44106049637,
        ];
        let b: [f64; 4] = [
            -8.47351093090,
            23.08336743743,
            -21.06224101826,
            3.13082909833,
        ];
        let y = u - 0.5;
        let r = y * y;
        y * (((a[3] * r + a[2]) * r + a[1]) * r + a[0])
            / ((((b[3] * r + b[2]) * r + b[1]) * r + b[0]) * r + 1.0)
    } else if 0.92 < u && u <= 1.0 {
        let c = [
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
        c[0] + r
            * (c[1]
                + r * (c[2]
                    + r * (c[3] + r * (c[4] + r * (c[5] + r * (c[6] + r * c[7] + r * c[8]))))))
    } else {
        -inverse_cumulative_normal(1.0 - u)
    }
}

pub fn cumulative_normal(x: f64) -> f64 {
    let a = [
        0.319381530,
        -0.356563782,
        1.781477937,
        -1.821255978,
        1.330274429,
    ];
    let mut result: f64;
    if x < -7.0 {
        result = normal_density(x) / (1.0 + x * x).sqrt();
    } else {
        if x > 7.0 {
            result = 1.0 - cumulative_normal(-x);
        } else {
            let tmp = 1.0 / (1.0 + 0.2316419 * x.abs());
            result = 1.0
                - normal_density(x)
                    * (tmp * (a[0] + tmp * (a[1] + tmp * (a[2] + tmp * (a[3] + tmp * a[4])))));
            if x <= 0.0 {
                result = 1.0 - result;
            }
        }
    }
    result
}
