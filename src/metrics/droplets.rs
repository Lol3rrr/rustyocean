use lazy_static::lazy_static;
use prometheus::{labels, GaugeVec, IntGaugeVec, Opts, Registry};

use crate::api::{self, DropletStatus, Droplets};

lazy_static! {
    static ref DROPLET_UP: IntGaugeVec = IntGaugeVec::new(
        Opts::new("droplet_up", "If a given Droplet is currently running"),
        &["id", "name", "region"]
    )
    .unwrap();
    static ref DROPLET_VCPUS: IntGaugeVec = IntGaugeVec::new(
        Opts::new("droplet_vcpus", "The Number of VCPUs for a given Droplet"),
        &["id", "name", "region"]
    )
    .unwrap();
    static ref DROPLET_MEMORY: IntGaugeVec = IntGaugeVec::new(
        Opts::new("droplet_memory", "The Memory for a given Droplet"),
        &["id", "name", "region"]
    )
    .unwrap();
    static ref DROPLET_DISK: IntGaugeVec = IntGaugeVec::new(
        Opts::new("droplet_disk", "The Disk size for a given Droplet"),
        &["id", "name", "region"]
    )
    .unwrap();
    static ref DROPLET_TRANSFER: GaugeVec = GaugeVec::new(
        Opts::new("droplet_transfer", "The Transfer for a given Droplet"),
        &["id", "name", "region"]
    )
    .unwrap();
    static ref DROPLET_PRICE_MONTHLY: GaugeVec = GaugeVec::new(
        Opts::new(
            "droplet_price_monthly",
            "The Monthly Price for a given Droplet"
        ),
        &["id", "name", "region"]
    )
    .unwrap();
    static ref DROPLET_PRICE_HOURLY: GaugeVec = GaugeVec::new(
        Opts::new(
            "droplet_price_hourly",
            "The Hourly Price for a given Droplet"
        ),
        &["id", "name", "region"]
    )
    .unwrap();
}

pub fn register_metrics(registry: &Registry) {
    registry.register(Box::new(DROPLET_UP.clone())).unwrap();
    registry.register(Box::new(DROPLET_VCPUS.clone())).unwrap();
    registry.register(Box::new(DROPLET_MEMORY.clone())).unwrap();
    registry.register(Box::new(DROPLET_DISK.clone())).unwrap();
    registry
        .register(Box::new(DROPLET_TRANSFER.clone()))
        .unwrap();
    registry
        .register(Box::new(DROPLET_PRICE_MONTHLY.clone()))
        .unwrap();
    registry
        .register(Box::new(DROPLET_PRICE_HOURLY.clone()))
        .unwrap();
}

fn clear_metrics() {
    DROPLET_UP.reset();
    DROPLET_VCPUS.reset();
    DROPLET_MEMORY.reset();
    DROPLET_DISK.reset();
    DROPLET_TRANSFER.reset();
    DROPLET_PRICE_MONTHLY.reset();
    DROPLET_PRICE_HOURLY.reset();
}

#[tracing::instrument(skip(client))]
pub async fn update(client: &api::API) {
    let droplets = match client.load_resource::<Droplets>().await {
        Ok(d) => d,
        Err(e) => {
            tracing::error!("Loading-Droplets: {:?}", e);
            return;
        }
    };

    clear_metrics();

    for droplet in droplets.iter() {
        let id = droplet.id;
        let id_str = id.to_string();

        let up = match &droplet.status {
            DropletStatus::Active => 1,
            _ => 0,
        };

        let droplet_labels = labels! {
            "id" => id_str.as_ref(),
            "name" => droplet.name.as_ref(),
            "region" => droplet.region.slug.as_ref(),
        };

        DROPLET_UP.with(&droplet_labels).set(up);
        DROPLET_VCPUS
            .with(&droplet_labels)
            .set(droplet.vcpus as i64);
        DROPLET_MEMORY
            .with(&droplet_labels)
            .set(droplet.memory as i64);
        DROPLET_DISK.with(&droplet_labels).set(droplet.disk as i64);
        DROPLET_TRANSFER
            .with(&droplet_labels)
            .set(droplet.size.transfer);
        DROPLET_PRICE_MONTHLY
            .with(&droplet_labels)
            .set(droplet.size.price_monthly);
        DROPLET_PRICE_HOURLY
            .with(&droplet_labels)
            .set(droplet.size.price_hourly);
    }
}
