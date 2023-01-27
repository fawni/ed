pub mod new;
pub mod prime;

pub fn calculate(weight: f32, height_cm: f32) -> f32 {
    let height = height_cm / 100.0;
    weight / height.powf(2.0)
}
