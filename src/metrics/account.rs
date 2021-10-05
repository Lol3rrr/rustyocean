use lazy_static::lazy_static;
use prometheus::{IntGauge, Registry};

use crate::api;

lazy_static! {
    static ref DROPLET_LIMIT: IntGauge = IntGauge::new(
        "droplet_limit",
        "The Number of Droplets your Account is allowed to have"
    )
    .unwrap();
    static ref FLOATING_IP_LIMIT: IntGauge = IntGauge::new(
        "floating_ip_limit",
        "The Number of Floating-IPs your Account is allowed to have"
    )
    .unwrap();
    static ref VOLUME_LIMIT: IntGauge = IntGauge::new(
        "volume_limit",
        "The Number of Volumes your Account is allowed to have"
    )
    .unwrap();
}

pub fn register_metrics(registry: &Registry) {
    registry.register(Box::new(DROPLET_LIMIT.clone())).unwrap();
    registry
        .register(Box::new(FLOATING_IP_LIMIT.clone()))
        .unwrap();
    registry.register(Box::new(VOLUME_LIMIT.clone())).unwrap();
}

#[tracing::instrument(skip(client))]
pub async fn update(client: &api::API) {
    let account = match client.get_account().await {
        Ok(a) => a,
        Err(e) => {
            tracing::error!("Loading-Account: {:?}", e);
            return;
        }
    };

    DROPLET_LIMIT.set(account.droplet_limit as i64);
    FLOATING_IP_LIMIT.set(account.floating_ip_limit as i64);
    VOLUME_LIMIT.set(account.volume_limit as i64);
}
