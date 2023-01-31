pub mod new;
pub mod prime;

pub fn calculate(weight: f32, height_cm: f32) -> f32 {
    let height = height_cm / 100.0;
    weight / height.powf(2.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! t {
        ($t:ident: $p:expr => $r:expr) => {
            #[test]
            fn $t() {
                assert_eq!(format!("{:.1}", $p), $r.to_string())
            }
        };
    }

    t!(bmi: calculate(55.0, 185.0) => 16.1);
    t!(new_bmi: new::calculate(50.0, 172.0) => 16.8);
    t!(prime_bmi: prime::calculate(39.0, 164.0) => 0.7);
}
