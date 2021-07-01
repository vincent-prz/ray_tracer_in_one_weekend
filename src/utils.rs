pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min
    }
    if x > max {
        return max
    }
    x
}

const PI: f64 = 3.1415926535897932385;

pub fn degrees_to_radians(degree_val: f64) -> f64 {
    PI * degree_val / 180.0
}