use std::collections::HashMap;

use crate::basics::{self, AccountData, Address};
use protocol::{ConsensusVersion, NetworkId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Genesis {
    #[serde(rename = "id")]
    pub schema_id: String,
    pub network: NetworkId,
    pub proto: ConsensusVersion,
    #[serde(rename = "alloc")]
    pub allocation: Vec<GenesisAllocation>,
    #[serde(rename = "rwd")]
    pub rewards_pool: String,
    #[serde(rename = "fees")]
    pub fee_sink: String,
    pub comment: String,
    pub timestamp: u32,
    #[serde(rename = "devmode")]
    pub dev_mode: bool,
}

impl Genesis {
    pub fn id(&self) -> String {
        format!("{}-{}", self.network, self.schema_id)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct GenesisAllocation {
    #[serde(rename = "addr")]
    pub address: String,
    pub comment: String,
    pub state: basics::AccountData,
}

#[derive(Debug, Default)]
pub struct GenesisBalances {
    pub balances: HashMap<Address, AccountData>,
    pub fee_sink: Address,
    pub rewards_pool: Address,
    pub timestamp: i64,
}
