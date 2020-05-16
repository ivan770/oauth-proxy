mod config;
mod error;
mod macros;
mod params;

use config::ClientConfig;
use dotenv::dotenv;
use params::AuthenticationParameters;
use std::{env::var, net::SocketAddr};
use tide::{
    http::{headers::HeaderValue, StatusCode},
    security::{CorsMiddleware, Origin},
    Error, Request, Server,
};

fn get_host() -> SocketAddr {
    var("HOST")
        .expect("HOST variable not found in .env")
        .parse()
        .expect("Invalid HOST value in .env")
}

#[async_std::main]
async fn main() {
    dotenv().expect("Unable to load .env file");

    let mut tide = Server::new();

    tide.middleware(
        CorsMiddleware::new()
            .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
            .allow_origin(Origin::from(get!("FRONTEND_URL"))),
    );

    tide.at("/").get(|_| async { Ok("Security!") });

    tide.at("/login")
        .post(|mut request: Request<()>| async move {
            let params = request
                .body_json::<AuthenticationParameters>()
                .await
                .map_err(|_| Error::from_str(StatusCode::BadRequest, "Invalid request body"))?;

            match params.authenticate(ClientConfig::from_env()) {
                Ok(token) => Ok(token),
                Err(error) => Err(error.into()),
            }
        });

    tide.listen(get_host()).await.unwrap();
}
