use tide::prelude::*;
use tide::Request;

#[async_std::main]
async fn main() -> tide::Result<()> {
    let mut app = tide::new();
    app.at("/").get(greeting);
    app.at("/:name").get(greeting);
    app.listen("127.0.0.1:8080").await?;
    Ok(())
}

async fn greeting(request: Request<()>) -> tide::Result<String> {
    let name = request
        .param("name")
        .unwrap_or("World");

    Ok(format!("Hello, {}!", name))
}
