use std::env;

pub struct Context {
    pub access_token: String,
    pub api_base_url: String,
    pub oauth_base_url: String,
    pub oauth_client_id: String,
    pub oauth_client_secret: String,
    pub oauth_grant_type: String,
    pub oauth_scope: String,
    pub scoped_voyager_directory: String,
    pub global_planetscale_directory: String,
    pub organization: String,
    pub service_token: String,
}

impl Default for Context {
    fn default() -> Self {
        let current_directory = env::current_dir().unwrap().to_string_lossy().to_string();
        let base_directory = if let Ok(home_directory) = env::var("HOME") {
            home_directory
        } else {
            current_directory.clone()
        };

        Self {
            access_token: "".to_owned(),
            api_base_url: "https://api.planetscale.com/v1".to_owned(),
            oauth_base_url: "https://auth.planetscale.com/oauth".to_owned(),
            oauth_client_id: "wzzkYKOfRcxFAiMgDgfbhO9yIikNIlt9-yhosmvPBQA".to_owned(),
            oauth_client_secret: "eIDdgw21BYsovcrpC4iKZQ0o7ol9cN1LsSr8fuNyg5o".to_owned(),
            oauth_grant_type: "urn:ietf:params:oauth:grant-type:device_code".to_owned(),
            oauth_scope: "read_databases write_databases read_user read_organization".to_owned(),
            scoped_voyager_directory: format!("{}/voyager", &current_directory),
            global_planetscale_directory: format!("{}/.config/planetscale", &base_directory),
            organization: "".to_owned(),
            service_token: "".to_owned(),
        }
    }
}
