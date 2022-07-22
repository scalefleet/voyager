use crate::{planetscale::OrganizationQuery, Context, Result};
use ureq::Agent;

pub struct PlanetScale<'c> {
    pub agent: Agent,
    pub context: &'c Context,
}

impl<'c> PlanetScale<'c> {
    pub fn new(agent: Agent, context: &'c Context) -> Result<Self> {
        Ok(PlanetScale { agent, context })
    }

    pub fn bearer_token(&self) -> String {
        if !self.context.access_token.is_empty() {
            format!("Bearer {}", &self.context.access_token)
        } else {
            String::new()
        }
    }

    pub fn organization(&'c self) -> &'c dyn OrganizationQuery {
        self
    }
}
