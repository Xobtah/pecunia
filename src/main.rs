use std::error::Error;
use std::fs::File;
use std::io::Read;
use log::info;
use serde_json::json;
use crate::market::bitpanda::bitpanda_api::BitPandaApi;
use crate::model::pecunia_configuration::PecuniaConfiguration;
use crate::model::state::State;
use crate::trader::Trader;

mod market;
mod model;
mod trader;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        panic!("USAGE : ./pecunia CFG_FILE_PATH");
    }
    let mut pc_cfg = String::new();
    File::open(&args[1])?.read_to_string(&mut pc_cfg)?;
    let pc_cfg = serde_json::from_str::<PecuniaConfiguration>(&pc_cfg)?;
    log4rs::init_file(&pc_cfg.log4rs_file_path, log4rs_logstash::config::deserializers())?;
    let bp = BitPandaApi::new();
    let mut state = State::from_file(&pc_cfg.state_file_path)?;
    info!("{}", json!({
        "state": state,
        "action": Trader::trade(&bp, &pc_cfg, &mut state)?.name().to_string()
    }));
    state.to_file(&pc_cfg.state_file_path)?;

    Ok(())
}
