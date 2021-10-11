use lazy_static::lazy_static;
use prometheus::{labels, IntGaugeVec, Opts, Registry};

use crate::api;

lazy_static! {
    static ref CDN_ENDPOINT: IntGaugeVec = IntGaugeVec::new(
        Opts::new("cdn_endpoint", "Information about a CDN-Endpoint"),
        &["id", "origin", "endpoint", "ttl", "custom_domain"]
    )
    .unwrap();
}

pub fn register_metrics(registry: &Registry) {
    registry.register(Box::new(CDN_ENDPOINT.clone())).unwrap();
}

#[tracing::instrument(skip(client))]
pub async fn update(client: &api::API) {
    let cdn_endpoints = match client.get_cdn_endpoints().await {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("Loading CDN-Endpoints: {:?}", e);
            return;
        }
    };

    CDN_ENDPOINT.reset();

    for cdn_endpoint in cdn_endpoints.iter() {
        let ttl_str = cdn_endpoint.ttl.to_string();

        let cdn_endpoint_labels = labels! {
            "id" => cdn_endpoint.id.as_ref(),
            "origin" => cdn_endpoint.origin.as_ref(),
            "endpoint" => cdn_endpoint.endpoint.as_ref(),
            "ttl" => ttl_str.as_ref(),
            "custom_domain" => cdn_endpoint.custom_domain.as_ref(),
        };

        CDN_ENDPOINT.with(&cdn_endpoint_labels).set(1);
    }
}
