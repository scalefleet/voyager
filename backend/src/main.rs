use backend::{
    planetscale::PlanetScaleConfig,
    tracing::{tracing_subscribe, ResultTracingExt},
    Result,
};
use std::env;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscribe().expect("Tracing subscription failed.");

    let home_dir = env::var("HOME").unwrap();
    let pscale_config =
        PlanetScaleConfig::new(format!("{}/.config/planetscale", home_dir).as_str()).maybe_log()?;

    println!("Organization: {}", pscale_config.org);
    println!("Access token: {}", pscale_config.token);

    Ok(())
}

/// PlanetScale GUI tool that simplify your local development workflow.
#[derive(clap::Parser)]
#[clap(about, long_about=None, version)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    /// PlanetScale CLI config directory. Defaults to $HOME/.config/planetscale.
    #[clap(long, value_parser)]
    config_dir: Option<String>,

    /// Use custom service token instead of the default token generated by `pscale auth login`.
    #[clap(long, value_parser)]
    service_token: Option<String>,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Start the Voyager server on local network, this is the default command.
    Start,
}
