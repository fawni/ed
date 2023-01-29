use crate::Sex;

pub fn calculate(weight: f32, height: f32, age: f32, sex: Sex) -> f32 {
    match sex {
        Sex::Male => (13.397 * weight) + (4.799 * height) - (5.677 * age) + 88.362,
        Sex::Female => (9.247 * weight) + (3.098 * height) - (4.330 * age) + 447.593,
    }
}
