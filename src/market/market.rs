use std::error::Error;
use serde::{Serialize, Deserialize};
use crate::market::bitpanda::model::{Order, PriceTick};
use crate::market::bitpanda::model::fee_group::FeeGroup;

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct InstrumentCode {
    pub str: &'static str
}

impl InstrumentCode {
    pub const BTC_EUR: InstrumentCode = InstrumentCode { str: "BTC_EUR" };
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct OrderSide {
    pub str: &'static str
}

impl OrderSide {
    pub const BUY: OrderSide = OrderSide { str: "BUY" };
    pub const SELL: OrderSide = OrderSide { str: "SELL" };
}

pub trait Market {
    fn price_tick(&self, instrument_code: InstrumentCode) -> Result<Vec<PriceTick>, Box<dyn Error>>;
    fn create_order(&self, instrument_code: InstrumentCode, side: OrderSide, amount: f64) -> Result<Order, Box<dyn Error>>;
    fn fee_groups(&self) -> Result<Vec<FeeGroup>, Box<dyn Error>>;
}