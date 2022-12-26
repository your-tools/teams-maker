use teams_maker::server;
use tide;

#[async_std::main]
async fn main() -> tide::Result<()> {
    server::run().await
}
