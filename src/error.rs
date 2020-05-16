use tide::{Error, StatusCode};

const URL_PARSE_ERROR: &str = "Unable to parse SERVER_URL from .env";
const GENERIC_CONNECTION_ERROR: &str = "Generic server connection error";

pub enum ProxyError {
    UrlParseError,
    TokenObtainError(String),
    GenericConnectionError,
}

impl From<ProxyError> for Error {
    fn from(proxy_error: ProxyError) -> Self {
        match proxy_error {
            ProxyError::UrlParseError => {
                Error::from_str(StatusCode::InternalServerError, URL_PARSE_ERROR)
            }
            ProxyError::TokenObtainError(msg) => Error::from_str(StatusCode::Forbidden, msg),
            ProxyError::GenericConnectionError => {
                Error::from_str(StatusCode::ServiceUnavailable, GENERIC_CONNECTION_ERROR)
            }
        }
    }
}
