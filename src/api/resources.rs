use super::{APIRessource, GetResouceError, API};

use async_trait::async_trait;
use serde::Deserialize;

/// Represents a single Account and some basic information that is assosicated with it
#[derive(Debug, Deserialize)]
pub struct Account {
    /// The Number of Droplets that can belong to the Account at any given Time
    pub droplet_limit: u64,
    /// The Email address of the Account
    pub email: String,
    /// Whether or not the Email has been verified
    pub email_verified: bool,
    /// The Number of Floating IPs that can belong to the Account at any given Time
    pub floating_ip_limit: u64,
    // TODO
    // Investigate this attribute a bit more
    /// The current Status of the Account (?)
    pub status: String,
    /// The UUID to uniquely identify this Account
    pub uuid: String,
    /// The Number of Volumes that can belong to the Account at any given Time
    pub volume_limit: u64,
}

#[async_trait]
impl APIRessource for Account {
    type LoadData = Self;

    async fn load(api: &API) -> Result<Self::LoadData, GetResouceError> {
        let raw_body = api.get("/account").await?;

        let raw_acc = raw_body
            .get("account")
            .ok_or(GetResouceError::MissingData)?;

        let acc = serde_json::from_value(raw_acc.clone())?;

        Ok(acc)
    }
}

/// Represents the Balance of a given Account
#[derive(Debug, Deserialize)]
pub struct Balance {
    pub account_balance: String,
    pub generated_at: String,
    pub month_to_date_balance: String,
    /// The current Usage in this Month
    pub month_to_date_usage: String,
}

#[async_trait]
impl APIRessource for Balance {
    type LoadData = Self;

    async fn load(api: &API) -> Result<Self::LoadData, GetResouceError> {
        let raw_body = api.get("/customers/my/balance").await?;

        let balance = serde_json::from_value(raw_body)?;
        Ok(balance)
    }
}

/// Represents a single Droplet
#[derive(Debug, Deserialize)]
pub struct Droplet {
    /// The ID to uniquely identify a Droplet
    pub id: u64,
    /// The Name of the Droplet
    pub name: String,
    /// The specified Memory assigned to the Droplet
    pub memory: u64,
    /// The specified Number of virtuel CPU-Cores assigned to the Droplet
    pub vcpus: u64,
    /// The specified Disk Size assigned to the Droplet
    pub disk: u64,
    pub locked: bool,
    /// The current Status of the Droplet
    pub status: DropletStatus,
    /// The Time at which the Droplet has been created
    pub created_at: String,
    /// The Size and other general Information about the Droplet
    pub size: DropletSize,
    /// The Region in which the Droplet exists
    pub region: Region,
}

/// The Status of a Droplet
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

/// Information about a Droplets specified Size
#[derive(Debug, Deserialize)]
pub struct DropletSize {
    /// The Slug used to identify a Droplet-Size
    pub slug: String,
    /// The Memory of the Droplet
    pub memory: u64,
    /// The Number of vCPUs of the Droplet
    pub vcpus: u64,
    /// The Size of the Disk for the Droplet
    pub disk: u64,
    /// The Transfer Bandwidth for the Droplet
    pub transfer: f64,
    /// The Monthly Price for the Droplet
    pub price_monthly: f64,
    /// The Hourly Price for the Droplet
    pub price_hourly: f64,
    /// A Description of the Droplet Size
    // TODO
    // Investigate this attribute
    pub description: String,
}

/// This Resource represents a List of Droplets that can be loaded from the API
pub struct Droplets {}

#[async_trait]
impl APIRessource for Droplets {
    type LoadData = Vec<Droplet>;

    async fn load(api: &API) -> Result<Self::LoadData, GetResouceError> {
        let raw_body = api.get("/droplets").await?;

        let raw_droplets = raw_body
            .get("droplets")
            .ok_or(GetResouceError::MissingData)?;

        let droplets = serde_json::from_value(raw_droplets.clone())?;

        Ok(droplets)
    }
}

/// Represents a Region
#[derive(Debug, Deserialize)]
pub struct Region {
    /// The Name of the Region
    pub name: String,
    /// The Slug to uniquely identify this Region
    pub slug: String,
}

/// Represents a FloatingIP
#[derive(Debug, Deserialize)]
pub struct FloatingIp {
    /// The public IP
    pub ip: String,
    /// The Region in which this FloatingIP exists
    pub region: Region,
}

/// Represents a List of FloatingIPs that can be loaded from the API
pub struct FloatingIps {}

#[async_trait]
impl APIRessource for FloatingIps {
    type LoadData = Vec<FloatingIp>;

    async fn load(api: &API) -> Result<Self::LoadData, GetResouceError> {
        let raw_body = api.get("/floating_ips").await?;

        let raw_floating_ips = raw_body
            .get("floating_ips")
            .ok_or(GetResouceError::MissingData)?;

        let floating_ips = serde_json::from_value(raw_floating_ips.clone())?;

        Ok(floating_ips)
    }
}

/// Represents a single VPC
#[derive(Debug, Deserialize)]
pub struct VPC {
    /// The Name of the VPC
    pub name: String,
    /// A small Description of the VPC, e.g it's prupose or content
    pub description: String,
    /// The Region in which this VPC exists
    pub region: String,
    /// The IP-Range that belongs to this VPC
    pub ip_range: String,
    /// Whether or not this is the default VPC for the Region
    pub default: bool,
    /// The ID of the VPC
    pub id: String,
    pub urn: String,
    /// The Time of creation
    pub created_at: String,
}

/// Represents a List of VPCs that can be loaded from the API
pub struct VPCs {}

#[async_trait]
impl APIRessource for VPCs {
    type LoadData = Vec<VPC>;

    async fn load(api: &API) -> Result<Self::LoadData, GetResouceError> {
        let raw_body = api.get("/vpcs").await?;

        let raw_vpcs = raw_body.get("vpcs").ok_or(GetResouceError::MissingData)?;

        let vpcs = serde_json::from_value(raw_vpcs.clone())?;

        Ok(vpcs)
    }
}

/// Represents a single CDN Endpoint
#[derive(Debug, Deserialize)]
pub struct CdnEndpoint {
    /// The ID of the Endpoint
    pub id: String,
    pub origin: String,
    pub endpoint: String,
    /// The TTL for the Content of this CDN-Endpoint
    pub ttl: u64,
    pub certificate_id: String,
    pub custom_domain: String,
    /// The Time at which this was created
    pub created_at: String,
}

/// Represents a List of CdnEndpoints that can be loaded from the API
pub struct CdnEndpoints {}

#[async_trait]
impl APIRessource for CdnEndpoints {
    type LoadData = Vec<CdnEndpoint>;

    async fn load(api: &API) -> Result<Self::LoadData, GetResouceError> {
        let raw_body = api.get("/cdn/endpoints").await?;

        let raw_endpoints = raw_body
            .get("endpoints")
            .ok_or(GetResouceError::MissingData)?;

        let endpoints = serde_json::from_value(raw_endpoints.clone())?;

        Ok(endpoints)
    }
}
