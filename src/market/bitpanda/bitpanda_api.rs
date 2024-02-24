use std::error::Error;
use serde::de::DeserializeOwned;
use serde_json::json;
use crate::market::bitpanda::model::{BitPandaError, Candlestick, Currency, MarketTicker, Order, PriceTick};
use crate::market::bitpanda::model::balances::Balances;
use crate::market::{InstrumentCode, OrderSide};
use crate::market::bitpanda::model::fee_group::FeeGroup;
use crate::market::market::Market;

type BitPandaResult<T> = Result<T, Box<dyn Error>>;

pub struct BitPandaApi {
    base_url: String
}

impl BitPandaApi {

    pub fn new() -> Self {
        BitPandaApi {
            base_url: String::from("https://api.exchange.bitpanda.com/public/v1")
        }
    }

    fn bitpanda_get<T: DeserializeOwned>(url: &str) -> BitPandaResult<T> {
        //let secret = std::env::var("BITPANDA_API_KEY")?;
        let rcl = reqwest::blocking::Client::new();
        let res = rcl.get(url)
            //.header("Authorization", "Bearer ".to_owned() + &secret)
            .send()?;
        match res.status() {
            reqwest::StatusCode::OK => match res.json::<T>() {
                Ok(parsed) => Ok(parsed),
                Err(e) => Err(Box::new(BitPandaError::from(&format!("Failed to parse success response :\n{}", e))))
            },
            _ => match res.json::<BitPandaError>() {
                Ok(parsed) => Err(Box::new(BitPandaError::from(format!("Failed to request BitPanda API : {}", parsed.error).as_str()))),
                Err(_) => Err(Box::new(BitPandaError::from("Failed to request BitPanda API and to parse error response")))
            }
        }
    }

    fn bitpanda_post<T: DeserializeOwned>(url: &str, body: String) -> BitPandaResult<T> {
        //let secret = std::env::var("BITPANDA_API_KEY")?;
        let rcl = reqwest::blocking::Client::new();
        let res = rcl.post(url)
            //.header("Authorization", "Bearer ".to_owned() + &secret)
            .body(body)
            .send()?;
        match res.status() {
            reqwest::StatusCode::OK => match res.json::<T>() {
                Ok(parsed) => Ok(parsed),
                Err(_) => Err(Box::new(BitPandaError::from("Failed to parse success response")))
            },
            _ => match res.json::<BitPandaError>() {
                Ok(parsed) => Err(Box::new(BitPandaError::from(format!("Failed to request BitPanda API : {}", parsed.error).as_str()))),
                Err(_) => Err(Box::new(BitPandaError::from("Failed to request BitPanda API and to parse error response")))
            }
        }
    }

    pub fn _currencies(&self) -> BitPandaResult<Vec<Currency>> {
        BitPandaApi::bitpanda_get(&format!("{}/currencies", self.base_url))
    }

    pub fn _candlesticks(&self, instrument_code: &str) -> BitPandaResult<Vec<Candlestick>> {
        BitPandaApi::bitpanda_get(&format!("{}/candlesticks/{}?unit=MINUTES&period=1&from=2022-06-10T08%3A10%3A59.999Z&to=2022-06-10T08%3A11%3A59.999Z", self.base_url, instrument_code))
    }

    pub fn _market_ticker(&self, instrument_code: &str) -> BitPandaResult<MarketTicker> {
        BitPandaApi::bitpanda_get(&format!("{}/market-ticker/{}", self.base_url, instrument_code))
    }

    pub fn _balances(&self) -> BitPandaResult<Balances> {
        BitPandaApi::bitpanda_get(&format!("{}/account/balances", self.base_url))
    }

}

impl Market for BitPandaApi {

    fn price_tick(&self, instrument_code: InstrumentCode) -> BitPandaResult<Vec<PriceTick>> {
        BitPandaApi::bitpanda_get(&format!("{}/price-ticks/{}", self.base_url, &instrument_code.str))
    }

    fn create_order(&self, instrument_code: InstrumentCode, side: OrderSide, amount: f64) -> Result<Order, Box<dyn Error>> {
        BitPandaApi::bitpanda_post(&format!("{}/account/orders", self.base_url), json!({
            "instrument_code": instrument_code,
            "side": side.str,
            "type": "MARKET",
            "amount": amount
        }).as_str().unwrap().to_owned())
    }

    fn fee_groups(&self) -> Result<Vec<FeeGroup>, Box<dyn Error>> {
        BitPandaApi::bitpanda_get(&format!("{}/fees", self.base_url))
    }

}
