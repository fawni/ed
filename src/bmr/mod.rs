use std::fmt::{Display, Formatter};

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

impl Display for Sex {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match *self {
            Self::Male => write!(f, "Male"),
            Self::Female => write!(f, "Female"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! t {
        ($t:ident: $p:expr => $r:expr) => {
            #[test]
            fn $t() {
                assert_eq!(format!("{:.0}", $p), $r.to_string())
            }
        };
    }

    t!(harris_benedict: harris_benedict::calculate(46.0, 178.0, 19.0, Sex::Male) => 1461);
    t!(harris_benedict_revised: harris_benedict::revised::calculate(53.5, 188.0, 19.0, Sex::Female) => 1442);
    t!(mifflin_st_jeor: mifflin_st_jeor::calculate(62.0, 183.0, 22.0, Sex::Female) => 1493);
    t!(cunningham: cunningham::calculate(47.0, 10.0) => 1431);
    t!(katch_mcardle: katch_mcardle::calculate(51.5, 12.0) => 1349);
    t!(katch_mcardle_hyprid: katch_mcardle::hyprid::calculate(58.5, 13.5) => 1462);
}
