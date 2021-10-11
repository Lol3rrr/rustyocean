use crate::metrics::vpc;

use super::{APIRessource, GetResouceError, API};

use async_trait::async_trait;
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

#[derive(Debug, Deserialize)]
pub struct Balance {
    pub account_balance: String,
    pub generated_at: String,
    pub month_to_date_balance: String,
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
