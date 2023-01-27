#[allow(dead_code)]
pub enum ActivityLevel {
    Paralyzed,
    Immobile,
    Constricted,
    Little,
    Sedentary,
    SlightlyActive,
    LightlyActive,
    ModeratelyActive,
    VeryActive,
    ExtraActive,
}

#[allow(clippy::from_over_into)]
impl Into<f32> for ActivityLevel {
    fn into(self) -> f32 {
        match self {
            Self::Paralyzed => 1.0,
            Self::Immobile => 1.05,
            Self::Constricted => 1.1,
            Self::Little => 1.16,
            Self::Sedentary => 1.2,
            Self::SlightlyActive => 1.375,
            Self::LightlyActive => 1.425,
            Self::ModeratelyActive => 1.55,
            Self::VeryActive => 1.75,
            Self::ExtraActive => 1.9,
        }
    }
}
