use crate::config::Config;
use honeywell_tcc::TccApi;
use std::time::Duration;
use tracing::info;
use tracing_subscriber::fmt::layer;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::EnvFilter;

mod config;

#[tokio::main]
async fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var(
            "RUST_LOG",
            format!("{}=INFO", env!("CARGO_PKG_NAME").replace("-", "_")),
        );
    }

    tracing_subscriber::registry()
        .with(EnvFilter::from_default_env())
        .with(layer().compact())
        .init();

    info!("Starting");
    info!("Reading config");

    let config = Config::from_env()?;
    let tcc = TccApi::new_with_login(&config.tcc_email, &config.tcc_password).await?;

    loop {
        info!("Fetching locations");
        let locations = tcc.get_locations().await?;

        info!("Updating zone temperatures if necessary");
        let mut zones_updated = 0;

        for location in locations {
            for zone in location.zones {
                if zone.target_heat_temperature > config.max_temperature {
                    zones_updated += 1;
                    tcc.set_zone_temperature(zone.id.clone(), config.max_temperature)
                        .await?;
                }
            }
        }

        info!("Done. Updated {zones_updated} zones.");
        tokio::time::sleep(Duration::from_secs(config.interval_sec)).await;
    }
}
