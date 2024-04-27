use crate::commons::auth;
use crate::commons::errors::*;
use reqwest::StatusCode;
use reqwest::blocking::Response;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, USER_AGENT, CONTENT_TYPE};
use serde::Serialize;
use std::io::Read;

static API_HOST: &str = "https://api.bitfinex.com/v2/";
static API_SIGNATURE_PATH: &str = "/api/v2/auth/";
static API_SIGNATURE_READ_PATH: &str = "/api/v2/auth/r/";
static API_SIGNATURE_WRITE_PATH: &str = "/api/v2/auth/w/";
static NO_PARAMS: &[(); 0] = &[];

#[derive(Clone)]
pub struct Client {
    api_key: String,
    secret_key: String,
    inner_client: reqwest::blocking::Client,
}

impl Client {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Client {
            api_key: api_key.unwrap_or_else(|| "".into()),
            secret_key: secret_key.unwrap_or_else(|| "".into()),
            inner_client: reqwest::blocking::Client::builder()
                .pool_idle_timeout(None)
                .build()
                .unwrap(),
        }
    }

    pub fn get(&self, endpoint: String, request: String) -> Result<String> {
        let mut url: String = format!("{}{}", API_HOST, endpoint);
        if !request.is_empty() {
            url.push_str(format!("?{}", request).as_str());
        }

        let client = &self.inner_client;
        let response = client.get(url.as_str()).send()?;

        self.handler(response)
    }

    pub fn post_signed(&self, request: String, payload: String) -> Result<String> {
        let url: String = format!("{}auth/{}", API_HOST, request);

        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(
                request,
                payload.clone(),
                API_SIGNATURE_PATH.to_string(),
            )?)
            .body(payload)
            .send()?;

        self.handler(response)
    }

    pub fn post_signed_read(&self, request: String, payload: String) -> Result<String> {
        self.post_signed_params_read(request, payload, NO_PARAMS)
    }

    pub fn post_signed_params_read<P: Serialize + ?Sized>(
        &self, request: String, payload: String, params: &P,
    ) -> Result<String> {
        let url: String = format!("{}auth/r/{}", API_HOST, request);

        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(
                request,
                payload.clone(),
                API_SIGNATURE_READ_PATH.to_string(),
            )?)
            .body(payload)
            .query(params)
            .send()?;

        self.handler(response)
    }

    pub fn post_signed_write(&self, request: String, payload: String) -> Result<String> {
        self.post_signed_params_write(request, payload, NO_PARAMS)
    }

    pub fn post_signed_params_write<P: Serialize + ?Sized>(
        &self, request: String, payload: String, params: &P,
    ) -> Result<String> {
        let url: String = format!("{}auth/w/{}", API_HOST, request);

        let client = &self.inner_client;
        let response = client
            .post(url.as_str())
            .headers(self.build_headers(
                request,
                payload.clone(),
                API_SIGNATURE_WRITE_PATH.to_string(),
            )?)
            .body(payload)
            .query(params)
            .send()?;
        self.handler(response)
    }

    fn build_headers(
        &self, request: String, payload: String, sig_path: String,
    ) -> Result<HeaderMap> {
        let nonce: String = auth::generate_nonce()?;
        let signature_path: String = format!("{}{}{}{}", sig_path, request, nonce, payload);

        let signature = auth::sign_payload(self.secret_key.as_bytes(), signature_path.as_bytes())?;

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("bitfinex-rs"));
        headers.insert(
            HeaderName::from_static("bfx-nonce"),
            HeaderValue::from_str(nonce.as_str())?,
        );
        headers.insert(
            HeaderName::from_static("bfx-apikey"),
            HeaderValue::from_str(self.api_key.as_str())?,
        );
        headers.insert(
            HeaderName::from_static("bfx-signature"),
            HeaderValue::from_str(signature.as_str())?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Ok(headers)
    }

    fn handler(&self, mut response: Response) -> Result<String> {
        let mut body = String::new();
        match response.status() {
            StatusCode::OK => {
                response.read_to_string(&mut body)?;
                Ok(body)
            }
            StatusCode::INTERNAL_SERVER_ERROR => {
                response.read_to_string(&mut body)?;
                bail!("Internal Server Error: {}", body);
            }
            StatusCode::SERVICE_UNAVAILABLE => {
                response.read_to_string(&mut body)?;
                bail!("Service Unavailable: {}", body);
            }
            StatusCode::UNAUTHORIZED => {
                response.read_to_string(&mut body)?;
                bail!("Unauthorized {}", body);
            }
            StatusCode::BAD_REQUEST => {
                response.read_to_string(&mut body)?;
                bail!(format!("Bad Request: {}", body));
            }
            _ => {
                response.read_to_string(&mut body)?;
                bail!(format!("Received response: {}", body));
            }
        }
    }
}
