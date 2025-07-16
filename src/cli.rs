use clap::Parser;

/// Simple program to demonstrate a boolean flag
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    /// Use the computer mode
    #[arg(long, short = 'c')]
    pub computer: bool,
}

pub fn get_args() -> Args {
    Args::parse()
}
