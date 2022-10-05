use sploy::error::Error;

#[tokio::main]
async fn main() -> Result<(), Error> {
    sploy::run().await
}