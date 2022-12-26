use teams_maker::server;


#[async_std::main]
async fn main() -> tide::Result<()> {
    server::run().await
}
