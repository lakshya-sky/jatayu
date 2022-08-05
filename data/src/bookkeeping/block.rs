use crate::{basics, committee, transactions};

pub type BlockHash = crypto::util::HashDigest;

#[derive(Debug, Default)]
pub struct TxnCommitments {
    pub native_sha512_256_commitment: crypto::util::HashDigest,
    pub sha256_commitment: crypto::util::HashDigest,
}

#[derive(Debug, Default)]
pub struct ParticipationUpdates {
    expired_participation_accounts: Vec<basics::Address>,
}

#[derive(Debug, Default)]
pub struct RewardsState {
    pub fee_sink: basics::Address,
    pub rewards_pool: basics::Address,
    pub rewards_level: u64,
    pub rewards_rate: u64,
    pub rewards_residue: u64,
    pub rewards_recalculation_round: basics::Round,
}

#[derive(Debug, Default)]
pub struct UpgradeVote {
    upgrade_propose: protocol::ConsensusVersion,
    upgrade_delay: basics::Round,
    upgrade_approve: bool,
}

#[derive(Debug, Default)]
pub struct UpgradeState {
    current_protocol: protocol::ConsensusVersion,
    next_protocol: protocol::ConsensusVersion,
    next_protocol_approvals: u64,
    next_protcol_vote_before: basics::Round,
    next_protocol_switch_on: basics::Round,
}

#[derive(Debug, Default)]
pub struct BlockHeader {
    pub round: basics::Round,
    pub branch: BlockHash,
    pub seed: committee::Seed,
    pub txn_commitments: TxnCommitments,
    pub timestamp: u64,
    pub genesis_id: String,
    pub genesis_hash: crypto::util::HashDigest,
    pub rewards_state: RewardsState,
    pub upgrade_state: UpgradeState,
    pub upgrade_vote: UpgradeVote,
    pub txn_counter: u64,
    pub participation_updates: ParticipationUpdates,
}

#[derive(Debug, Default)]
pub struct Block {
    pub header: BlockHeader,
    pub payset: transactions::payset::PaySet,
}
