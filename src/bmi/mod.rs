pub mod new;
pub mod prime;

pub fn calculate(weight: f32, height_cm: f32) -> f32 {
    let height = height_cm / 100.0;
    weight / height.powf(2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bmi() {
        assert_eq!(format!("{:.1}", calculate(55.0, 185.0)), "16.1");
    }

    #[test]
    fn new_bmi() {
        assert_eq!(format!("{:.1}", new::calculate(50.0, 172.0)), "16.8");
    }

    #[test]
    fn prime_bmi() {
        assert_eq!(format!("{:.1}", prime::calculate(39.0, 164.0)), "0.7");
    }
}
