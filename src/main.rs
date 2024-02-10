use std::path::Path;

use clap::Parser;

use diagnostick::config::config_file::load_configuration;
use diagnostick::config::config_file::ConfigurationFile;
use diagnostick::Diagnostick;
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

    let args: Args = Args::parse();

    let config_path: Option<&str> = match args.configuration_file.is_empty() {
        true => None,
        _ => Some(&args.configuration_file),
    };

    // TODO Manage verbose

    let config: ConfigurationFile = load_configuration(config_path);
    let diag: Diagnostick = Diagnostick::new();
    diag.save(Path::new(&(config.output))).unwrap();

    dbg!(config);
}
