use std::ops::Deref;

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use serde::Deserialize;

pub struct HelloWorldController {
    pub router: Router,
}

#[derive(Debug, Deserialize)]
pub struct HelloParams {
    name: Option<String>,
    first_name: Option<String>,
}

pub struct Wrapper {
    value: HelloParams,
}

impl Deref for Wrapper {
    type Target = HelloParams;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl HelloWorldController {
    pub fn new() -> Self {
        HelloWorldController {
            router: Router::new().route("/hello-world", get(Self::get_hello_world)),
        }
    }

    async fn get_hello_world(params: Query<HelloParams>) -> impl IntoResponse {
        let name = match params.name {
            Some(ref na) => na,
            None => "",
        };

        Html(format!("Hello <strong>World!!! {:?}</strong>", name))
    }
}
