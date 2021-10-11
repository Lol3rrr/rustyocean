use lazy_static::lazy_static;
use prometheus::{labels, IntGaugeVec, Opts, Registry};

use crate::api::{self, VPCs};

lazy_static! {
    static ref VPC: IntGaugeVec = IntGaugeVec::new(
        Opts::new("vpc", "Information about a VPC"),
        &["id", "name", "region", "ip_range"]
    )
    .unwrap();
}

pub fn register_metrics(registry: &Registry) {
    registry.register(Box::new(VPC.clone())).unwrap();
}

#[tracing::instrument(skip(client))]
pub async fn update(client: &api::API) {
    let vpcs = match client.load_resource::<VPCs>().await {
        Ok(v) => v,
        Err(e) => {
            tracing::error!("Loading VPCs: {:?}", e);
            return;
        }
    };

    for vpc in vpcs.iter() {
        let vpc_labels = labels! {
            "id" => vpc.id.as_ref(),
            "name" => vpc.name.as_ref(),
            "region" => vpc.region.as_ref(),
            "ip_range" => vpc.ip_range.as_ref(),
        };

        VPC.with(&vpc_labels).set(1);
    }
}
