use std::{path::PathBuf, str::FromStr};

use clap::Parser;

use zic_rs::{config::Config, options::BloatOption, ZoneInfoCompiler};

const ZONEINFO_DIR: &str = "/usr/share/zoneinfo";

/// Timezone compiler in Rust
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Set compiler output flag for backword compatability data
    #[arg(short, default_value_t=BloatOption::Slim)]
    bloat: BloatOption,
    
    /// Create a directory to create the files in
    #[arg(short)]
    directory: Option<PathBuf>,

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

impl From<Args> for Config {
    fn from(value: Args) -> Self {
        let directory = value.directory.unwrap_or(PathBuf::from_str(ZONEINFO_DIR).expect("const is path."));
        Config::new()
            .set_bloat(value.bloat)
            .set_verbose(value.verbose)
            .set_leap_seconds_flag(value.leapseconds)
            .set_localtime_flag(value.localtime)
            .set_posixrules_flag(value.posixrules)
            .directory(directory)
    }
}


fn main() {
    let args = Args::parse();

    let filenames = args.filenames.clone();
    let compiler = ZoneInfoCompiler::new(args.into());

    println!("Compiler with config:");
    println!("{compiler:#?}");
    println!("============");
    println!("TODO: Compiler the below.\n{:#?}", filenames);
    println!("============");
}