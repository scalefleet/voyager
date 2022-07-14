mod responses;

use crate::{Error, ErrorKind, Result};
use async_trait::async_trait;
use reqwest::Client;
use responses::OrganizationListResponse;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
pub struct PlanetScaleConfig {
    org: String,
    token: String,
}

impl PlanetScaleConfig {
    pub fn new(planetscale_dir: &str) -> Result<Self> {
        let mut file = File::open(format!("{planetscale_dir}/pscale.yml"))?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let state: HashMap<String, String> =
            if let Ok(state) = serde_yaml::from_str(content.as_str()) {
                state
            } else {
                return Err(Error::new(ErrorKind::Unauthenticated));
            };

        let org: String = if let Some(org) = state.get(&String::from("org")) {
            org.to_owned()
        } else {
            return Err(Error::new(ErrorKind::Unauthenticated));
        };

        let mut file = File::open(format!("{planetscale_dir}/access-token"))?;
        let mut token = String::new();
        file.read_to_string(&mut token)?;

        Ok(Self { org, token })
    }
}

pub struct PlanetScale<'a> {
    client: &'a Client,
    domain: &'static str,
}

impl<'a> PlanetScale<'a> {
    pub fn new(client: &'a Client) -> Self {
        PlanetScale {
            client,
            domain: "https://api.planetscale.com",
        }
    }

    pub fn org(&'a self) -> &impl PlanetScaleOrg<'a> {
        self
    }
}

#[async_trait]
pub trait PlanetScaleOrg<'a> {
    async fn list(&'a self) -> Result<OrganizationListResponse>;
}

#[async_trait]
impl<'a> PlanetScaleOrg<'a> for PlanetScale<'a> {
    async fn list(&'a self) -> Result<OrganizationListResponse> {
        let request = self
            .client
            .get(format!("{}/v1/organizations", self.domain))
            .build()?;

        let response: OrganizationListResponse = self.client.execute(request).await?.json().await?;

        Ok(response)
    }
}

#[async_trait]
pub trait PlanetScaleDatabase<'a> {
    async fn list(&'a self) -> Result<OrganizationListResponse>;
}

#[async_trait]
pub trait PlanetScaleBranch<'a> {
    async fn list(&'a self) -> Result<OrganizationListResponse>;
}
