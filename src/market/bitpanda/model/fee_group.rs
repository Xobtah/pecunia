use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeGroup {
    #[serde(rename = "fee_group_id")]
    pub fee_group_id: String,
    #[serde(rename = "display_text")]
    pub display_text: String,
    #[serde(rename = "fee_tiers")]
    pub fee_tiers: Vec<FeeTier>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeTier {
    pub volume: String,
    #[serde(rename = "fee_group_id")]
    pub fee_group_id: String,
    #[serde(rename = "maker_fee")]
    pub maker_fee: String,
    #[serde(rename = "taker_fee")]
    pub taker_fee: String,
}
