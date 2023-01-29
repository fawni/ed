pub mod revised;

use crate::Sex;

pub fn calculate(weight: f32, height: f32, age: f32, sex: Sex) -> f32 {
    match sex {
        Sex::Male => (13.7516 * weight) + (5.0033 * height) - (6.755 * age) + 66.473,
        Sex::Female => (9.5634 * weight) + (1.8496 * height) - (4.6756 * age) + 655.0955,
    }
}
