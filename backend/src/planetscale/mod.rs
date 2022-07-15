mod responses;

use crate::{Error, ErrorKind, Result};
use async_trait::async_trait;
use reqwest::Client;
use responses::OrganizationListResponse;
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(serde::Deserialize)]
pub struct PlanetScaleConfig {
    pub org: String,
    pub token: String,
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

        Ok(Self {
            org,
            token: format!("Bearer {token}"),
        })
    }
}

pub struct PlanetScale {
    client: Client,
    domain: &'static str,
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
pub trait PlanetScaleOrg {
    async fn list(&self) -> Result<OrganizationListResponse>;
}

#[async_trait]
impl PlanetScaleOrg for PlanetScale {
    async fn list(&self) -> Result<OrganizationListResponse> {
        let request = self
            .client
            .get(format!("{}/v1/organizations", self.domain))
            .build()?;

        let response: OrganizationListResponse = self.client.execute(request).await?.json().await?;

        Ok(response)
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
