mod ipc_server;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    if let Err(error) = ipc_server::run().await {
        tracing::error!("IPC server stopped: {error}");
    }
}
