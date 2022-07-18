use std::{collections::HashMap, error::Error};

use crate::basics::{self, AccountData, Address};
use protocol::{ConsensusVersion, NetworkId};
use serde::{Deserialize, Serialize};

pub type GenesisResult<T> = Result<T, Box<dyn std::error::Error>>;

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
    pub fn balances(&self) -> GenesisResult<GenesisBalances> {
        let mut gen_alloc = HashMap::new();
        for entry in self.allocation.clone().into_iter() {
            let addr = basics::unmarshal_checksum_address(&entry.address).map_err(
                |err| -> Box<dyn Error> {
                    format!("cannot parse genesis addr {}: {}", &entry.address, err).into()
                },
            )?;
            let is_presesent = gen_alloc.get(&addr).is_some();
            if is_presesent {
                return Err(format!("repeated allocation to {:?}", &addr).into());
            }
            gen_alloc.insert(addr, entry.state);
        }
        let fee_sink = basics::unmarshal_checksum_address(&self.fee_sink).map_err(
            |err| -> Box<dyn Error> {
                format!("cannot parse fee sink addr {:?}: {}", &self.fee_sink, err).into()
            },
        )?;
        let rewards_pool = basics::unmarshal_checksum_address(&self.rewards_pool).map_err(
            |err| -> Box<dyn Error> {
                format!(
                    "cannot parse rewards pool addr {:?}: {}",
                    &self.fee_sink, err
                )
                .into()
            },
        )?;
        Ok(GenesisBalances::new_with_timestamp(
            gen_alloc,
            fee_sink,
            rewards_pool,
            self.timestamp,
        ))
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
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
    pub timestamp: u32,
}

impl GenesisBalances {
    pub fn new_with_timestamp(
        balances: HashMap<Address, AccountData>,
        fee_sink: Address,
        rewards_pool: Address,
        timestamp: u32,
    ) -> Self {
        Self {
            balances,
            fee_sink,
            rewards_pool,
            timestamp,
        }
    }
}
