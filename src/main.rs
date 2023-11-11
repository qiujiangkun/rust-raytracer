use clap::Parser;
use raytracer::config::Config;
use raytracer::raytracer::render;
use std::fs;

#[derive(Parser, Debug)]
#[clap(
    version = "1.0",
    author = "Your Name",
    about = "Render scenes using a raytracer"
)]
struct Opts {
    #[clap(help = "Sets the path to the configuration file")]
    config_file: String,

    #[clap(help = "Sets the path to the output file")]
    output_file: String,
}

fn main() {
    let opts: Opts = Opts::parse();

    let json = fs::read(&opts.config_file).expect("Unable to read config file.");
    let scene = serde_json::from_slice::<Config>(&json).expect("Unable to parse config json");

    println!("\nRendering {}", opts.output_file);
    render(&opts.output_file, scene);
}
