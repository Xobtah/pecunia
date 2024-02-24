use std::error::Error;
use crate::market::market::Market;
use crate::{PecuniaConfiguration, State};

pub struct UpdateLastPrice;
pub struct Buy;
pub struct Sell;
pub struct DoNothing;

pub trait Action {
    fn execute(&self, m: &dyn Market, cfg: &PecuniaConfiguration, s: &mut State) -> Result<(), Box<dyn Error>>;
    fn name(&self) -> &str;
}

fn get_fee(m: &dyn Market) -> Result<f64, Box<dyn Error>> {
    Ok(m.fee_groups()?.get(0).unwrap().fee_tiers.iter().filter(|f| f.volume.eq("0.0")).next().unwrap().taker_fee.parse()?)
}

impl Action for UpdateLastPrice {
    fn execute(&self, _: &dyn Market, _: &PecuniaConfiguration, s: &mut State) -> Result<(), Box<dyn Error>> {
        s.set_last_price(s.market_price());
        Ok(())
    }

    fn name(&self) -> &str {
        "UpdateLastPrice"
    }
}

impl Action for Buy {
    fn execute(&self, m: &dyn Market, cfg: &PecuniaConfiguration, s: &mut State) -> Result<(), Box<dyn Error>> {
        let fee: f64 = get_fee(m)?;
        //m.create_order(InstrumentCode::BTC_EUR, OrderSide::BUY, amount)?;
        s.buy(s.market_price(), fee, cfg.invest_percent);
        Ok(())
    }

    fn name(&self) -> &str {
        "Buy"
    }
}

impl Action for Sell {
    fn execute(&self, m: &dyn Market, _: &PecuniaConfiguration, s: &mut State) -> Result<(), Box<dyn Error>> {
        //let trade = s.last_trade();
        //m.create_order(InstrumentCode::BTC_EUR, OrderSide::SELL, trade.amount)?;
        let fee: f64 = get_fee(m)?;
        s.sell(s.market_price(), fee);
        Ok(())
    }

    fn name(&self) -> &str {
        "Sell"
    }
}

impl Action for DoNothing {
    fn execute(&self, _: &dyn Market, _: &PecuniaConfiguration, _: &mut State) -> Result<(), Box<dyn Error>> {
        Ok(())
    }

    fn name(&self) -> &str {
        "DoNothing"
    }
}
