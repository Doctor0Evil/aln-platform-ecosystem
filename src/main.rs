use aln_platform_ecosystem::cli::Cli;
use clap::Parser;
use env_logger::Env;

fn main() {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();
    let cli = Cli::parse();
    cli.execute();
}
