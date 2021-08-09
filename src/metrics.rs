pub mod account {
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
}

pub mod balance {
    use lazy_static::lazy_static;
    use prometheus::{Gauge, Registry};

    use crate::api;

    lazy_static! {
        static ref ACCOUNT_BALANCE: Gauge =
            Gauge::new("account_balance", "The current Account-Balance").unwrap();
        static ref MONTH_TO_DATE_BALANCE: Gauge =
            Gauge::new("month_to_date_balance", "The current Balance with the Usage of the Month already subtracted from the Account-Balance").unwrap();
        static ref MONTH_TO_DATE_USAGE: Gauge =
            Gauge::new("month_to_date_usage", "The current Usage for this Month").unwrap();
    }

    pub fn register_metrics(registry: &Registry) {
        registry
            .register(Box::new(ACCOUNT_BALANCE.clone()))
            .unwrap();
        registry
            .register(Box::new(MONTH_TO_DATE_BALANCE.clone()))
            .unwrap();
        registry
            .register(Box::new(MONTH_TO_DATE_USAGE.clone()))
            .unwrap();
    }

    #[tracing::instrument(skip(client))]
    pub async fn update(client: &api::API) {
        let balance = match client.get_balance().await {
            Ok(b) => b,
            Err(e) => {
                tracing::error!("Loading-Balance: {:?}", e);
                return;
            }
        };

        if let Ok(acc_balance) = balance.account_balance.parse() {
            ACCOUNT_BALANCE.set(acc_balance);
        }
        if let Ok(month_to_date_balance) = balance.month_to_date_balance.parse() {
            MONTH_TO_DATE_BALANCE.set(month_to_date_balance);
        }
        if let Ok(month_to_date_usage) = balance.month_to_date_usage.parse() {
            MONTH_TO_DATE_USAGE.set(month_to_date_usage);
        }
    }
}

pub mod droplets {
    use lazy_static::lazy_static;
    use prometheus::{labels, GaugeVec, IntGaugeVec, Opts, Registry};

    use crate::api::{self, DropletStatus};

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

    #[tracing::instrument(skip(client))]
    pub async fn update(client: &api::API) {
        let droplets = match client.get_droplets().await {
            Ok(d) => d,
            Err(e) => {
                tracing::error!("Loading-Droplets: {:?}", e);
                return;
            }
        };

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
}

pub mod floating_ip {
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

        for floating_ip in floating_ips.iter() {
            let floating_ip_labels = labels! {
                "ip" => floating_ip.ip.as_ref(),
                "region" => floating_ip.region.slug.as_ref(),
            };

            FLOATING_IP.with(&floating_ip_labels).set(1);
        }
    }
}

pub mod vpc {
    use lazy_static::lazy_static;
    use prometheus::{labels, IntGaugeVec, Opts, Registry};

    use crate::api;

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
        let vpcs = match client.get_vpcs().await {
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
}

pub mod cdn_endpoint {
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
}
