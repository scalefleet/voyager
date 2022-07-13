use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};

#[derive(Deserialize)]
struct PscaleConfig {
    org: String,
}

#[derive(Deserialize)]
struct PscaleState {
    version: String,
}

pub struct PscaleMeta {
    org: String,
    version: String,
}

impl PscaleMeta {
    pub fn new(planetscale_dir: &str) -> io::Result<Self> {
        let mut file = File::open(format!("{planetscale_dir}/pscale.yml"))?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let config: PscaleConfig = serde_yaml::from_str(content.as_str()).unwrap();

        let mut file = File::open(format!("{planetscale_dir}/state.yml"))?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let state: PscaleState = serde_yaml::from_str(content.as_str()).unwrap();

        Ok(Self {
            org: config.org,
            version: state.version,
        })
    }
}

use std::process::Command;

pub struct Pscale(Command);

impl Pscale {
    pub fn new() -> Self {
        Pscale(Command::new("pscale"))
    }

    pub fn branch(&mut self) -> PscaleBranch {
        PscaleBranch(self.0.arg("branch"))
    }

    pub fn database(&mut self) -> PscaleDatabase {
        PscaleDatabase(self.0.arg("database"))
    }

    pub fn data_imports(&mut self) -> PscaleDataImports {
        PscaleDataImports(self.0.arg("data-imports"))
    }

    pub fn deploy_request(&mut self) -> PscaleDeployRequest {
        PscaleDeployRequest(self.0.arg("deploy-request"))
    }

    pub fn org(&mut self) -> PscaleOrg {
        PscaleOrg(self.0.arg("org"))
    }
}

impl Default for Pscale {
    fn default() -> Self {
        Self::new()
    }
}

pub struct PscaleBranch<'a>(&'a mut Command);

pub struct PscaleDatabase<'a>(&'a mut Command);

pub struct PscaleDataImports<'a>(&'a mut Command);

pub struct PscaleDeployRequest<'a>(&'a mut Command);

pub struct PscaleOrg<'a>(&'a mut Command);
