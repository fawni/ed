use std::fmt::{Display, Formatter};

#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
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
    ExtremelyActive,
}

impl Display for ActivityLevel {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            Self::Paralyzed => write!(f, "Completely Paralyzed (1.0)",),
            Self::Immobile => write!(f, "Immobile (1.05)"),
            Self::Constricted => write!(f, "Constricted Lifestyle (1.1)"),
            Self::Little => write!(f, "No Exercise, Little Movement (1.16)"),
            Self::Sedentary => write!(f, "Sedentary Lifestyle (1.2)"),
            Self::SlightlyActive => write!(f, "Slightly Active (1.375)"),
            Self::LightlyActive => write!(f, "Lightly Active (1.425)"),
            Self::ModeratelyActive => write!(f, "Moderately Active (1.55)"),
            Self::VeryActive => write!(f, "Very Active (1.75)"),
            Self::ExtremelyActive => write!(f, "Extremely Active (1.9)"),
        }
    }
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
            Self::ExtremelyActive => 1.9,
        }
    }
}

impl From<usize> for ActivityLevel {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Paralyzed,
            1 => Self::Immobile,
            2 => Self::Constricted,
            3 => Self::Little,
            4 => Self::Sedentary,
            5 => Self::SlightlyActive,
            6 => Self::LightlyActive,
            7 => Self::ModeratelyActive,
            8 => Self::VeryActive,
            9 => Self::ExtremelyActive,
            _ => unreachable!(),
        }
    }
}

pub fn calculate(bmr: f32, activity_level: ActivityLevel) -> f32 {
    let activity: f32 = activity_level.into();
    bmr * activity
}

#[cfg(test)]
mod tests {
    use crate::bmr;

    use super::*;

    #[test]
    fn tdee() {
        assert_eq!(
            format!(
                "{:.0}",
                calculate(
                    bmr::katch_mcardle::calculate(47.0, 10.0),
                    ActivityLevel::Sedentary,
                )
            ),
            "1540"
        );
    }
}
