mod controllers;
mod errors;

use axum::Router;
use controllers::{authorization, fallback};
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use crate::controllers::hello_world;

#[tokio::main]
async fn main() {
    let main_route = Router::new()
        .merge(hello_world::HelloWorldController::new().router)
        .merge(fallback::FallbackController::new().router)
        .merge(authorization::AuthorizationController::new().router);
    axum::Server::bind(&SocketAddr::new(
        IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
        3000,
    ))
    .serve(main_route.into_make_service())
    .await
    .unwrap();
}
