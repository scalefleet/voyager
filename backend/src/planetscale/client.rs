use crate::{
    planetscale::{organization::PlanetScaleOrg, responses::OrganizationListResponse},
    Result,
};
use async_trait::async_trait;
use reqwest::Client;

pub struct PlanetScale {
    pub client: Client,
    pub domain: &'static str,
}

impl PlanetScale {
    pub fn new(client: Client) -> Self {
        PlanetScale {
            client,
            domain: "https://api.planetscale.com",
        }
    }

    pub fn org(&self) -> &impl PlanetScaleOrg {
        self
    }
}

#[async_trait]
pub trait PlanetScaleDatabase {
    async fn list(&self) -> Result<OrganizationListResponse>;
}

#[async_trait]
pub trait PlanetScaleBranch {
    async fn list(&self) -> Result<OrganizationListResponse>;
}
