use crate::{planetscale::OrganizationQuery, Configuration, Result};
use ureq::Agent;

pub struct PlanetScale<'c> {
    pub agent: Agent,
    pub configuration: &'c Configuration,
}

impl<'c> PlanetScale<'c> {
    pub fn new(agent: Agent, configuration: &'c Configuration) -> Result<Self> {
        Ok(PlanetScale {
            agent,
            configuration,
        })
    }

    pub fn base_url(&self) -> String {
        format!(
            "{}/v{}",
            &self.configuration.planetscale_api, &self.configuration.planetscale_api_version
        )
    }

    pub fn bearer_token(&self) -> String {
        if let Some(access_token) = &self.configuration.planetscale_access_token {
            format!("Bearer {access_token}")
        } else if let Some(service_token) = &self.configuration.planetscale_service_token {
            format!("Bearer {service_token}")
        } else {
            String::new()
        }
    }

    pub fn org(&'c self) -> &'c dyn OrganizationQuery {
        self
    }
}
