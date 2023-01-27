// american "prime"
// const PRIME: f32 = 25.0;

// actual prime
const PRIME: f32 = 22.0;

pub fn calculate(weight: f32, height_cm: f32) -> f32 {
    let height = height_cm / 100.0;
    (weight / height.powf(2.0)) / PRIME
}
