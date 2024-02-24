use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceTick {
    #[serde(rename = "instrument_code")]
    pub instrument_code: String,
    pub price: String,
    pub amount: String,
    #[serde(rename = "taker_side")]
    pub taker_side: String,
    pub volume: String,
    pub time: String,
    #[serde(rename = "trade_timestamp")]
    pub trade_timestamp: i64,
    pub sequence: i64,
}
