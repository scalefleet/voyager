use astra::{Body, Response, Server};
use backend::{
    planetscale::{PlanetScale, PscaleConfiguration},
    tracing::{tracing_subscribe, ResultTracingExt},
    Configuration,
};
use clap::Parser;
use colored::Colorize;
use std::fs::File;
use std::io::Read;
use ureq::AgentBuilder;

fn main() {
    tracing_subscribe().expect_and_log("tracing subscription failed");

    let cli = Cli::parse();

    println!();
    println!("{} {}", "Voyager".blue(), "v.0.0.0".green());
    println!();

    tracing::info!("resolving configurations");
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

    let _planetscale = PlanetScale::new(agent, &configuration).unwrap_or_log();

    tracing::info!("configurations resolved");

    tracing::info!("starting Voyager server at http://localhost:3000");

    Server::bind("localhost:3000")
        .serve(|_| Response::new(Body::new("ok")))
        .expect_and_log("failed to start server");
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
