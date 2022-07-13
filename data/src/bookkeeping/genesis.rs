use crate::basics;
use protocol::{ConsensusVersion, NetworkId};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct Genesis {
    #[serde(rename = "id")]
    schema_id: String,
    network: NetworkId,
    proto: ConsensusVersion,
    #[serde(rename = "alloc")]
    allocation: Vec<GenesisAllocation>,
    #[serde(rename = "rwd")]
    rewards_pool: String,
    #[serde(rename = "fees")]
    fee_sink: String,
    comment: String,
    timestamp: u32,
    #[serde(rename = "devmode")]
    dev_mode: bool,
}

impl Genesis {
    pub fn id(&self) -> String {
        format!("{}-{}", self.network, self.schema_id)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
struct GenesisAllocation {
    #[serde(rename = "addr")]
    address: String,
    comment: String,
    state: basics::AccountData,
}
