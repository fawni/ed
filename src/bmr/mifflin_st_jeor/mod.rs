use crate::Sex;

pub fn calculate(weight: f32, height: f32, age: f32, sex: Sex) -> f32 {
    match sex {
        Sex::Male => (10.0 * weight) + (6.25 * height) - (5.0 * age) + 5.0,
        Sex::Female => (10.0 * weight) + (6.25 * height) - (5.0 * age) - 161.0,
    }
}
