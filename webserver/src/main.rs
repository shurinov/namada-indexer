use anyhow::Context;
use clap::Parser;
use mimalloc::MiMalloc;
use webserver::app::ApplicationServer;
use webserver::config::AppConfig;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = AppConfig::parse();

    config.log.init();

    ApplicationServer::serve(config)
        .await
        .context("could not initialize application routes")?;

    Ok(())
}
