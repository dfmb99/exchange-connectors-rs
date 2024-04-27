use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct OkxContentError {
    pub code: String,
    pub msg: String,
}

error_chain! {
    errors {
        OkxError(response: OkxContentError)
     }

    foreign_links {
        ReqError(reqwest::Error);
        InvalidHeaderError(reqwest::header::InvalidHeaderValue);
        IoError(std::io::Error);
        ParseFloatError(std::num::ParseFloatError);
        UrlParserError(url::ParseError);
        Json(serde_json::Error);
        Tungstenite(tungstenite::Error);
        TimestampError(std::time::SystemTimeError);
    }
}
