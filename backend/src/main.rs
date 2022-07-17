use backend::{
    planetscale::{PlanetScale, PscaleConfiguration},
    tracing::{tracing_subscribe, ResultTracingExt},
    Configuration,
};
use clap::Parser;
use std::fs::File;
use std::io::Read;
use ureq::AgentBuilder;

fn main() {
    tracing_subscribe().expect_and_log("tracing subscription failed");

    let cli = Cli::parse();

    let mut configuration = Configuration::default();

    let pscale_configuration =
        PscaleConfiguration::new(&configuration.planetscale_directory).unwrap_or_log();

    configuration.planetscale_organization = pscale_configuration.organization;

    if let Some(service_token) = cli.service_token {
        configuration.planetscale_access_token = Some(service_token);
    } else {
        let file = File::open(format!(
            "{}/access-token",
            &configuration.planetscale_directory
        ));

        if let Ok(mut file) = file {
            let mut access_token = String::new();

            file.read_to_string(&mut access_token)
                .expect_and_log("failed to read access token");
            configuration.planetscale_access_token = Some(access_token);
        }
    }

    let agent = AgentBuilder::new().https_only(true).build();

    let planetscale = PlanetScale::new(agent, &configuration).unwrap_or_log();

    match &cli.command {
        Some(Command::Dashboard) => println!("aye"),
        None => {
            for organization in planetscale.org().list().unwrap_or_log().data {
                println!("{}", &organization.name);
            }
        }
    }
}

/// PlanetScale GUI tool that simplify your local development workflow.
#[derive(clap::Parser)]
#[clap(about, long_about=None, version)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,

    /// PlanetScale CLI config directory. Defaults to $HOME/.config/planetscale.
    #[clap(long, value_parser)]
    config_dir: Option<String>,

    /// Use custom service token instead of the access token received from `pscale auth login`.
    #[clap(long, value_parser)]
    service_token: Option<String>,
}

#[derive(clap::Subcommand)]
enum Command {
    /// Start the Voyager web dashboard.
    Dashboard,
}
