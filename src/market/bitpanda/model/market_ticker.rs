use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarketTicker {
    #[serde(rename = "instrument_code")]
    pub instrument_code: String,
    pub sequence: i64,
    pub state: String,
    #[serde(rename = "is_frozen")]
    pub is_frozen: i64,
    #[serde(rename = "quote_volume")]
    pub quote_volume: String,
    #[serde(rename = "base_volume")]
    pub base_volume: String,
    #[serde(rename = "last_price")]
    pub last_price: String,
    #[serde(rename = "best_bid")]
    pub best_bid: String,
    #[serde(rename = "best_ask")]
    pub best_ask: String,
    #[serde(rename = "price_change")]
    pub price_change: String,
    #[serde(rename = "price_change_percentage")]
    pub price_change_percentage: String,
    pub high: String,
    pub low: String,
}
