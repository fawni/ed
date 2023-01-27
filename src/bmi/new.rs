const SCALING_FACTOR: f32 = 1.3;

pub fn calculate(weight: f32, height_cm: f32) -> f32 {
    let height = height_cm / 100.0;
    SCALING_FACTOR * (weight / height.powf(2.5))
}
