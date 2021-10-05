use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Account {
    pub droplet_limit: u64,
    pub email: String,
    pub email_verified: bool,
    pub floating_ip_limit: u64,
    pub status: String,
    pub uuid: String,
    pub volume_limit: u64,
}

#[derive(Debug, Deserialize)]
pub struct Balance {
    pub account_balance: String,
    pub generated_at: String,
    pub month_to_date_balance: String,
    pub month_to_date_usage: String,
}

#[derive(Debug, Deserialize)]
pub struct Droplet {
    pub id: u64,
    pub name: String,
    pub memory: u64,
    pub vcpus: u64,
    pub disk: u64,
    pub locked: bool,
    pub status: DropletStatus,
    pub created_at: String,
    pub size: DropletSize,
    pub region: Region,
}

#[derive(Debug, Deserialize)]
pub enum DropletStatus {
    #[serde(rename = "new")]
    New,
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "archive")]
    Archive,
}

#[derive(Debug, Deserialize)]
pub struct DropletSize {
    pub slug: String,
    pub memory: u64,
    pub vcpus: u64,
    pub disk: u64,
    pub transfer: f64,
    pub price_monthly: f64,
    pub price_hourly: f64,
    pub description: String,
}

#[derive(Debug, Deserialize)]
pub struct Region {
    pub name: String,
    pub slug: String,
}

#[derive(Debug, Deserialize)]
pub struct FloatingIp {
    pub ip: String,
    pub region: Region,
}

#[derive(Debug, Deserialize)]
pub struct VPC {
    pub name: String,
    pub description: String,
    pub region: String,
    pub ip_range: String,
    pub default: bool,
    pub id: String,
    pub urn: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CdnEndpoint {
    pub id: String,
    pub origin: String,
    pub endpoint: String,
    pub ttl: u64,
    pub certificate_id: String,
    pub custom_domain: String,
    pub created_at: String,
}
