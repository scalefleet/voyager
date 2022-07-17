use crate::planetscale::{
    responses::{List, Organization},
    PlanetScale,
};

pub trait OrganizationQuery {
    fn list(&self) -> Result<List<Organization>, ureq::Error>;
}

impl<'c> OrganizationQuery for PlanetScale<'c> {
    fn list(&self) -> Result<List<Organization>, ureq::Error> {
        let response: List<Organization> = self
            .agent
            .get(format!("{}/organizations", &self.base_url()).as_str())
            .set("Authorization", self.bearer_token().as_str())
            .call()?
            .into_json()?;

        Ok(response)
    }
}
