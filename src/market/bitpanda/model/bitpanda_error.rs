use std::error::Error;
use std::fmt::{Debug, Display, Formatter};
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BitPandaError {
    pub error: String,
}

impl BitPandaError {
    pub fn from(err: &str) -> Self {
        BitPandaError {
            error: String::from(err)
        }
    }
}

impl Debug for BitPandaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Display for BitPandaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.error)
    }
}

impl Error for BitPandaError {}
