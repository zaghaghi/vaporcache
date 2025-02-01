use lambda_extension::{service_fn, tracing, Error, Extension};

mod events_extension;
mod server;

use events_extension::events_processor;
use server::VaporCacheServer;

#[tokio::main]
async fn main() -> Result<(), Error> {
    tracing::init_default_subscriber();

    let server = VaporCacheServer::start().await?;

    Extension::new()
        .with_events_processor(service_fn(events_processor))
        .run()
        .await?;

    server.stop().await
}
