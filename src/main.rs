use clap::Parser;

use diagnostick::run_checks;
use diagnostick::BANNER;

/// Configuration Checker
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(short, long, default_value = "")]
    configuration_file: String,
    #[arg(short, long, action)]
    verbose: bool,
}

fn main() {
    print!("{}", BANNER);

    run_checks();
}
