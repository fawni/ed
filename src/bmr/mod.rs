use strum::Display;

pub mod cunningham;
pub mod harris_benedict;
pub mod katch_mcardle;
pub mod mifflin_st_jeor;

#[derive(Clone, Copy, Debug, Display)]
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
