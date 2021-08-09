use std::fmt::Debug;

use serde::Deserialize;

pub struct API {
    key: String,
    client: reqwest::Client,
}

const BASE_URL: &str = "https://api.digitalocean.com/v2";

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

#[derive(Debug)]
pub enum GetError {
    Reqwest(reqwest::Error),
    StatusCode(reqwest::StatusCode),
    Serde(serde_json::Error),
}

impl From<reqwest::Error> for GetError {
    fn from(e: reqwest::Error) -> Self {
        Self::Reqwest(e)
    }
}
impl From<serde_json::Error> for GetError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e)
    }
}

#[derive(Debug)]
pub enum GetResouceError {
    GetResource(GetError),
    MissingData,
    Serde(serde_json::Error),
}

impl From<GetError> for GetResouceError {
    fn from(e: GetError) -> Self {
        Self::GetResource(e)
    }
}
impl From<serde_json::Error> for GetResouceError {
    fn from(e: serde_json::Error) -> Self {
        Self::Serde(e)
    }
}

impl API {
    pub fn new<I>(key: I) -> Self
    where
        I: Into<String>,
    {
        let client = reqwest::Client::new();
        Self {
            key: key.into(),
            client,
        }
    }

    #[tracing::instrument(skip(self))]
    pub async fn get<I>(&self, resource: I) -> Result<serde_json::Value, GetError>
    where
        I: Into<String> + Debug,
    {
        let resource = {
            let mut tmp = resource.into();
            if !tmp.starts_with('/') {
                tmp = format!("/{}", tmp);
            }
            tmp
        };
        let url = format!("{}{}", BASE_URL, resource);

        let req = self
            .client
            .request(reqwest::Method::GET, url)
            .bearer_auth(&self.key)
            .build()?;

        let response = self.client.execute(req).await?;
        if response.status() != reqwest::StatusCode::OK {
            return Err(GetError::StatusCode(response.status()));
        }

        let raw_body = response.bytes().await?;

        let body = serde_json::from_slice(&raw_body)?;

        return Ok(body);
    }

    pub async fn get_account(&self) -> Result<Account, GetResouceError> {
        let raw_body = self.get("/account").await?;

        let acc_value = raw_body
            .get("account")
            .ok_or(GetResouceError::MissingData)?;
        let acc = serde_json::from_value(acc_value.clone())?;
        Ok(acc)
    }

    pub async fn get_balance(&self) -> Result<Balance, GetResouceError> {
        let raw_body = self.get("/customers/my/balance").await?;

        let balance = serde_json::from_value(raw_body)?;
        Ok(balance)
    }

    pub async fn get_droplets(&self) -> Result<Vec<Droplet>, GetResouceError> {
        let raw_body = self.get("/droplets").await?;

        let raw_droplets = raw_body
            .get("droplets")
            .ok_or(GetResouceError::MissingData)?;
        let droplets = serde_json::from_value(raw_droplets.clone())?;
        Ok(droplets)
    }

    pub async fn get_floating_ips(&self) -> Result<Vec<FloatingIp>, GetResouceError> {
        let raw_body = self.get("/floating_ips").await?;

        let raw_ips = raw_body
            .get("floating_ips")
            .ok_or(GetResouceError::MissingData)?;
        let ips = serde_json::from_value(raw_ips.clone())?;
        Ok(ips)
    }

    pub async fn get_vpcs(&self) -> Result<Vec<VPC>, GetResouceError> {
        let raw_body = self.get("/vpcs").await?;

        let raw_vpcs = raw_body.get("vpcs").ok_or(GetResouceError::MissingData)?;
        let vpcs = serde_json::from_value(raw_vpcs.clone())?;
        Ok(vpcs)
    }

    pub async fn get_cdn_endpoints(&self) -> Result<Vec<CdnEndpoint>, GetResouceError> {
        let raw_body = self.get("/cdn/endpoints").await?;

        let raw_endpoints = raw_body
            .get("endpoints")
            .ok_or(GetResouceError::MissingData)?;
        let endpoints = serde_json::from_value(raw_endpoints.clone())?;
        Ok(endpoints)
    }
}
