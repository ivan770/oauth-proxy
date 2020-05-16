use crate::{config::ClientConfig, error::ProxyError};
use oauth2::basic::BasicClient;
use oauth2::reqwest::http_client;
use oauth2::{
    RequestTokenError::ServerResponse, ResourceOwnerPassword, ResourceOwnerUsername, Scope,
    TokenResponse,
};
use serde::{Deserialize, Serialize};
use std::convert::TryInto;

const DEFAULT_SCOPE: &str = "*";

fn default_scope() -> String {
    DEFAULT_SCOPE.to_string()
}

#[derive(Serialize, Deserialize)]
pub struct AuthenticationParameters {
    pub username: String,
    pub password: String,
    #[serde(default = "default_scope")]
    pub scope: String,
}

impl AuthenticationParameters {
    pub fn authenticate(self, config: ClientConfig) -> Result<String, ProxyError> {
        let client: BasicClient = config.try_into()?;

        Ok(client
            .exchange_password(
                &ResourceOwnerUsername::new(self.username),
                &ResourceOwnerPassword::new(self.password),
            )
            .add_scope(Scope::new(self.scope))
            .request(http_client)
            .map_err(|error| match error {
                ServerResponse(val) => ProxyError::TokenObtainError(val.to_string()),
                _ => ProxyError::GenericConnectionError,
            })?
            .access_token()
            .secret()
            .to_owned())
    }
}
