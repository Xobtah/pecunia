use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Candlestick {
    #[serde(rename = "last_sequence")]
    pub last_sequence: i64,
    #[serde(rename = "instrument_code")]
    pub instrument_code: String,
    pub granularity: Granularity,
    pub high: String,
    pub low: String,
    pub open: String,
    pub close: String,
    #[serde(rename = "total_amount")]
    pub total_amount: String,
    pub volume: String,
    pub time: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Granularity {
    pub unit: String,
    pub period: i64,
}
