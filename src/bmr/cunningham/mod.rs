pub fn calculate(weight: f32, bodyfat: f32) -> f32 {
    500.0 + (22.0 * (weight * (1.0 - bodyfat / 100.0)))
}
