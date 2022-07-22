use crate::{
    planetscale::{responses, PlanetScale},
    tracing::ResultTracingExt,
    Error,
};

pub trait AuthenticationQuery {
    fn verify_device(&self) -> Result<responses::DeviceVerification, Error>;
    fn get_tokens(&self, device_id: &str) -> Result<responses::OauthTokens, Error>;
    fn revoke_tokens(&self) -> Result<(), Error>;
}

impl<'c> AuthenticationQuery for PlanetScale<'c> {
    fn verify_device(&self) -> Result<responses::DeviceVerification, Error> {
        let result = self
            .agent
            .post(format!("{}/{}", &self.context.oauth_base_url, "authorize_device").as_str())
            .send_form(&[
                ("client_id", &self.context.oauth_client_id),
                ("scope", &self.context.oauth_scope),
            ]);

        match result {
            Ok(response) => {
                let data: responses::DeviceVerification = response
                    .into_json()
                    .expect_and_log("failed to deserialize device verification response");

                Ok(data)
            }
            Err(ureq::Error::Status(_, response)) => {
                let data: responses::AuthenticationError = response
                    .into_json()
                    .expect_and_log("failed to deserialize device verification error response");

                match data.error_code.as_str() {
                    "authorization_pending" => Err(Error::AuthorizationPending),
                    "slow_down" => Err(Error::TooFrequentPolling),
                    _ => Err(Error::Other),
                }
            }
            _ => Err(Error::Other),
        }
    }

    fn get_tokens(&self, device_id: &str) -> Result<responses::OauthTokens, Error> {
        let result = self
            .agent
            .post(format!("{}/{}", &self.context.oauth_base_url, "token").as_str())
            .send_form(&[
                ("grant_type", &self.context.oauth_grant_type),
                ("device_id", device_id),
                ("client_id", &self.context.oauth_client_id),
            ]);

        match result {
            Ok(response) => {
                let data: responses::OauthTokens = response
                    .into_json()
                    .expect_and_log("failed to deserialize oauth tokens response");

                Ok(data)
            }
            Err(ureq::Error::Status(_, response)) => {
                let data: responses::AuthenticationError = response
                    .into_json()
                    .expect_and_log("failed to deserialize oauth tokens error response");

                match data.error_code.as_str() {
                    "authorization_pending" => Err(Error::AuthorizationPending),
                    "slow_down" => Err(Error::TooFrequentPolling),
                    _ => Err(Error::Other),
                }
            }
            _ => Err(Error::Other),
        }
    }

    fn revoke_tokens(&self) -> Result<(), Error> {
        if self.context.access_token.is_empty() {
            return Err(Error::Unauthenticated);
        }

        let result = self
            .agent
            .post(format!("{}/{}", &self.context.oauth_base_url, "revoke").as_str())
            .send_form(&[
                ("client_id", &self.context.oauth_client_id),
                ("client_secret", &self.context.oauth_client_secret),
                ("token", &self.context.access_token),
            ]);

        match result {
            Ok(_) => Ok(()),
            Err(_) => Err(Error::Other),
        }
    }
}
