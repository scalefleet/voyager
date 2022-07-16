use crate::{
    planetscale::{responses::OrganizationListResponse, PlanetScaleOrg},
    Configuration, Result,
};
use std::fs::File;
use std::io::Read;
use ureq::Agent;

pub struct PlanetScale {
    pub agent: Agent,
    pub base_url: String,
    pub bearer_token: String,
}

impl PlanetScale {
    pub fn new(agent: Agent, configuration: &Configuration) -> Result<Self> {
        let mut file = File::open(format!(
            "{}/access-token",
            &configuration.planetscale_directory
        ))?;
        let mut token = String::new();
        file.read_to_string(&mut token)?;

        Ok(PlanetScale {
            agent,
            base_url: format!(
                "{}/v{}",
                &configuration.planetscale_api, &configuration.planetscale_api_version
            ),
            bearer_token: format!("Bearer {token}"),
        })
    }

    pub fn org(&self) -> &dyn PlanetScaleOrg {
        self
    }
}

pub trait PlanetScaleDatabase {
    fn list(&self) -> Result<OrganizationListResponse>;
}

pub trait PlanetScaleBranch {
    fn list(&self) -> Result<OrganizationListResponse>;
}
