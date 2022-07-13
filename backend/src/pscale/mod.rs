mod responses;

use crate::pscale::responses::OrganizationListResponse;
use async_trait::async_trait;
use reqwest::{Client, Error};
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

pub struct Pscale<'a> {
    client: &'a Client,
    domain: &'static str,
}

impl<'a> Pscale<'a> {
    pub fn new(client: &'a Client) -> Self {
        Pscale {
            client,
            domain: "https://api.planetscale.com",
        }
    }

    pub fn org(&'a self) -> &impl PscaleOrg<'a> {
        self
    }
}

#[async_trait]
pub trait PscaleOrg<'a> {
    async fn list(&'a self) -> Result<OrganizationListResponse, Error>;
}

#[async_trait]
impl<'a> PscaleOrg<'a> for Pscale<'a> {
    async fn list(&'a self) -> Result<OrganizationListResponse, Error> {
        let request = self
            .client
            .get(format!("{}/v1/organizations", self.domain))
            .build()?;

        let response: OrganizationListResponse = self.client.execute(request).await?.json().await?;

        Ok(response)
    }
}

#[async_trait]
pub trait PscaleDatabase<'a> {
    async fn list(&'a self) -> Result<OrganizationListResponse, Error>;
}

#[async_trait]
pub trait PscaleBranch<'a> {
    async fn list(&'a self) -> Result<OrganizationListResponse, Error>;
}
