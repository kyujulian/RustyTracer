use rand::prelude::*;

pub fn degrees_to_radians(degrees: f64) -> f64 {
    return degrees * std::f64::consts::PI / 180.0;
}

pub fn random_double() -> f64 {
    //Returns a random real in [0,1).

    let mut rng = rand::thread_rng();

    return rng.gen();
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    //Returns a random real in [min,max).

    let mut rng = rand::thread_rng();

    return rng.gen_range(min..max);
}
