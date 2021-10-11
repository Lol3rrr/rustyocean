use lazy_static::lazy_static;
use prometheus::{labels, IntGaugeVec, Opts, Registry};

use crate::api;

lazy_static! {
    static ref FLOATING_IP: IntGaugeVec = IntGaugeVec::new(
        Opts::new("floating_ip", "Information about a Floating-IP"),
        &["ip", "region"]
    )
    .unwrap();
}

pub fn register_metrics(registry: &Registry) {
    registry.register(Box::new(FLOATING_IP.clone())).unwrap();
}

#[tracing::instrument(skip(client))]
pub async fn update(client: &api::API) {
    let floating_ips = match client.get_floating_ips().await {
        Ok(f) => f,
        Err(e) => {
            tracing::error!("Loading Floating-IPs: {:?}", e);
            return;
        }
    };

    FLOATING_IP.reset();

    for floating_ip in floating_ips.iter() {
        let floating_ip_labels = labels! {
            "ip" => floating_ip.ip.as_ref(),
            "region" => floating_ip.region.slug.as_ref(),
        };

        FLOATING_IP.with(&floating_ip_labels).set(1);
    }
}
