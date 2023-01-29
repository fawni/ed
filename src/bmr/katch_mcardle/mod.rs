pub mod hyprid;

pub fn calculate(weight: f32, bodyfat: f32) -> f32 {
    370.0 + (21.6 * (weight * (1.0 - bodyfat / 100.0)))
}
