use std::error::Error;
use crate::{PecuniaConfiguration, State};
use crate::market::{InstrumentCode};
use crate::market::market::Market;
use crate::trader::{Action, Buy, DoNothing, Sell, UpdateLastPrice};

pub struct Trader;

fn diff(market_price: f64, state_price: f64) -> f64 {
    (market_price - state_price) * 100. / state_price
}

impl Trader {

    fn decision(cfg: &PecuniaConfiguration, s: &State) -> Box<dyn Action> {
        if s.last_price() <= 0. || (s.last_price() < s.market_price() && s.last_trade().is_none()) {
            Box::new(UpdateLastPrice)
        } else if diff(s.market_price(), s.last_price()) <= -cfg.threshold {
            Box::new(Buy)
        } else if s.last_trade().is_some() && diff(s.market_price(), s.last_trade().unwrap().price()) >= cfg.threshold {
            Box::new(Sell)
        } else {
            Box::new(DoNothing)
        }
    }

    // TODO : INCLUDE TIME
    // TODO : DON'T BUY IF NO MONEY
    pub fn trade<M: Market>(m: &M, cfg: &PecuniaConfiguration, state: &mut State) -> Result<Box<dyn Action>, Box<dyn Error>> {
        let pt = m.price_tick(InstrumentCode::BTC_EUR)?;
        state.set_market_price(pt.get(0).unwrap().price.to_string().parse()?);
        let action = Trader::decision(cfg, state);
        action.execute(m, cfg, state)?;
        state.update_last_exec();
        Ok(action)
    }

}

#[cfg(test)]
mod tests {
    use std::error::Error;
    use std::sync::Once;
    use crate::market::bitpanda::model::{Order, PriceTick};
    use crate::market::{InstrumentCode, OrderSide};
    use crate::market::market::Market;
    use crate::{PecuniaConfiguration, State, Trader};
    use crate::market::bitpanda::model::fee_group::{FeeGroup, FeeTier};
    use crate::model::state::Trade;

    static INIT_LOG: fn() = || log4rs::init_file("cfg/log4rs.test.yaml", Default::default()).unwrap();
    static INIT_LOG_ONCE: Once = Once::new();

    #[derive(Default)]
    struct MockMarket {
        price_ticks: Vec<PriceTick>,
        order: Order,
        fee_groups: Vec<FeeGroup>
    }

    impl Market for MockMarket {
        fn price_tick(&self, _instrument_code: InstrumentCode) -> Result<Vec<PriceTick>, Box<dyn Error>> {
            Ok(self.price_ticks.clone())
        }

        fn create_order(&self, _instrument_code: InstrumentCode, _side: OrderSide, _amount: f64) -> Result<Order, Box<dyn Error>> {
            Ok(self.order.clone())
        }

        fn fee_groups(&self) -> Result<Vec<FeeGroup>, Box<dyn Error>> {
            Ok(self.fee_groups.clone())
        }
    }

    #[test]
    fn init_state() {
        INIT_LOG_ONCE.call_once(INIT_LOG);
        // Data
        let price = 1.;
        let market = MockMarket {
            price_ticks: vec!(PriceTick {
                price: price.to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let cfg = PecuniaConfiguration::default();
        let mut state = State::default();

        // Exec
        Trader::trade(&market, &cfg, &mut state).unwrap();

        // Test
        assert_eq!(state.last_price(), price);
    }

    #[test]
    fn market_price_higher() {
        INIT_LOG_ONCE.call_once(INIT_LOG);
        // Data
        let price = 2.;
        let market = MockMarket {
            price_ticks: vec!(PriceTick {
                price: price.to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let cfg = PecuniaConfiguration::default();
        let mut state = State::new(0.);
        state.set_last_price(1.);

        // Exec
        Trader::trade(&market, &cfg, &mut state).unwrap();

        // Test
        assert_eq!(state.last_price(), price);
    }

    #[test]
    fn nothing_to_do() {
        INIT_LOG_ONCE.call_once(INIT_LOG);
        // Data
        let market = MockMarket {
            price_ticks: vec!(PriceTick {
                price: 99.0.to_string(),
                ..Default::default()
            }),
            ..Default::default()
        };
        let cfg = PecuniaConfiguration {
            threshold: 5.,
            ..Default::default()
        };
        let last_price = 100.;
        let mut state = State::new(0.);
        state.set_last_price(last_price);

        // Exec
        Trader::trade(&market, &cfg, &mut state).unwrap();

        // Test
        assert_eq!(state.last_price(), last_price);
    }

    #[test]
    fn time_to_buy() {
        INIT_LOG_ONCE.call_once(INIT_LOG);
        // Data
        let price = 95.;
        let fee = 0.1;
        let market = MockMarket {
            price_ticks: vec!(PriceTick {
                price: price.to_string(),
                ..Default::default()
            }),
            fee_groups: vec!(FeeGroup {
                fee_tiers: vec!(
                    FeeTier {
                        volume: "0.0".to_owned(),
                        taker_fee: fee.to_string(),
                        ..Default::default()
                    }
                ),
                ..Default::default()
            }),
            ..Default::default()
        };
        let cfg = PecuniaConfiguration {
            threshold: 5.,
            invest_percent: 10.,
            ..Default::default()
        };
        let mut state = State::new(100.);
        state.set_last_price(100.);

        // Exec
        Trader::trade(&market, &cfg, &mut state).unwrap();

        // Test
        assert_eq!(state.last_price(), price);
        assert_eq!(state.trades_nb(), 1);
        assert_eq!(state.last_trade().unwrap().price(), price);
        let mut amount = state.starting_capital() / cfg.invest_percent / price;
        amount = amount - amount * fee;
        assert_eq!(state.last_trade().unwrap().amount(), amount);
    }

    #[test]
    fn time_to_sell() {
        INIT_LOG_ONCE.call_once(INIT_LOG);
        // Data
        let price = 105.;
        let fee = 0.1;
        let market = MockMarket {
            price_ticks: vec!(PriceTick {
                price: price.to_string(),
                ..Default::default()
            }),
            fee_groups: vec!(FeeGroup {
                fee_tiers: vec!(
                    FeeTier {
                        volume: "0.0".to_owned(),
                        taker_fee: fee.to_string(),
                        ..Default::default()
                    }
                ),
                ..Default::default()
            }),
            ..Default::default()
        };
        let cfg = PecuniaConfiguration {
            threshold: 5.,
            invest_percent: 10.,
            ..Default::default()
        };
        let mut state = State::new(100.);
        state.set_last_price(100.);
        state.set_capital(0.);
        state.push_trade(Trade::new(0, String::default(), 100., 1.));

        // Exec
        Trader::trade(&market, &cfg, &mut state).unwrap();

        // Test
        assert_eq!(state.last_price(), price);
        assert_eq!(state.trades_nb(), 0);
        assert_eq!(state.capital(), 105. - 105. * fee);
    }

}
