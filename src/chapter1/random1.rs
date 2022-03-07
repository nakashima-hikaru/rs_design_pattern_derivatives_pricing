use rand::Rng;

pub fn get_one_gaussian_by_simulation() -> f64 {
    let mut result: f64 = 0.0;
    let mut rng = rand::thread_rng();
    for _j in 0..12 {
        result += rng.gen::<f64>();
    }
    result -= 6.0;
    result
}

pub fn get_one_gaussian_by_box_muller() -> f64 {
    let mut x;
    let mut y;
    let mut size_squared;
    let mut rng = rand::thread_rng();
    loop {
        x = 2.0 * rng.gen::<f64>() - 1.0;
        y = 2.0 * rng.gen::<f64>() - 1.0;
        size_squared = x * x + y * y;
        if size_squared < 1.0 {
            break;
        }
    }
    x * (-2.0 * size_squared.ln() / size_squared).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_random1() {
        fn test_get_one_gaussian_by_simulation() {
            get_one_gaussian_by_simulation();
        }
        test_get_one_gaussian_by_simulation();
    }
}
