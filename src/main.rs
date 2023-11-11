use clap::Parser;
use common::{info, setup_logs, LogLevel};
use eyre::*;
use raytracer::config::Config;
use raytracer::raytracer::render;
use std::fs;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[clap(
    version = "1.0",
    author = "Your Name",
    about = "Render scenes using a raytracer"
)]
struct Arguments {
    #[clap(help = "Sets the path to the configuration file")]
    config_file: PathBuf,

    #[clap(help = "Sets the path to the output file")]
    output_file: PathBuf,
}

fn main() -> Result<()> {
    setup_logs(LogLevel::Info)?;
    let args: Arguments = Arguments::parse();

    let json = fs::read(&args.config_file).expect("Unable to read config file.");
    let scene = serde_json::from_slice::<Config>(&json).expect("Unable to parse config json");

    info!(
        "Rendering {} -> {}",
        args.config_file.display(),
        args.output_file.display()
    );
    render(&args.output_file, scene)?;
    Ok(())
}
