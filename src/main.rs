use std::ops::RangeInclusive;

use clap::Parser;

/// Timezone compiler in Rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Set compiler output flag for backword compatability data
    #[arg(short, default_value_t=BloatOption::Slim)]
    bloat: BloatOption,
    
    /// Create a directory to create the files in
    #[arg(short)]
    directory: Option<String>,

    /// Use timezone as localtime
    #[arg(short='l', default_value_t=false)]
    localtime: bool,

    /// Read the leapsecond information
    #[arg(short='L', default_value_t=false)]
    leapseconds: bool,

// check whether supported!
    /// Use timezones posix rules
    #[arg(short='p', default_value_t=false)]
    posixrules: bool,

    /// Timestamp range for output files
    #[arg(short='r')]
    range: Option<String>,

    /// Filename alias for a link
    #[arg(short='t')]
    localtimelink: Option<String>,

    /// Set verbosity of compiler
    #[arg(short, default_value_t=false)]
    verbose: bool,

    filenames: Vec<String>,
}

#[derive(clap::ValueEnum, Clone, Debug, Copy, PartialEq, Eq)]
pub enum BloatOption {
    Slim, 
    Fat,
}

pub struct BloatOptionErr;

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


fn main() {
    let args = Args::parse();

    assert!(true);

    assert_eq!(args.bloat, BloatOption::Slim);
    assert_eq!(&args.bloat.to_string(), "slim");

}