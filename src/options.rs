//! Options for the zic

#[derive(clap::ValueEnum, Default, Clone, Debug, Copy, PartialEq, Eq)]
pub enum BloatOption {
    #[default]
    Slim, 
    Fat,
}

pub struct BloatOptionErr;

impl core::fmt::Display for BloatOptionErr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("Invalid bloat flag was provided.")
    }
}

impl core::str::FromStr for BloatOption {
    type Err = BloatOptionErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "fat" => Ok(Self::Fat),
            "slim"=> Ok(Self::Slim),
            _=> Err(BloatOptionErr),
        }
    }
}

impl core::fmt::Display for BloatOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BloatOption::Slim=> f.write_str("slim"),
            BloatOption::Fat=> f.write_str("fat")
        }
    }
}