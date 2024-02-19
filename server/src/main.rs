use std::time::Duration;
use zero_to_prod::{greeting, health_check};

#[async_std::main]
async fn main() -> tide::Result<()> {
    zero_to_prod::run(Duration::new(2,0)).await
}
