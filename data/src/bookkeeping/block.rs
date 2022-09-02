use crate::{basics, committee, transactions};
use serde::{Deserialize, Serialize};

pub type BlockHash = crypto::util::HashDigest;

#[skip_serializing_default]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct TxnCommitments {
    pub native_sha512_256_commitment: crypto::util::HashDigest,
    pub sha256_commitment: crypto::util::HashDigest,
}

#[skip_serializing_default]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct ParticipationUpdates {
    expired_participation_accounts: Vec<basics::Address>,
}

#[skip_serializing_default]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RewardsState {
    pub fee_sink: basics::Address,
    pub rewards_pool: basics::Address,
    pub rewards_level: u64,
    pub rewards_rate: u64,
    pub rewards_residue: u64,
    pub rewards_recalculation_round: basics::Round,
}

#[skip_serializing_default]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpgradeVote {
    upgrade_propose: protocol::ConsensusVersion,
    upgrade_delay: basics::Round,
    upgrade_approve: bool,
}

#[skip_serializing_default]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct UpgradeState {
    current_protocol: protocol::ConsensusVersion,
    next_protocol: protocol::ConsensusVersion,
    next_protocol_approvals: u64,
    next_protcol_vote_before: basics::Round,
    next_protocol_switch_on: basics::Round,
}

#[skip_serializing_default]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct BlockHeader {
    #[serde(rename="rnd")]
    pub round: basics::Round,
    #[serde(rename="prev")]
    pub branch: BlockHash,
    #[serde(rename="seed")]
    pub seed: committee::Seed,
    #[serde(flatten)]
    pub txn_commitments: TxnCommitments,
    #[serde(rename="ts")]
    pub timestamp: u32,
    #[serde(rename="gen")]
    pub genesis_id: String,
    #[serde(rename="gh")]
    pub genesis_hash: crypto::util::HashDigest,
    #[serde(flatten)]
    pub rewards_state: RewardsState,
    #[serde(flatten)]
    pub upgrade_state: UpgradeState,
    #[serde(flatten)]
    pub upgrade_vote: UpgradeVote,
    #[serde(rename="tc")]
    pub txn_counter: u64,
    #[serde(flatten)]
    pub participation_updates: ParticipationUpdates,
}

#[skip_serializing_default]
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Block {
    #[serde(flatten)]
    pub header: BlockHeader,
    #[serde(rename = "txns")]
    #[serialize_always]
    pub payset: transactions::payset::PaySet,
}
