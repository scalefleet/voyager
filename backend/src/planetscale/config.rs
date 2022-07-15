use crate::{Error, ErrorKind, Result};
use std::collections::HashMap;
use std::fs::File;
use std::io::Read;

#[derive(serde::Deserialize)]
pub struct PlanetScaleConfig {
    /// User organization for the session. Default to username if user has no organization.
    pub organization: String,
    /// Either access token or service token prefixed with "Bearer ".
    pub bearer_token: String,
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

        let organization: String = if let Some(organization) = state.get(&String::from("org")) {
            organization.to_owned()
        } else {
            return Err(Error::new(ErrorKind::Unauthenticated));
        };

        let mut file = File::open(format!("{planetscale_dir}/access-token"))?;
        let mut token = String::new();
        file.read_to_string(&mut token)?;

        Ok(Self {
            organization,
            bearer_token: format!("Bearer {token}"),
        })
    }
}
