use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default)]
pub struct PecuniaConfiguration {
    pub state_file_path: String,
    pub log4rs_file_path: String,
    pub threshold: f64,
    pub invest_percent: f64
}
