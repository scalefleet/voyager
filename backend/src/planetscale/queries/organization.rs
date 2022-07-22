use crate::{
    planetscale::{responses, PlanetScale},
    tracing::ResultTracingExt,
};

pub trait OrganizationQuery {
    fn list(&self) -> Result<responses::OrganizationList, ureq::Error>;
}

impl<'c> OrganizationQuery for PlanetScale<'c> {
    fn list(&self) -> Result<responses::OrganizationList, ureq::Error> {
        let response: responses::OrganizationList = self
            .agent
            .get(format!("{}/organizations", &self.context.api_base_url).as_str())
            .set("Authorization", self.bearer_token().as_str())
            .call()
            .on_err_then_log("failed to fetch organization list")?
            .into_json()
            .on_err_then_log("failed to deserialize organization list response")?;

        Ok(response)
    }
}
