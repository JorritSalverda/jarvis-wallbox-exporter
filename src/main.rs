mod wallbox_api_client;
mod model;

use wallbox_api_client::{WallboxApiClient, WallboxApiClientConfig};
use jarvis_lib::bigquery_client::{BigqueryClient, BigqueryClientConfig};
use jarvis_lib::config_client::{ConfigClient, ConfigClientConfig};
use jarvis_lib::exporter_service::{ExporterService, ExporterServiceConfig};
use jarvis_lib::state_client::{StateClient, StateClientConfig};

#[tokio::main]
pub async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let wallbox_api_client_config = WallboxApiClientConfig::from_env()?;
    let wallbox_api_client = WallboxApiClient::new(wallbox_api_client_config);

    let state_client_config = StateClientConfig::from_env().await?;
    let state_client = StateClient::new(state_client_config);

    let bigquery_client_config = BigqueryClientConfig::from_env().await?;
    let bigquery_client = BigqueryClient::new(bigquery_client_config);

    let config_client_config = ConfigClientConfig::from_env()?;
    let config_client = ConfigClient::new(config_client_config);

    let exporter_service_config = ExporterServiceConfig::new(
        config_client,
        bigquery_client,
        state_client,
        Box::new(wallbox_api_client),
    )?;
    let exporter_service = ExporterService::new(exporter_service_config);

    exporter_service.run().await?;

    Ok(())
}
