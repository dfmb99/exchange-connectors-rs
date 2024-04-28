use crate::commons::errors::*;
use crate::rest::client::Client;
use serde::Serializer;
use serde_json::{from_str, to_string, Value};

#[derive(Clone)]
pub struct Funding {
    client: Client,
}

#[derive(Serialize, Deserialize)]
pub struct FundingOfferData {
    pub id: i64,
    pub symbol: String,
    pub creation_timestamp: i64,
    pub update_timestamp: i64,
    pub amount: f64,
    pub amount_orig: f64,
    pub offer_type: String,
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    pub flags: Option<i32>,
    pub status: String,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_5: Option<String>,
    pub rate: f64,
    pub period: i32,
    #[serde(serialize_with = "bool_or_int")]
    pub notify: Value,
    pub hidden: i32,
    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,
    #[serde(serialize_with = "bool_or_int")]
    pub renew: Value,
    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct FundingOffer {
    pub timestamp: i64,
    pub offer_type: String,
    pub message_id: Option<i64>,
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    pub funding_offer_data: FundingOfferData,
    pub code: Option<i32>,
    pub status: String,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct CanceledFundingOffer {
    pub timestamp: i64,
    pub offer_type: String,
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
    pub status: String,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct SubmitOfferParams {
    #[serde(rename = "type")]
    pub offer_type: String,
    pub symbol: String,
    pub amount: String,
    pub rate: String,
    pub period: i32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flags: Option<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct CancelOfferParams {
    pub id: i64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct CancelAllOffersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

fn bool_or_int<S>(x: &Value, s: S) -> std::result::Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match x {
        Value::Bool(_) => s.serialize_bool(serde_json::from_value(x.to_owned()).unwrap()),
        _ => s.serialize_i32(serde_json::from_value(x.to_owned()).unwrap()),
    }
}

impl Funding {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Funding {
            client: Client::new(api_key, secret_key),
        }
    }

    pub fn active_offers<T>(&self, symbol: T) -> Result<Vec<FundingOfferData>>
    where
        T: Into<Option<String>>,
    {
        let value = symbol.into().unwrap_or_else(|| "".into());
        let payload: String = "{}".to_string();
        let data = self
            .client
            .post_signed_read(format!("funding/offers/{}", value), payload)?;

        let offers: Vec<FundingOfferData> = from_str(data.as_str())?;

        Ok(offers)
    }

    pub fn submit_offer(&self, params: &SubmitOfferParams) -> Result<FundingOffer> {
        let payload: String = to_string(params)?;
        let data = self
            .client
            .post_signed_write("funding/offer/submit".into(), payload)?;

        let offers: FundingOffer = from_str(data.as_str())?;

        Ok(offers)
    }

    pub fn cancel_offer(&self, params: &CancelOfferParams) -> Result<FundingOffer> {
        let payload: String = to_string(params)?;
        let data = self
            .client
            .post_signed_write("funding/offer/cancel".into(), payload)?;

        let offers: FundingOffer = from_str(data.as_str())?;

        Ok(offers)
    }

    pub fn cancel_all_offers(
        &self,
        params: &CancelAllOffersParams,
    ) -> Result<CanceledFundingOffer> {
        let payload: String = to_string(params)?;
        let data = self
            .client
            .post_signed_write("funding/offer/cancel/all".into(), payload)?;

        let offers: CanceledFundingOffer = from_str(data.as_str())?;

        Ok(offers)
    }
}
