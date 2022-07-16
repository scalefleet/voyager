use crate::{
    planetscale::{responses::OrganizationListResponse, PlanetScaleConfig, PlanetScaleOrg},
    Result,
};
use ureq::Agent;

pub struct PlanetScale {
    pub agent: Agent,
    pub base_url: String,
    pub bearer_token: String,
}

impl PlanetScale {
    pub fn new(agent: Agent, config: &PlanetScaleConfig) -> Self {
        PlanetScale {
            agent,
            base_url: "https://api.planetscale.com".to_owned(),
            bearer_token: config.bearer_token.to_owned(),
        }
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
