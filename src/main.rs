//! src/main.rs
use anyhow::Result;
use memdb_demo::configuration;
use memdb_demo::startup::Application;
use tracing;

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let configuration = configuration::get_configuration()?;
    let application = Application::build(configuration).await?;

    tracing::info!("{}", "Creating MemoryDB Cluster...");

    if let Some(cluster) = application.response.cluster() {
        tracing::info!("Name: {}", cluster.name().unwrap_or("Unknown"));
        tracing::info!("Engine: {}", cluster.engine().unwrap_or("Unknown"));
        tracing::info!("ARN: {}", cluster.arn().unwrap_or("Unknown"));
    } else {
        tracing::info!("No cluster information returned");
    }

    Ok(())
}
