use clap::Parser;

/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    bloat: Option<String>,
    
    #[arg(short, long)]
    directory: Option<String>,

    #[arg(short='l', long)]
    timezone: Option<String>,

    #[arg(short='L', long)]
    leapsecondfilename: Option<String>,

    

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.bloat);
    }
}