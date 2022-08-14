use std::{collections::HashMap, error::Error};

use crate::{
    basics::{self, AccountData, Address},
    bookkeeping::block::{self, RewardsState},
};
use crypto::util::HashDigest;
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

impl msgp::Marshaler for Genesis {
    fn marshal_msg(&self, _buf: Option<Vec<u8>>) -> Vec<u8> {
        rmp_serde::to_vec(self).unwrap()
    }
}

impl crypto::util::MsgpHashable for Genesis {
    fn hash_id(&self) -> protocol::HashId {
        protocol::GENESIS
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

pub fn make_genesis_block(
    proto: ConsensusVersion,
    genesis_bal: GenesisBalances,
    genesis_id: String,
    genesis_hash: HashDigest,
) -> GenesisResult<block::Block> {
    let consensus = match config::consensus::CONSENSUS.get().unwrap().read() {
        Ok(p) => p,
        Err(e) => return Err(format!("unable to get protocol map {:?}", e).into()),
    };
    let params = match consensus.get(&proto) {
        Some(p) => p,
        None => return Err(format!("unsupported protocol {}", proto).into()),
    };

    let mut genesis_rewards_state = RewardsState {
        fee_sink: genesis_bal.fee_sink,
        rewards_pool: genesis_bal.rewards_pool,
        rewards_recalculation_round: params.rewards_rate_refresh_interval,
        ..Default::default()
    };
    let initial_rewards = genesis_bal
        .balances
        .get(&genesis_bal.rewards_pool)
        .unwrap()
        .microalgos
        .0;
    if params.initial_rewards_rate_calculation {
        genesis_rewards_state.rewards_rate = initial_rewards.saturating_sub(params.min_balance)
            / params.rewards_rate_refresh_interval;
    } else {
        genesis_rewards_state.rewards_rate = initial_rewards / params.rewards_rate_refresh_interval;
    }
    let mut blk = block::Block {
        //header: block::BlockHeader {
        //    seed: committee::Seed::from(genesis_hash),
        //    txn_commitments: block::TxnCommitments{
        //        native_sha512_256_commitment:
        //    },
        //    timestamp: (),
        //    genesis_id: (),
        //    genesis_hash: (),
        //    rewards_state: (),
        //    upgrade_state: (),
        //    upgrade_vote: (),
        //    txn_counter: (),
        //    participation_updates: (),
        //    ..Default::default()
        //},
        ..Default::default()
    };
    if params.support_genesis_hash {
        blk.header.genesis_hash = genesis_hash;
    }

    todo!();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn marshal_unmarshal_genesis_allocation() {
        let gen_alloc = GenesisAllocation::default();
        dbg!(hex::encode(rmp_serde::to_vec_named(&gen_alloc).unwrap()));
    }
}
