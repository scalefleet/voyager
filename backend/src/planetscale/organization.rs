use crate::{
    planetscale::{responses::OrganizationListResponse, PlanetScale},
    Result,
};

pub trait PlanetScaleOrg {
    fn list(&self) -> Result<OrganizationListResponse>;
}

impl PlanetScaleOrg for PlanetScale {
    fn list(&self) -> Result<OrganizationListResponse> {
        let response: OrganizationListResponse = self
            .agent
            .get(format!("{}/v1/organizations", &self.base_url).as_str())
            .set("Authorization", self.bearer_token.as_str())
            .call()?
            .into_json()?;

        Ok(response)
    }
}
