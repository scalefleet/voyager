use std::env;

pub struct Configuration {
    pub planetscale_access_token: Option<String>,
    pub planetscale_api: String,
    pub planetscale_api_version: usize,
    pub planetscale_directory: String,
    pub planetscale_organization: String,
    pub planetscale_service_token: Option<String>,
}

impl Default for Configuration {
    fn default() -> Self {
        let base_dir = if let Ok(home_dir) = env::var("HOME") {
            home_dir
        } else {
            env::current_dir().unwrap().to_string_lossy().to_string()
        };

        Self {
            planetscale_access_token: None,
            planetscale_api: "https://api.planetscale.com".to_owned(),
            planetscale_api_version: 1,
            planetscale_directory: format!("{}/.config/planetscale", &base_dir),
            planetscale_organization: "".to_owned(),
            planetscale_service_token: None,
        }
    }
}
