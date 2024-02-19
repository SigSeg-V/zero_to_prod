use std::time::Duration;
use async_std::prelude::FutureExt;
use tide::prelude::*;
use tide::{Request, Response};

pub fn setup_app() -> tide::Server<()> {
    let mut app = tide::new();
    app.at("/").get(greeting);
    app.at("/:name").get(greeting);
    app.at("/health_check").get(health_check);
    app
}

pub async fn run(timeout: Duration) -> tide::Result<()> {
    let mut app = setup_app();
    app
        .listen("127.0.0.1:8080").await?;
    Ok(())
}

pub async fn greeting(request: Request<()>) -> tide::Result<String> {
    let name = request
        .param("name")
        .unwrap_or("World");

    Ok(format!("Hello, {}!", name))
}

pub async fn health_check(request: Request<()>) -> tide::Result<> {
    Ok(Response::new(200))
}
