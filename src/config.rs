use crate::{error::ProxyError, get};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, TokenUrl};
use std::convert::TryFrom;

pub struct ClientConfig {
    pub client_id: String,
    pub client_secret: String,
    pub url: String,
}

impl ClientConfig {
    pub fn from_env() -> Self {
        ClientConfig {
            client_id: get!("CLIENT_ID"),
            client_secret: get!("CLIENT_SECRET"),
            url: get!("SERVER_URL"),
        }
    }
}

impl TryFrom<ClientConfig> for BasicClient {
    type Error = ProxyError;

    fn try_from(config: ClientConfig) -> Result<Self, Self::Error> {
        Ok(BasicClient::new(
            ClientId::new(config.client_id),
            Some(ClientSecret::new(config.client_secret)),
            AuthUrl::new(config.url.clone()).map_err(|_| ProxyError::UrlParseError)?,
            Some(TokenUrl::new(config.url).map_err(|_| ProxyError::UrlParseError)?),
        ))
    }
}
