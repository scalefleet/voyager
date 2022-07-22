use crate::{
    planetscale::{responses, PlanetScale},
    tracing::ResultTracingExt,
};

pub trait DatabaseQuery {
    fn list(&self) -> Result<responses::DatabaseList, ureq::Error>;
}

impl<'c> DatabaseQuery for PlanetScale<'c> {
    fn list(&self) -> Result<responses::DatabaseList, ureq::Error> {
        let response: responses::DatabaseList = self
            .agent
            .get(
                format!(
                    "{}/organizations/{}/databases",
                    &self.context.api_base_url, &self.context.organization
                )
                .as_str(),
            )
            .set("Authorization", self.bearer_token().as_str())
            .call()
            .on_err_then_log("failed to fetch database list")?
            .into_json()
            .on_err_then_log("failed to deserialize database list response")?;

        Ok(response)
    }
}
