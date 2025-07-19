use crate::commons::auth;
use crate::commons::errors::BitfinexError;
use reqwest::blocking::Response;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue, CONTENT_TYPE, USER_AGENT};
use reqwest::StatusCode;
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
        let client = reqwest::blocking::Client::builder()
            .pool_idle_timeout(None)
            .build()
            .expect("Failed to create HTTP client");

        Client {
            api_key: api_key.unwrap_or_default(),
            secret_key: secret_key.unwrap_or_default(),
            inner_client: client,
        }
    }

    pub fn get(&self, endpoint: String, request: String) -> Result<String, BitfinexError> {
        let url = if request.is_empty() {
            format!("{API_HOST}{endpoint}")
        } else {
            format!("{API_HOST}{endpoint}?{request}")
        };

        let response = self.inner_client.get(&url).send()?;
        self.handler(response)
    }

    pub fn post_signed(&self, request: String, payload: String) -> Result<String, BitfinexError> {
        let url = format!("{API_HOST}auth/{request}");
        let headers = self.build_headers(&request, &payload, API_SIGNATURE_PATH)?;

        let response = self
            .inner_client
            .post(&url)
            .headers(headers)
            .body(payload)
            .send()?;

        self.handler(response)
    }

    pub fn post_signed_read(
        &self,
        request: String,
        payload: String,
    ) -> Result<String, BitfinexError> {
        self.post_signed_params_read(request, payload, NO_PARAMS)
    }

    pub fn post_signed_params_read<P: Serialize + ?Sized>(
        &self,
        request: String,
        payload: String,
        params: &P,
    ) -> Result<String, BitfinexError> {
        let url = format!("{API_HOST}auth/r/{request}");
        let headers = self.build_headers(&request, &payload, API_SIGNATURE_READ_PATH)?;

        let response = self
            .inner_client
            .post(&url)
            .headers(headers)
            .body(payload)
            .query(params)
            .send()?;

        self.handler(response)
    }

    pub fn post_signed_write(
        &self,
        request: String,
        payload: String,
    ) -> Result<String, BitfinexError> {
        self.post_signed_params_write(request, payload, NO_PARAMS)
    }

    pub fn post_signed_params_write<P: Serialize + ?Sized>(
        &self,
        request: String,
        payload: String,
        params: &P,
    ) -> Result<String, BitfinexError> {
        let url = format!("{API_HOST}auth/w/{request}");
        let headers = self.build_headers(&request, &payload, API_SIGNATURE_WRITE_PATH)?;

        let response = self
            .inner_client
            .post(&url)
            .headers(headers)
            .body(payload)
            .query(params)
            .send()
            .map_err(BitfinexError::RequestError);

        self.handler(response?)
    }

    fn build_headers(
        &self,
        request: &str,
        payload: &str,
        sig_path: &str,
    ) -> Result<HeaderMap, BitfinexError> {
        let nonce = auth::generate_nonce()?;
        let signature_path = format!("{sig_path}{request}{nonce}{payload}");
        let signature = auth::sign_payload(self.secret_key.as_bytes(), signature_path.as_bytes())?;

        let mut headers = HeaderMap::new();
        headers.insert(USER_AGENT, HeaderValue::from_static("bitfinex-rs"));
        headers.insert(
            HeaderName::from_static("bfx-nonce"),
            HeaderValue::from_str(&nonce).map_err(BitfinexError::InvalidHeader)?,
        );
        headers.insert(
            HeaderName::from_static("bfx-apikey"),
            HeaderValue::from_str(&self.api_key).map_err(BitfinexError::InvalidHeader)?,
        );
        headers.insert(
            HeaderName::from_static("bfx-signature"),
            HeaderValue::from_str(&signature).map_err(BitfinexError::InvalidHeader)?,
        );
        headers.insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));

        Ok(headers)
    }

    fn handler(&self, mut response: Response) -> Result<String, BitfinexError> {
        let mut body = String::new();
        response.read_to_string(&mut body)?;

        match response.status() {
            StatusCode::OK => Ok(body),
            StatusCode::INTERNAL_SERVER_ERROR => Err(BitfinexError::InternalServerError(body)),
            StatusCode::SERVICE_UNAVAILABLE => Err(BitfinexError::ServiceUnavailable(body)),
            StatusCode::UNAUTHORIZED => Err(BitfinexError::Unauthorized(body)),
            StatusCode::BAD_REQUEST => Err(BitfinexError::BadRequest(body)),
            _ => Err(BitfinexError::Unknown(body)),
        }
    }
}
