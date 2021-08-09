pub mod api;

use std::{sync::Arc, time::Duration};

mod metrics;

pub fn register_metrics(registry: &prometheus::Registry) {
    metrics::account::register_metrics(registry);
    metrics::balance::register_metrics(registry);
    metrics::droplets::register_metrics(registry);
    metrics::floating_ip::register_metrics(registry);
    metrics::vpc::register_metrics(registry);
    metrics::cdn_endpoint::register_metrics(registry);
}

#[tracing::instrument(skip(client))]
async fn load_metrics(client: &api::API) {
    metrics::account::update(client).await;
    metrics::balance::update(client).await;
    metrics::droplets::update(client).await;
    metrics::floating_ip::update(client).await;
    metrics::vpc::update(client).await;
    metrics::cdn_endpoint::update(client).await;
}

#[tracing::instrument(skip(client))]
pub async fn update_metrics(client: Arc<api::API>) {
    loop {
        tracing::debug!("Updating-Metrics...");
        load_metrics(&client).await;

        tokio::time::sleep(Duration::from_secs(60)).await;
    }
}
