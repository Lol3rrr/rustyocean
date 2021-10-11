//! This contains all the Stuff to interact with the DigitalOcean API

use std::fmt::Debug;

use async_trait::async_trait;

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

        Ok(body)
    }

    /// Simply loads the given Resource from the API
    pub async fn load_resource<R>(&self) -> Result<R::LoadData, GetResouceError>
    where
        R: APIRessource,
    {
        R::load(self).await
    }
}

/// Defines a simple Interface to load a Resource from the API
#[async_trait]
pub trait APIRessource {
    /// The Concrete Type to be loaded, this also allows you to load a List of Resources and not
    /// only a single Instance
    type LoadData: Sized;

    /// Loads the Resource from the API
    async fn load(api: &API) -> Result<Self::LoadData, GetResouceError>;
}
