use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_id")]
    pub client_id: String,
    #[serde(rename = "account_id")]
    pub account_id: String,
    #[serde(rename = "instrument_code")]
    pub instrument_code: String,
    pub time: String,
    pub side: String,
    pub price: String,
    pub amount: String,
    #[serde(rename = "filled_amount")]
    pub filled_amount: String,
    #[serde(rename = "type")]
    pub type_field: String,
    #[serde(rename = "time_in_force")]
    pub time_in_force: String,
}
