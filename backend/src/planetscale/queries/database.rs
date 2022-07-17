use crate::planetscale::{
    responses::{Database, List},
    PlanetScale,
};

pub trait DatabaseQuery {
    fn list(&self) -> Result<List<Database>, ureq::Error>;
}

impl<'c> DatabaseQuery for PlanetScale<'c> {
    fn list(&self) -> Result<List<Database>, ureq::Error> {
        let response: List<Database> = self
            .agent
            .get(
                format!(
                    "{}/organizations/{}/databases",
                    &self.base_url(),
                    &self.configuration.planetscale_organization
                )
                .as_str(),
            )
            .set("Authorization", self.bearer_token().as_str())
            .call()?
            .into_json()?;

        Ok(response)
    }
}
