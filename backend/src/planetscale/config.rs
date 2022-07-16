use crate::{Error, ErrorKind, Result};
use std::fs::File;
use std::io::Read;

#[derive(serde::Deserialize)]
pub struct PlanetScaleConfig {
    /// User organization for the session. Default to username if user has no organization.
    pub organization: String,
}

impl PlanetScaleConfig {
    pub fn new(planetscale_dir: &str) -> Result<Self> {
        let mut file = File::open(format!("{planetscale_dir}/pscale.yml"))?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;

        let config: Self = if let Ok(config) = serde_yaml::from_str(content.as_str()) {
            config
        } else {
            return Err(Error::new(ErrorKind::Unauthenticated));
        };

        Ok(config)
    }
}
