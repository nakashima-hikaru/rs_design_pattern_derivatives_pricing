use rand::Rng;

pub fn get_one_gaussian_by_simulation() -> f64 {
    let mut rng = rand::thread_rng();
    (0..12).map(|_x| rng.gen::<f64>() - 0.5).sum()
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
