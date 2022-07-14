use backend::{Error, ErrorKind, Result, planetscale::PlanetScaleConfig};

#[tokio::main]
async fn main() -> Result<()> {
    let config = if let Ok(config) = PlanetScaleConfig::new("/home/izznatsir/.config/planetscale") {
		config
	} else {
		return Err(Error::new(ErrorKind::Unauthenticated));
	};

	Ok(())
}

/// PlanetScale GUI tool that simplify your local development workflow.
#[derive(clap::Parser)]
#[clap(about, long_about=None, version)]
struct Cli {
    #[clap(subcommand)]
    command: Command,

    #[clap(long, value_parser)]
    service_token: Option<String>,
}

#[derive(clap::Subcommand)]
enum Command {
    Start,
}
