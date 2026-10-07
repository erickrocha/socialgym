use business::commons::chat_hub::ChatHub;
use std::sync::Arc;

/// The REST and the gRPC servers share one process (and one chat hub, so a message sent over one
/// transport reaches clients connected over the other). They are independent: one failing to
/// start or stopping later must not take the other down (SYS-C008-010), so each failure is logged
/// and the process lives while one of them serves.
#[tokio::main]
async fn main() {
    if let Err(error) = run().await {
        println!("Error: {error}");
    }
}

async fn run() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    dotenvy::dotenv().ok();

    let database = Arc::new(business::commons::db_pool::connect().await.map_err(|error| anyhow::anyhow!(error))?);
    let chat_hub = ChatHub::new();

    log::info!("Starting server...");
    let grpc = {
        let (database, chat_hub) = (database.clone(), chat_hub.clone());
        tokio::spawn(async move {
            if let Err(error) = integration::serve(database, chat_hub).await {
                log::error!("gRPC server stopped: {error}");
            }
        })
    };
    let rest = application::serve(database, chat_hub).await;
    if let Err(error) = &rest {
        log::error!("REST server stopped: {error}");
    }
    // Reached when REST ended: stay up for as long as gRPC serves, and report a total failure.
    let _ = grpc.await;
    rest
}
