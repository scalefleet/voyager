use crate::{
    planetscale::{responses::OrganizationListResponse, PlanetScale},
    Result,
};
use async_trait::async_trait;

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
