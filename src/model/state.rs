use std::error::Error;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use chrono::Utc;
use serde::{Serialize, Deserialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Trade {
    date: i64,
    instrument_code: String,
    price: f64,
    amount: f64
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    starting_capital: f64,
    capital: f64,
    trades: Vec<Trade>,
    last_price: f64,
    market_price: f64,
    last_exec: i64
}

impl State {

    pub fn new(starting_capital: f64) -> Self {
        State {
            starting_capital,
            capital: starting_capital,
            trades: Vec::new(),
            last_price: 0.,
            market_price: 0.,
            last_exec: Utc::now().timestamp()
        }
    }

    pub fn from_file(path: &str) -> Result<Self, Box<dyn Error>> {
        match File::open(Path::new(path)) {
            Ok(mut file) => {
                let mut s = String::new();
                file.read_to_string(&mut s)?;
                Ok(serde_json::from_str(&s)?)
            },
            Err(_) => Ok(State::new(100.))
        }
    }

    pub fn to_file(&self, path: &str) -> Result<(), Box<dyn Error>> {
        File::create(Path::new(path))?.write(serde_json::to_string(self)?.as_bytes())?;
        Ok(())
    }

    pub fn update_last_exec(&mut self) {
        self.last_exec = Utc::now().timestamp();
    }

    pub fn last_trade(&self) -> Option<&Trade> {
        self.trades.last()
    }

    pub fn trades_nb(&self) -> usize {
        self.trades.len()
    }

    pub fn buy(&mut self, price: f64, fee: f64, invest_percent: f64) {
        let investment = self.starting_capital / invest_percent;
        let amount_ht = investment / price;
        let amount_ttc = amount_ht - amount_ht * fee;
        self.capital = self.capital - investment;
        self.set_last_price(price);
        self.trades.push(Trade {
            date: Utc::now().timestamp(),
            instrument_code: "BTC_EUR".to_owned(),
            price,
            amount: amount_ttc
        });
    }

    pub fn sell(&mut self, price: f64, fee: f64) {
        let trade = self.last_trade().unwrap();
        self.capital = self.capital + trade.amount * price;
        self.capital = self.capital - self.capital * fee;
        self.last_price = price;
        self.trades.pop();
    }

    pub fn starting_capital(&self) -> f64 {
        self.starting_capital
    }
    pub fn capital(&self) -> f64 {
        self.capital
    }
    pub fn _trades(&self) -> &Vec<Trade> {
        &self.trades
    }
    pub fn last_price(&self) -> f64 {
        self.last_price
    }
    pub fn market_price(&self) -> f64 {
        self.market_price
    }
    pub fn _last_exec(&self) -> i64 {
        self.last_exec
    }

    pub fn _set_starting_capital(&mut self, starting_capital: f64) {
        self.starting_capital = starting_capital;
    }
    pub fn set_capital(&mut self, capital: f64) {
        self.capital = capital;
    }
    pub fn set_last_price(&mut self, lp: f64) {
        self.last_price = lp;
    }
    pub fn set_market_price(&mut self, mp: f64) {
        self.market_price = mp;
    }
    pub fn push_trade(&mut self, t: Trade) {
        self.trades.push(t);
    }

}

impl Trade {
    pub fn new(date: i64, instrument_code: String, price: f64, amount: f64) -> Self {
        Self { date, instrument_code, price, amount }
    }
    pub fn _date(&self) -> i64 {
        self.date
    }
    pub fn _instrument_code(&self) -> &str {
        &self.instrument_code
    }
    pub fn price(&self) -> f64 {
        self.price
    }
    pub fn amount(&self) -> f64 {
        self.amount
    }
}

impl Display for Trade {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {} {} {}", self.date, self.amount, self.instrument_code, self.price)
    }
}
