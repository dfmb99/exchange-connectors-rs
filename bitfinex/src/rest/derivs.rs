use crate::rest::client::*;
use crate::commons::errors::*;
use serde_json::{from_str, to_string};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DerivStatus {
    pub key: String,
    pub timestamp: u64,
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    pub deriv_price: f64,
    pub spot_price: f64,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    pub insurance_fund_balance: f64,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    pub next_funding_timestamp: u64,
    pub next_funding_accrued: f64,
    pub next_funding_step: f64,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
    pub current_funding: f64,
    #[serde(skip_serializing)]
    _placeholder_5: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,
    pub mark_price: f64,
    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_8: Option<String>,
    pub open_interest: Option<f64>,
    #[serde(skip_serializing)]
    _placeholder_9: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_10: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_11: Option<String>,
    pub clamp_min: Option<f64>,
    pub clamp_max: Option<f64>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct DerivStatusHist {
    pub timestamp: u64,
    #[serde(skip_serializing)]
    _placeholder_1: Option<String>,
    pub deriv_price: f64,
    pub spot_price: f64,
    #[serde(skip_serializing)]
    _placeholder_2: Option<String>,
    pub insurance_fund_balance: f64,
    #[serde(skip_serializing)]
    _placeholder_3: Option<String>,
    pub next_funding_timestamp: u64,
    pub next_funding_accrued: f64,
    pub next_funding_step: f64,
    #[serde(skip_serializing)]
    _placeholder_4: Option<String>,
    pub current_funding: f64,
    #[serde(skip_serializing)]
    _placeholder_5: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_6: Option<String>,
    pub mark_price: f64,
    #[serde(skip_serializing)]
    _placeholder_7: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_8: Option<String>,
    pub open_interest: Option<f64>,
    #[serde(skip_serializing)]
    _placeholder_9: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_10: Option<String>,
    #[serde(skip_serializing)]
    _placeholder_11: Option<String>,
    pub clamp_min: Option<f64>,
    pub clamp_max: Option<f64>,
}

#[derive(Serialize, Deserialize, Default)]
pub struct DerivsPosCollaterall {
    pub status: i32,
}

#[derive(Serialize, Deserialize, Default)]
pub struct DerivsPosCollaterallLimits {
    pub min_collateral: f64,
    pub max_collateral: f64,
}

#[derive(Debug, Clone, Default)]
pub struct DerivStatusHistParams {
    pub start: Option<String>,
    pub end: Option<String>,
    pub sort: Option<i32>,
    pub limit: Option<String>,
}

impl DerivStatusHistParams {
    pub fn to_query(&self) -> String {
        format!(
            "{}={}&{}={}&{}={}&{}={}",
            "start",
            self.start.to_owned().unwrap_or_else(|| "".into()),
            "end",
            self.end.to_owned().unwrap_or_else(|| "".into()),
            "sort",
            self.sort
                .map(|a| a.to_string())
                .unwrap_or_else(|| "".into()),
            "limit",
            self.limit.to_owned().unwrap_or_else(|| "".into()),
        )
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct DerivsPosCollaterallParams {
    pub symbol: String,
    pub collateral: f64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct DerivsPosCollaterallLimitsParams {
    pub symbol: String,
}

#[derive(Clone)]
pub struct Derivs {
    client: Client,
}

impl Derivs {
    pub fn new(api_key: Option<String>, secret_key: Option<String>) -> Self {
        Derivs {
            client: Client::new(api_key, secret_key),
        }
    }

    /// Endpoint used to receive different types of platform information - currently supports derivatives pair status only.
    ///
    /// * `symbols` - array of symbols to fetch or 'ALL' to fetch all symbols
    pub fn derivs_status(&self, symbols: Vec<&str>) -> Result<Vec<DerivStatus>> {
        let mut request = String::new();
        for symbol in symbols {
            if !request.is_empty() {
                request = format!("{},{}", request, symbol);
            } else {
                request = symbol.to_string();
            }
        }
        let data = self
            .client
            .get("status/deriv".into(), format!("keys={}", request))?;

        let ticker: Vec<DerivStatus> = from_str(data.as_str())?;

        Ok(ticker)
    }

    pub fn derivs_status_hist<S>(
        &self, symbol: S, params: &DerivStatusHistParams,
    ) -> Result<Vec<DerivStatusHist>>
    where
        S: Into<String>,
    {
        let data = self.client.get(
            format!("status/deriv/{}/hist", symbol.into()),
            params.to_query(),
        )?;

        let ticker: Vec<DerivStatusHist> = from_str(data.as_str())?;

        Ok(ticker)
    }

    pub fn derivs_pos_collateral(
        &self, params: &DerivsPosCollaterallParams,
    ) -> Result<Vec<DerivsPosCollaterall>> {
        let payload: String = to_string(params)?;
        let data = self
            .client
            .post_signed_write("deriv/collateral/set".into(), payload)?;

        let ticker: Vec<DerivsPosCollaterall> = from_str(data.as_str())?;

        Ok(ticker)
    }

    pub fn derivs_pos_collateral_limits(
        &self, params: &DerivsPosCollaterallLimitsParams,
    ) -> Result<DerivsPosCollaterallLimits> {
        let payload: String = to_string(params)?;
        let data = self
            .client
            .post_signed("calc/deriv/collateral/limits".into(), payload)?;

        let ticker: DerivsPosCollaterallLimits = from_str(data.as_str())?;

        Ok(ticker)
    }

    pub fn list_derivs_pairs(&self) -> Result<Vec<String>> {
        let data = self
            .client
            .get("conf/pub:list:pair:futures".into(), String::new())?;

        let derivs: Vec<Vec<String>> = from_str(data.as_str())?;

        Ok(derivs[0].to_owned())
    }
}
