use crate::commons::auth::get_signature;
use crate::commons::errors::*;
use crate::commons::utils::*;
use crate::rest::api::API;
use error_chain::bail;
use reqwest::blocking::Response;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::StatusCode;
use serde::de::DeserializeOwned;
use std::time::SystemTime;

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    passphrase: String,
    host: String,
    simulated_trading: bool,
    inner_client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(
        api_key: Option<String>, secret_key: Option<String>, passphrase: Option<String>,
        host: String, simulated_trading: bool,
    ) -> Self {
        Client {
            api_key: api_key.unwrap_or_else(|| "".into()),
            secret_key: secret_key.unwrap_or_else(|| "".into()),
            passphrase: passphrase.unwrap_or_else(|| "".into()),
            host,
            simulated_trading,
            inner_client: reqwest::blocking::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
        }
    }

    pub fn get<T: DeserializeOwned>(&self, endpoint: API, request: Option<String>) -> Result<T> {
        let mut url: String = format!("{}{}", self.host, String::from(endpoint));
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{request}").as_str());
            }
        }

        let client = &self.inner_client;
        let response = client.get(url.as_str()).send()?;

        self.handler(response)
    }

    pub fn post<T: DeserializeOwned>(&self, endpoint: API, data: String) -> Result<T> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));

        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(true)?)
            .body(data)
            .send()?;

        self.handler(response)
    }

    pub fn put<T: DeserializeOwned>(&self, endpoint: API, data: String) -> Result<T> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));

        let client = &self.inner_client;
        let response = client
            .put(url.as_str())
            .headers(self.build_headers(true)?)
            .body(data)
            .send()?;

        self.handler(response)
    }

    pub fn delete<T: DeserializeOwned>(&self, endpoint: API, data: String) -> Result<T> {
        let url: String = format!("{}{}", self.host, String::from(endpoint));

        let client = &self.inner_client;
        let response = client
            .delete(url.as_str())
            .headers(self.build_headers(true)?)
            .body(data)
            .send()?;

        self.handler(response)
    }

    pub fn get_signed<T: DeserializeOwned>(
        &self, endpoint: API, request: Option<String>,
    ) -> Result<T> {
        let request_path = String::from(endpoint);
        let mut url: String = format!("{}{}", self.host, request_path);
        if let Some(request) = request {
            if !request.is_empty() {
                url.push_str(format!("?{request}").as_str());
            }
        }
        let client = &self.inner_client;

        let now = SystemTime::now();
        let signature = get_signature(
            self.secret_key.as_str(),
            &get_timestamp_iso_format(now),
            "GET",
            &request_path,
            "",
        );

        let response = client
            .get(url.as_str())
            .headers(self.build_signed_headers(true, now, signature)?)
            .send()?;

        self.handler(response)
    }

    pub fn post_signed<T: DeserializeOwned>(&self, endpoint: API, data: String) -> Result<T> {
        let request_path = String::from(endpoint);
        let url: String = format!("{}{}", self.host, request_path);
        let client = &self.inner_client;

        let now = SystemTime::now();
        let signature = get_signature(
            self.secret_key.as_str(),
            &get_timestamp_iso_format(now),
            "POST",
            &request_path,
            &data,
        );

        let response = client
            .post(url.as_str())
            .headers(self.build_signed_headers(true, now, signature)?)
            .body(data)
            .send()?;

        self.handler(response)
    }

    pub fn put_signed<T: DeserializeOwned>(&self, endpoint: API, data: String) -> Result<T> {
        let request_path = String::from(endpoint);
        let url: String = format!("{}{}", self.host, request_path);
        let client = &self.inner_client;

        let now = SystemTime::now();
        let signature = get_signature(
            self.secret_key.as_str(),
            &get_timestamp_iso_format(now),
            "PUT",
            &request_path,
            &data,
        );

        let response = client
            .put(url.as_str())
            .headers(self.build_signed_headers(true, now, signature)?)
            .body(data)
            .send()?;

        self.handler(response)
    }

    pub fn delete_signed<T: DeserializeOwned>(&self, endpoint: API, data: String) -> Result<T> {
        let request_path = String::from(endpoint);
        let url: String = format!("{}{}", self.host, request_path);
        let client = &self.inner_client;

        let now = SystemTime::now();
        let signature = get_signature(
            self.secret_key.as_str(),
            &get_timestamp_iso_format(now),
            "DELETE",
            &request_path,
            &data,
        );

        let response = client
            .delete(url.as_str())
            .headers(self.build_signed_headers(true, now, signature)?)
            .body(data)
            .send()?;

        self.handler(response)
    }

    fn build_headers(&self, content_type: bool) -> Result<HeaderMap> {
        let mut custom_headers = HeaderMap::new();

        custom_headers.insert(USER_AGENT, HeaderValue::from_static("okx-rs"));
        if content_type {
            custom_headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
        }
        if self.simulated_trading {
            custom_headers.insert(
                HeaderName::from_static("x-simulated-trading"),
                HeaderValue::from_str("1")?,
            );
        }
        Ok(custom_headers)
    }

    fn build_signed_headers(
        &self, content_type: bool, timestamp: SystemTime, signature: String,
    ) -> Result<HeaderMap> {
        let mut custom_headers = self.build_headers(content_type)?;

        custom_headers.insert(
            HeaderName::from_static("ok-access-key"),
            HeaderValue::from_str(self.api_key.as_str())?,
        );

        custom_headers.insert(
            HeaderName::from_static("ok-access-sign"),
            HeaderValue::from_str(&signature)?,
        );

        custom_headers.insert(
            HeaderName::from_static("ok-access-timestamp"),
            HeaderValue::from_str(&get_timestamp_iso_format(timestamp))?,
        );

        custom_headers.insert(
            HeaderName::from_static("ok-access-passphrase"),
            HeaderValue::from_str(self.passphrase.as_str())?,
        );

        Ok(custom_headers)
    }

    fn handler<T: DeserializeOwned>(&self, response: Response) -> Result<T> {
        match response.status() {
            StatusCode::OK => Ok(response.json::<T>()?),
            StatusCode::INTERNAL_SERVER_ERROR => Err(ErrorKind::OkxError(response.json()?).into()),
            StatusCode::SERVICE_UNAVAILABLE => Err(ErrorKind::OkxError(response.json()?).into()),
            StatusCode::UNAUTHORIZED => Err(ErrorKind::OkxError(response.json()?).into()),
            StatusCode::BAD_REQUEST => Err(ErrorKind::OkxError(response.json()?).into()),
            s => {
                bail!(format!("Received response: {s:?}"));
            }
        }
    }
}
