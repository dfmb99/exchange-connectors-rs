use crate::commons::client::Client;
use crate::commons::errors::*;
use crate::rest::api::Status::SystemStatus;
use crate::rest::api::{ApiResponse, API};
use crate::serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct Status {
    pub client: Client,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SystemStatusParams {
    /// System maintenance status,scheduled: waiting; ongoing: processing; pre_open: pre_open; completed: completed ;canceled: canceled.
    /// Generally, pre_open last about 10 minutes. There will be pre_open when the time of upgrade is too long.
    /// If this parameter is not filled, the data with status scheduled, ongoing and pre_open will be returned by default
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

impl SystemStatusParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}",
            "state",
            self.state.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct SystemStatusResponse {
    pub begin: String,
    pub end: String,
    pub href: String,
    #[serde(rename = "preOpenBegin")]
    pub pre_open_begin: String,
    #[serde(rename = "scheDesc")]
    pub sche_desc: String,
    #[serde(rename = "serviceType")]
    pub service_type: String,
    pub state: String,
    pub system: String,
    pub title: String,
}

impl Status {
    /// Get event status of system upgrade
    pub fn get_system_status(
        &self,
        params: &SystemStatusParams,
    ) -> Result<ApiResponse<Vec<SystemStatusResponse>>> {
        let system_status: ApiResponse<Vec<SystemStatusResponse>> = self
            .client
            .get(API::Status(SystemStatus), Some(params.to_query()))?;

        Ok(system_status)
    }
}
