use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balances {
    #[serde(rename = "account_id")]
    pub account_id: String,
    pub balances: Vec<Balance>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Balance {
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "currency_code")]
    pub currency_code: String,
    pub change: String,
    pub available: String,
    pub locked: String,
    pub sequence: i64,
    pub time: String,
}
