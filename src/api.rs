//! This contains all the Stuff to interact with the DigitalOcean API

use std::fmt::Debug;

/// The API Instance to interact with the Digital Ocean API as a given User
pub struct API {
    /// The API Key used to authenticate with the API
    key: ApiKey,
    /// The Reqwest Client used to perform all these Requests
    client: reqwest::Client,
}

/// This represents a single API Key for the DigitalOcean API
pub struct ApiKey {
    /// The underlying Key String
    key: String,
}

impl From<String> for ApiKey {
    fn from(k: String) -> Self {
        Self { key: k }
    }
}
impl From<&str> for ApiKey {
    fn from(k: &str) -> Self {
        Self { key: k.to_string() }
    }
}

/// The Base API-Url for the DigitalOcean API
const BASE_URL: &str = "https://api.digitalocean.com/v2";

mod resources;
pub use resources::*;

/// The Error received when it could not get something from the API
#[derive(Debug)]
pub enum GetError {
    /// Something with performing the Request itself went wrong
    Reqwest(reqwest::Error),
    /// The Returned Response has an unexpected StatusCode
    StatusCode(reqwest::StatusCode),
    /// The Response Payload had an unexpected/invalid Format
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

/// The Error received when it could not load a specific Resource from the API
#[derive(Debug)]
pub enum GetResouceError {
    /// Loading the Data from the Resource failed
    GetResource(GetError),
    /// The Data from the API was missing some required Data
    MissingData,
    /// The Data could not be deserlazed properly
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
    /// This creates a new API Instance with the given API-Key
    pub fn new<I>(key: I) -> Self
    where
        I: Into<ApiKey>,
    {
        let client = reqwest::Client::new();
        Self {
            key: key.into(),
            client,
        }
    }

    /// Simply attempts to load the given Resource from the DigitalOcean API
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
            .bearer_auth(&self.key.key)
            .build()?;

        let response = self.client.execute(req).await?;
        if response.status() != reqwest::StatusCode::OK {
            return Err(GetError::StatusCode(response.status()));
        }

        let raw_body = response.bytes().await?;

        let body = serde_json::from_slice(&raw_body)?;

        return Ok(body);
    }

    /// Loads the Account Data for the Account assosicated with the API Key
    pub async fn get_account(&self) -> Result<Account, GetResouceError> {
        let raw_body = self.get("/account").await?;

        let acc_value = raw_body
            .get("account")
            .ok_or(GetResouceError::MissingData)?;
        let acc = serde_json::from_value(acc_value.clone())?;
        Ok(acc)
    }

    /// Loads the Balance information for the Account assosicated with the API Key
    pub async fn get_balance(&self) -> Result<Balance, GetResouceError> {
        let raw_body = self.get("/customers/my/balance").await?;

        let balance = serde_json::from_value(raw_body)?;
        Ok(balance)
    }

    /// Loads a list of all Droplets
    pub async fn get_droplets(&self) -> Result<Vec<Droplet>, GetResouceError> {
        let raw_body = self.get("/droplets").await?;

        let raw_droplets = raw_body
            .get("droplets")
            .ok_or(GetResouceError::MissingData)?;
        let droplets = serde_json::from_value(raw_droplets.clone())?;
        Ok(droplets)
    }

    /// Loads a list of all the Floating IP's
    pub async fn get_floating_ips(&self) -> Result<Vec<FloatingIp>, GetResouceError> {
        let raw_body = self.get("/floating_ips").await?;

        let raw_ips = raw_body
            .get("floating_ips")
            .ok_or(GetResouceError::MissingData)?;
        let ips = serde_json::from_value(raw_ips.clone())?;
        Ok(ips)
    }

    /// Loads a list of all the VPC's
    pub async fn get_vpcs(&self) -> Result<Vec<VPC>, GetResouceError> {
        let raw_body = self.get("/vpcs").await?;

        let raw_vpcs = raw_body.get("vpcs").ok_or(GetResouceError::MissingData)?;
        let vpcs = serde_json::from_value(raw_vpcs.clone())?;
        Ok(vpcs)
    }

    /// Loads a list of all the CDN-Endpoint's
    pub async fn get_cdn_endpoints(&self) -> Result<Vec<CdnEndpoint>, GetResouceError> {
        let raw_body = self.get("/cdn/endpoints").await?;

        let raw_endpoints = raw_body
            .get("endpoints")
            .ok_or(GetResouceError::MissingData)?;
        let endpoints = serde_json::from_value(raw_endpoints.clone())?;
        Ok(endpoints)
    }
}
