pub fn calculate(weight: f32, bodyfat_percent: f32) -> f32 {
    let bodyfat = bodyfat_percent / 100.0;
    (370.0 * (1.0 - bodyfat)) + (21.6 * (weight * (1.0 - bodyfat))) + (6.17 * (weight * bodyfat))
}
