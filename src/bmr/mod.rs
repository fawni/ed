pub mod cunningham;
pub mod harris_benedict;
pub mod katch_mcardle;
pub mod mifflin_st_jeor;

#[derive(Clone, Copy, Debug)]
pub enum Sex {
    Male,
    Female,
}

impl From<usize> for Sex {
    fn from(value: usize) -> Self {
        match value {
            0 => Self::Male,
            1 => Self::Female,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Sex {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Self::Male => write!(f, "Male"),
            Self::Female => write!(f, "Female"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn harris_benedict() {
        assert_eq!(
            format!(
                "{:.0}",
                harris_benedict::calculate(46.0, 178.0, 19.0, Sex::Male)
            ),
            "1461"
        );

        assert_eq!(
            format!(
                "{:.0}",
                harris_benedict::revised::calculate(53.5, 188.0, 19.0, Sex::Female)
            ),
            "1442"
        );
    }

    #[test]
    fn mifflin_st_jeor() {
        assert_eq!(
            format!(
                "{:.0}",
                mifflin_st_jeor::calculate(62.0, 183.0, 22.0, Sex::Female)
            ),
            "1493"
        );
    }

    #[test]
    fn cunningham() {
        assert_eq!(format!("{:.0}", cunningham::calculate(47.0, 10.0)), "1431");
    }

    #[test]
    fn katch_mcardle() {
        assert_eq!(
            format!("{:.0}", katch_mcardle::calculate(51.5, 12.0)),
            "1349"
        );

        assert_eq!(
            format!("{:.0}", katch_mcardle::hyprid::calculate(58.5, 13.5)),
            "1462"
        );
    }
}
