use once_cell::sync::OnceCell;
use protocol::ConsensusVersions;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use std::time::Duration;

use crate::ConfigResult;

pub type PaysetCommitType = i32;

pub type ConsensusProtocols = HashMap<ConsensusVersions, ConsensusParams>;

static CONSENSUS: OnceCell<ConsensusProtocols> = OnceCell::new();

const ConfigurableConsensusProtocolsFilename: &str = "consensus.json";

#[derive(Debug, Default, Clone)]
struct ConsensusParams {
    // consensus protocol upgrades.  votes for upgrades are collected for
    // upgrade_vote_rounds.  if the number of positive votes is over
    // upgrade_threshold, the proposal is accepted.
    //
    // upgrade_vote_rounds needs to be long enough to collect an
    // accurate sample of participants, and upgrade_threshold needs
    // to be high enough to ensure that there are sufficient participants
    // after the upgrade.
    //
    // a consensus protocol upgrade may specify the delay between its
    // acceptance and its execution.  this gives clients time to notify
    // users.  this delay is specified by the upgrade proposer and must
    // be between min_upgrade_wait_rounds and max_upgrade_wait_rounds (inclusive)
    // in the old protocol's parameters.  note that these parameters refer
    // to the representation of the delay in a block rather than the actual
    // delay: if the specified delay is zero, it is equivalent to,
    // default_upgrade_wait_rounds.
    //
    // the maximum length of a consensus version string is
    // max_version_string_len.
    upgrade_vote_rounds: u64,
    upgrade_threshold: u64,
    default_upgrade_wait_rounds: u64,
    min_upgrade_wait_rounds: u64,
    max_upgrade_wait_rounds: u64,
    max_version_string_len: i32,

    // max_txn_bytes_per_block determines the maximum number of bytes
    // that transactions can take up in a block.  specifically,
    // the sum of the lengths of encodings of each transaction
    // in a block must not exceed max_txn_bytes_per_block.
    max_txn_bytes_per_block: i32,

    // max_txn_bytes_per_block is the maximum size of a transaction's note field.
    max_txn_note_bytes: i32,

    // max_txn_life is how long a transaction can be live for:,
    // the maximum difference between last_valid and first_valid.
    //
    // note that in a protocol upgrade, the ledger must first be upgraded
    // to hold more past blocks for this value to be raised.
    max_txn_life: u64,

    // approved_upgrades describes the upgrade proposals that this protocol
    // implementation will vote for, along with their delay value
    // (in rounds).  a delay value of zero is the same as a delay of
    // default_upgrade_wait_rounds.
    approved_upgrades: HashMap<protocol::ConsensusVersion, u64>,

    // support_genesis_hash indicates support for the genesis_hash
    // fields in transactions (and requires them in blocks).
    support_genesis_hash: bool,

    // require_genesis_hash indicates that genesis_hash must be present
    // in every transaction.
    require_genesis_hash: bool,

    // default_key_dilution specifies the granularity of top-level ephemeral
    // keys. key_dilution is the number of second-level keys in each batch,
    // signed by a top-level "batch" key.  the default value can be
    // overridden in the account state.
    default_key_dilution: u64,

    // min_balance specifies the minimum balance that can appear in
    // an account.  to spend money below min_balance requires issuing
    // an account-closing transaction, which transfers all of the
    // money from the account, and deletes the account state.
    min_balance: u64,

    // min_txn_fee specifies the minimum fee allowed on a transaction.
    // a minimum fee is necessary to prevent do_s. in some sense this is
    // a way of making the spender subsidize the cost of storing this transaction.
    min_txn_fee: u64,

    // enable_fee_pooling specifies that the sum of the fees in a
    // group must exceed one min_txn_fee per txn, rather than check that
    // each txn has a min_fee.
    enable_fee_pooling: bool,

    // enable_app_cost_pooling specifies that the sum of fees for application calls
    // in a group is checked against the sum of the budget for application calls,
    // rather than check each individual app call is within the budget.
    enable_app_cost_pooling: bool,

    // reward_unit specifies the number of micro_algos corresponding to one reward
    // unit.
    //
    // rewards are received by whole reward units.  fractions of
    // reward_units do not receive rewards.
    //
    // ensure both considerations below  are taken into account if reward_unit is planned for change:,
    // 1. reward_units should not be changed without touching all accounts to apply their rewards
    // based on the old reward_units and then use the new reward_units for all subsequent calculations.
    // 2. having a consistent reward_unit is also important for preserving
    // a constant amount of total algos in the system:,
    // the block header tracks how many reward units worth of algos are in existence
    // and have logically received rewards.
    reward_unit: u64,

    // rewards_rate_refresh_interval is the number of rounds after which the
    // rewards level is recomputed for the next rewards_rate_refresh_interval rounds.
    rewards_rate_refresh_interval: u64,

    // seed-related parameters
    seed_lookback: u64, // how many blocks back we use seeds from in sortition. delta_s in the spec,
    seed_refresh_interval: u64, // how often an old block hash is mixed into the seed. delta_r in the spec,

    // ledger retention policy
    max_bal_lookback: u64, // (current round - max_bal_lookback) is the oldest round the ledger must answer balance queries for,

    // sortition threshold factors
    num_proposers: u64,
    soft_committee_size: u64,
    soft_committee_threshold: u64,
    cert_committee_size: u64,
    cert_committee_threshold: u64,
    next_committee_size: u64, // for any non-fpr votes >= deadline step, committee sizes and thresholds are constant,
    next_committee_threshold: u64,
    late_committee_size: u64,
    late_committee_threshold: u64,
    redo_committee_size: u64,
    redo_committee_threshold: u64,
    down_committee_size: u64,
    down_committee_threshold: u64,

    // time for nodes to wait for block proposal headers for period > 0, value should be set to 2 * small_lambda
    agreement_filter_timeout: Duration,
    // time for nodes to wait for block proposal headers for period = 0, value should be configured to suit best case
    // critical path
    agreement_filter_timeout_period0: Duration,

    fast_recovery_lambda: Duration, // time between fast recovery attempts,

    // how to commit to the payset: flat or merkle tree,
    payset_commit: PaysetCommitType,

    max_timestamp_increment: i64, // maximum time between timestamps on successive blocks,

    // support for the efficient encoding in signed_txn_in_block
    support_signed_txn_in_block: bool,

    // force the fee_sink address to be non-participating in the genesis balances.
    force_non_participating_fee_sink: bool,

    // support for apply_data in signed_txn_in_block
    apply_data: bool,

    // track reward distributions in apply_data
    rewards_in_apply_data: bool,

    // domain-separated credentials
    credential_domain_separation_enabled: bool,

    // support for transactions that mark an account non-participating
    support_become_non_participating_transactions: bool,

    // fix the rewards calculation by avoiding subtracting too much from the rewards pool
    pending_residue_rewards: bool,

    // asset support
    asset: bool,

    // max number of assets per account
    max_assets_per_account: i32,

    // max length of asset name
    max_asset_name_bytes: i32,

    // max length of asset unit name
    max_asset_unit_name_bytes: i32,

    // max length of asset url
    max_asset_url_bytes: i32,

    // support sequential transaction counter txn_counter
    txn_counter: bool,

    // transaction groups
    support_tx_groups: bool,

    // max group size
    max_tx_group_size: i32,

    // support for transaction leases
    // note: if fix_transaction_leases is not set, the transaction,
    // leases supported are faulty; specifically, they do not
    // enforce exclusion correctly when the first_valid of
    // transactions do not match.
    support_transaction_leases: bool,
    fix_transaction_leases: bool,

    // 0 for no support, otherwise highest version supported
    logic_sig_version: u64,

    // len(logic_sig.logic) + len(logic_sig.args[*]) must be less than this
    logic_sig_max_size: u64,

    // sum of estimated op cost must be less than this
    logic_sig_max_cost: u64,

    // max decimal precision for assets
    max_asset_decimals: u32,

    // support_rekeying indicates support for account rekeying (the rekey_to and auth_addr fields)
    support_rekeying: bool,

    // application support
    application: bool,

    // max number of application_args for an application_call transaction
    max_app_args: i32,

    // max sum([len(arg) for arg in txn.application_args])
    max_app_total_arg_len: i32,

    // maximum byte len of application approval program or clear state
    // when max_extra_app_program_pages > 0, this is the size of those pages.
    // so two "extra pages" would mean 3*max_app_program_len bytes are available.
    max_app_program_len: i32,

    // maximum total length of an application's programs (approval + clear state)
    // when max_extra_app_program_pages > 0, this is the size of those pages.
    // so two "extra pages" would mean 3*max_app_total_program_len bytes are available.
    max_app_total_program_len: i32,

    // extra length for application program in pages. a page is max_app_program_len bytes
    max_extra_app_program_pages: i32,

    // maximum number of accounts in the application_call accounts field.
    // this determines, in part, the maximum number of balance records
    // accessed by a single transaction
    max_app_txn_accounts: i32,

    // maximum number of app ids in the application_call foreign_apps field.
    // these are the only applications besides the called application for
    // which global state may be read in the transaction
    max_app_txn_foreign_apps: i32,

    // maximum number of asset ids in the application_call foreign_assets
    // field. these are the only assets for which the asset parameters may
    // be read in the transaction
    max_app_txn_foreign_assets: i32,

    // maximum number of "foreign references" (accounts, asa, app)
    // that can be attached to a single app call.
    max_app_total_txn_references: i32,

    // maximum cost of application approval program or clear state program
    max_app_program_cost: i32,

    // maximum length of a key used in an application's global or local
    // key/value store
    max_app_key_len: i32,

    // maximum length of a bytes value used in an application's global or
    // local key/value store
    max_app_bytes_value_len: i32,

    // maximum sum of the lengths of the key and value of one app state entry
    max_app_sum_key_value_lens: i32,

    // maximum number of inner transactions that can be created by an app call.
    // with enable_inner_transaction_pooling, limit is multiplied by max_tx_group_size
    // and enforced over the whole group.
    max_inner_transactions: i32,

    // should the number of inner transactions be pooled across group?
    enable_inner_transaction_pooling: bool,

    // provide greater isolation for clear state programs
    isolate_clear_state: bool,

    // the minimum app version that can be called in an inner transaction
    min_inner_appl_version: u64,

    // maximum number of applications a single account can create and store
    // app_params for at once
    max_apps_created: i32,

    // maximum number of applications a single account can opt in to and
    // store app_local_state for at once
    max_apps_opted_in: i32,

    // flat min_balance requirement for creating a single application and
    // storing its app_params
    app_flat_params_min_balance: u64,

    // flat min_balance requirement for opting in to a single application
    // and storing its app_local_state
    app_flat_opt_in_min_balance: u64,

    // min_balance requirement per key/value entry in local_state or
    // global_state key/value stores, regardless of value type
    schema_min_balance_per_entry: u64,

    // min_balance requirement (in addition to schema_min_balance_per_entry) for
    // integer values stored in local_state or global_state key/value stores
    schema_uint_min_balance: u64,

    // min_balance requirement (in addition to schema_min_balance_per_entry) for
    // []byte values stored in local_state or global_state key/value stores
    schema_bytes_min_balance: u64,

    // maximum number of total key/value pairs allowed by a given
    // local_state_schema (and therefore allowed in local_state)
    max_local_schema_entries: u64,

    // maximum number of total key/value pairs allowed by a given
    // global_state_schema (and therefore allowed in global_state)
    max_global_schema_entries: u64,

    // maximum total minimum balance requirement for an account, used
    // to limit the maximum size of a single balance record
    maximum_minimum_balance: u64,

    // compact_cert_rounds defines the frequency with which compact
    // certificates are generated.  every round that is a multiple
    // of compact_cert_rounds, the block header will include a merkle
    // commitment to the set of online accounts (that can vote after
    // another compact_cert_rounds rounds), and that block will be signed
    // (forming a compact certificate) by the voters from the previous
    // such merkle tree commitment.  a value of zero means no compact
    // certificates.
    compact_cert_rounds: u64,

    // compact_cert_top_voters is a bound on how many online accounts get to
    // participate in forming the compact certificate, by including the
    // top compact_cert_top_voters accounts (by normalized balance) into the
    // merkle commitment.
    compact_cert_top_voters: u64,

    // compact_cert_voters_lookback is the number of blocks we skip before
    // publishing a merkle commitment to the online accounts.  namely,
    // if block number n contains a merkle commitment to the online
    // accounts (which, incidentally, means n%compact_cert_rounds=0),
    // then the balances reflected in that commitment must come from
    // block n-compact_cert_voters_lookback.  this gives each node some
    // time (compact_cert_voters_lookback blocks worth of time) to
    // construct this merkle tree, so as to avoid placing the
    // construction of this merkle tree (and obtaining the requisite
    // accounts and balances) in the critical path.
    compact_cert_voters_lookback: u64,

    // compact_cert_weight_threshold specifies the fraction of top voters weight
    // that must sign the message (block header) for security.  the compact
    // certificate ensures this threshold holds; however, forming a valid
    // compact certificate requires a somewhat higher number of signatures,
    // and the more signatures are collected, the smaller the compact cert
    // can be.
    //
    // this threshold can be thought of as the maximum fraction of
    // malicious weight that compact certificates defend against.
    //
    // the threshold is computed as compact_cert_weight_threshold/(1<<32).
    compact_cert_weight_threshold: u32,

    // compact_cert_sec_kq is the security parameter (k+q) for the compact
    // certificate scheme.
    compact_cert_sec_kq: u64,

    // enable_asset_close_amount adds an extra field to the apply_data. the field contains the amount of the remaining
    // asset that were sent to the close-to address.
    enable_asset_close_amount: bool,

    // update the initial rewards rate calculation to take the reward pool minimum balance into account
    initial_rewards_rate_calculation: bool,

    // no_empty_local_deltas updates how apply_delta.eval_delta.local_deltas are stored
    no_empty_local_deltas: bool,

    // enable_keyreg_coherency_check enable the following extra checks on key registration transactions:,
    // 1. checking that [vote_pk/selection_pk/vote_key_dilution] are all set or all clear.
    // 2. checking that the vote_first is less or equal to vote_last.
    // 3. checking that in the case of going offline, both the vote_first and vote_last are clear.
    // 4. checking that in the case of going online the vote_last is non-zero and greater then the current network round.
    // 5. checking that in the case of going online the vote_first is less or equal to the last_valid+1.
    // 6. checking that in the case of going online the vote_first is less or equal to the next network round.
    enable_keyreg_coherency_check: bool,

    enable_extra_pages_on_app_update: bool,

    // max_proposed_expired_online_accounts is the maximum number of online accounts, which need
    // to be taken offline, that would be proposed to be taken offline.
    max_proposed_expired_online_accounts: i32,

    // enable_account_data_resource_separation enables the support for extended application and asset storage
    // in a separate table.
    enable_account_data_resource_separation: bool,

    //enable_batch_verification enable the use of the batch verification algorithm.
    enable_batch_verification: bool,

    // when rewards rate changes, use the new value immediately.
    rewards_calculation_fix: bool,

    // enable_state_proof_keyreg_check enables the check for state_proof key on key registration
    enable_state_proof_keyreg_check: bool,

    // max_keyreg_valid_period defines the longest period (in rounds) allowed for a keyreg transaction.
    // this number sets a limit to prevent the number of state_proof keys generated by the user from being too large, and also checked by the well_formed method.
    // the hard-limit for number of state_proof keys is derived from the maximum depth allowed for the merkle signature scheme's tree - 2^16.
    // more keys => deeper merkle tree => longer proof required => infeasible for our snark.
    max_keyreg_valid_period: u64,

    // unify_inner_tx_i_ds enables a consistent, unified way of computing inner transaction i_ds
    unify_inner_tx_i_ds: bool,

    // enable_sha256_txn_commitment_header enables the creation of a transaction vector commitment tree using sha256 hash function. (vector commitment extends merkle tree by having a position binding property).
    // this new header is in addition to the existing sha512_256 merkle root.
    // it is useful for verifying transaction on different blockchains, as some may not support sha512_256 opcode natively but sha256 is common.
    enable_sha256_txn_commitment_header: bool,
}

// LoadConfigurableConsensusProtocols loads the configurable protocols from the data directory
pub fn load_configurable_consensus_protocols<P: AsRef<Path>>(
    data_directory: P,
) -> ConfigResult<()> {
    let new_consensus = preload_configurable_consensus_protocols(data_directory)?;
    //CONSENSUS.= new_consensus;
    todo!()
}

// PreloadConfigurableConsensusProtocols loads the configurable protocols from the data directory
// and merge it with a copy of the Consensus map. Then, it returns it to the caller.
fn preload_configurable_consensus_protocols<P: AsRef<Path>>(
    data_directory: P,
) -> ConfigResult<ConsensusProtocols> {
    let consensus_protocol_path = data_directory
        .as_ref()
        .join(ConfigurableConsensusProtocolsFilename);
    match fs::File::open(consensus_protocol_path) {
        Ok(file) => {
            let configurable_consensus = ConsensusProtocols::new();
            //let configurable_consensus = serde_json::from_reader(&file)?;
            return Ok(merge(CONSENSUS.get().unwrap(), &configurable_consensus));
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                return Ok(CONSENSUS.get().unwrap().clone());
            }
            _ => return Err(err.into()),
        },
    }
}

fn merge(_c: &ConsensusProtocols, _t: &ConsensusProtocols) -> ConsensusProtocols {
    todo!()
}

fn init_consensus_protocols(consensus: &mut ConsensusProtocols) {
    let v7 = ConsensusParams {
        upgrade_vote_rounds: 10000,
        upgrade_threshold: 9000,
        default_upgrade_wait_rounds: 10000,
        max_version_string_len: 64,

        min_balance: 10000,
        min_txn_fee: 1000,
        max_txn_life: 1000,
        max_txn_note_bytes: 1024,
        max_txn_bytes_per_block: 1000000,
        default_key_dilution: 10000,

        max_timestamp_increment: 25,

        reward_unit: 1_000_000,
        rewards_rate_refresh_interval: 500_000,

        approved_upgrades: HashMap::new(),

        num_proposers: 30,
        soft_committee_size: 2500,
        soft_committee_threshold: 1870,
        cert_committee_size: 1000,
        cert_committee_threshold: 720,
        next_committee_size: 10000,
        next_committee_threshold: 7750,
        late_committee_size: 10000,
        late_committee_threshold: 7750,
        redo_committee_size: 10000,
        redo_committee_threshold: 7750,
        down_committee_size: 10000,
        down_committee_threshold: 7750,

        agreement_filter_timeout: Duration::from_secs(4),
        agreement_filter_timeout_period0: Duration::from_secs(4),

        fast_recovery_lambda: Duration::from_secs(5 * 60),

        seed_lookback: 2,
        seed_refresh_interval: 100,

        max_bal_lookback: 320,

        max_tx_group_size: 1,
        ..Default::default()
    };

    consensus.insert(ConsensusVersions::V7, v7);

    // v8 uses parameters and a seed derivation policy (the "twin seeds") from georgios' new analysis
    //v8 := v7

    //v8.seed_refresh_interval = 80
    //v8.num_proposers = 9
    //v8.soft_committee_size = 2990
    //v8.soft_committee_threshold = 2267
    //v8.cert_committee_size = 1500
    //v8.cert_committee_threshold = 1112
    //v8.next_committee_size = 5000
    //v8.next_committee_threshold = 3838
    //v8.late_committee_size = 5000
    //v8.late_committee_threshold = 3838
    //v8.redo_committee_size = 5000
    //v8.redo_committee_threshold = 3838
    //v8.down_committee_size = 5000
    //v8.down_committee_threshold = 3838

    //v8.approved_upgrades = map[protocol.consensus_version]uint64{}
    //consensus[protocol.consensus_v8] = v8

    //// v7 can be upgraded to v8.
    //v7.approved_upgrades[protocol.consensus_v8] = 0

    //// v9 increases the minimum balance to 100,000 micro_algos.
    //v9 := v8
    //v9.min_balance = 100000
    //v9.approved_upgrades = map[protocol.consensus_version]uint64{}
    //consensus[protocol.consensus_v9] = v9

    //// v8 can be upgraded to v9.
    //v8.approved_upgrades[protocol.consensus_v9] = 0

    //// v10 introduces fast partition recovery (and also raises num_proposers).
    //v10 := v9
    //v10.num_proposers = 20
    //v10.late_committee_size = 500
    //v10.late_committee_threshold = 320
    //v10.redo_committee_size = 2400
    //v10.redo_committee_threshold = 1768
    //v10.down_committee_size = 6000
    //v10.down_committee_threshold = 4560
    //v10.approved_upgrades = map[protocol.consensus_version]uint64{}
    //consensus[protocol.consensus_v10] = v10

    //// v9 can be upgraded to v10.
    //v9.approved_upgrades[protocol.consensus_v10] = 0

    //// v11 introduces signed_txn_in_block.
    //v11 := v10
    //v11.support_signed_txn_in_block = true
    //v11.payset_commit = payset_commit_flat
    //v11.approved_upgrades = map[protocol.consensus_version]uint64{}
    //consensus[protocol.consensus_v11] = v11

    //// v10 can be upgraded to v11.
    //v10.approved_upgrades[protocol.consensus_v11] = 0

    //// v12 increases the maximum length of a version string.
    //v12 := v11
    //v12.max_version_string_len = 128
    //v12.approved_upgrades = map[protocol.consensus_version]uint64{}
    //consensus[protocol.consensus_v12] = v12

    //// v11 can be upgraded to v12.
    //v11.approved_upgrades[protocol.consensus_v12] = 0

    //// v13 makes the consensus version a meaningful string.
    //v13 := v12
    //v13.approved_upgrades = map[protocol.consensus_version]uint64{}
    //consensus[protocol.consensus_v13] = v13

    //// v12 can be upgraded to v13.
    //v12.approved_upgrades[protocol.consensus_v13] = 0

    //// v14 introduces tracking of closing amounts in apply_data, and enables
    //// genesis_hash in transactions.
    //v14 := v13
    //v14.apply_data = true
    //v14.support_genesis_hash = true
    //v14.approved_upgrades = map[protocol.consensus_version]uint64{}
    //consensus[protocol.consensus_v14] = v14

    //// v13 can be upgraded to v14.
    //v13.approved_upgrades[protocol.consensus_v14] = 0

    //// v15 introduces tracking of reward distributions in apply_data.
    //v15 := v14
    //v15.rewards_in_apply_data = true
    //v15.force_non_participating_fee_sink = true
    //v15.approved_upgrades = map[protocol.consensus_version]uint64{}
    //consensus[protocol.consensus_v15] = v15

    //// v14 can be upgraded to v15.
    //v14.approved_upgrades[protocol.consensus_v15] = 0

    //// v16 fixes domain separation in credentials.
    //v16 := v15
    //v16.credential_domain_separation_enabled = true
    //v16.require_genesis_hash = true
    //v16.approved_upgrades = map[protocol.consensus_version]uint64{}
    //consensus[protocol.consensus_v16] = v16

    //// v15 can be upgraded to v16.
    //v15.approved_upgrades[protocol.consensus_v16] = 0

    //// consensus_v17 points to 'final' spec commit
    //v17 := v16
    //v17.approved_upgrades = map[protocol.consensus_version]uint64{}
    //consensus[protocol.consensus_v17] = v17

    //// v16 can be upgraded to v17.
    //v16.approved_upgrades[protocol.consensus_v17] = 0

    //// consensus_v18 points to reward calculation spec commit
    //v18 := v17
    //v18.pending_residue_rewards = true
    //v18.approved_upgrades = map[protocol.consensus_version]uint64{}
    //v18.txn_counter = true
    //v18.asset = true
    //v18.logic_sig_version = 1
    //v18.logic_sig_max_size = 1000
    //v18.logic_sig_max_cost = 20000
    //v18.max_assets_per_account = 1000
    //v18.support_tx_groups = true
    //v18.max_tx_group_size = 16
    //v18.support_transaction_leases = true
    //v18.support_become_non_participating_transactions = true
    //v18.max_asset_name_bytes = 32
    //v18.max_asset_unit_name_bytes = 8
    //v18.max_asset_url_bytes = 32
    //consensus[protocol.consensus_v18] = v18

    //// consensus_v19 is the official spec commit ( teal, assets, group tx )
    //v19 := v18
    //v19.approved_upgrades = map[protocol.consensus_version]uint64{}

    //consensus[protocol.consensus_v19] = v19

    //// v18 can be upgraded to v19.
    //v18.approved_upgrades[protocol.consensus_v19] = 0
    //// v17 can be upgraded to v19.
    //v17.approved_upgrades[protocol.consensus_v19] = 0

    //// v20 points to adding the precision to the assets.
    //v20 := v19
    //v20.approved_upgrades = map[protocol.consensus_version]uint64{}
    //v20.max_asset_decimals = 19
    //// we want to adjust the upgrade time to be roughly one week.
    //// one week, in term of rounds would be:
    //// 140651 = (7 * 24 * 60 * 60 / 4.3)
    //// for the sake of future manual calculations, we'll round that down
    //// a bit :
    //v20.default_upgrade_wait_rounds = 140000
    //consensus[protocol.consensus_v20] = v20

    //// v19 can be upgraded to v20.
    //v19.approved_upgrades[protocol.consensus_v20] = 0

    //// v21 fixes a bug in credential.lowest_output that would cause larger accounts to be selected to propose disproportionately more often than small accounts
    //v21 := v20
    //v21.approved_upgrades = map[protocol.consensus_version]uint64{}
    //consensus[protocol.consensus_v21] = v21
    //// v20 can be upgraded to v21.
    //v20.approved_upgrades[protocol.consensus_v21] = 0

    //// v22 is an upgrade which allows tuning the number of rounds to wait to execute upgrades.
    //v22 := v21
    //v22.approved_upgrades = map[protocol.consensus_version]uint64{}
    //v22.min_upgrade_wait_rounds = 10000
    //v22.max_upgrade_wait_rounds = 150000
    //consensus[protocol.consensus_v22] = v22

    //// v23 is an upgrade which fixes the behavior of leases so that
    //// it conforms with the intended spec.
    //v23 := v22
    //v23.approved_upgrades = map[protocol.consensus_version]uint64{}
    //v23.fix_transaction_leases = true
    //consensus[protocol.consensus_v23] = v23
    //// v22 can be upgraded to v23.
    //v22.approved_upgrades[protocol.consensus_v23] = 10000
    //// v21 can be upgraded to v23.
    //v21.approved_upgrades[protocol.consensus_v23] = 0

    //// v24 is the stateful teal and rekeying upgrade
    //v24 := v23
    //v24.approved_upgrades = map[protocol.consensus_version]uint64{}
    //v24.logic_sig_version = 2

    //// enable application support
    //v24.application = true

    //// although inners were not allowed yet, this gates downgrade checks, which must be allowed
    //v24.min_inner_appl_version = 6

    //// enable rekeying
    //v24.support_rekeying = true

    //// 100.1 algos (min_balance for creating 1,000 assets)
    //v24.maximum_minimum_balance = 100100000

    //v24.max_app_args = 16
    //v24.max_app_total_arg_len = 2048
    //v24.max_app_program_len = 1024
    //v24.max_app_total_program_len = 2048 // no effect until v28, when max_app_program_len increased
    //v24.max_app_key_len = 64
    //v24.max_app_bytes_value_len = 64
    //v24.max_app_sum_key_value_lens = 128 // set here to have no effect until max_app_bytes_value_len increases

    //// 0.1 algos (same min balance cost as an asset)
    //v24.app_flat_params_min_balance = 100000
    //v24.app_flat_opt_in_min_balance = 100000

    //// can look up sender + 4 other balance records per application txn
    //v24.max_app_txn_accounts = 4

    //// can look up 2 other app creator balance records to see global state
    //v24.max_app_txn_foreign_apps = 2

    //// can look up 2 assets to see asset parameters
    //v24.max_app_txn_foreign_assets = 2

    //// intended to have no effect in v24 (it's set to accounts +
    //// asas + apps). in later vers, it allows increasing the
    //// individual limits while maintaining same max references.
    //v24.max_app_total_txn_references = 8

    //// 64 byte keys @ ~333 micro_algos/byte + delta
    //v24.schema_min_balance_per_entry = 25000

    //// 9 bytes @ ~333 micro_algos/byte + delta
    //v24.schema_uint_min_balance = 3500

    //// 64 byte values @ ~333 micro_algos/byte + delta
    //v24.schema_bytes_min_balance = 25000

    //// maximum number of key/value pairs per local key/value store
    //v24.max_local_schema_entries = 16

    //// maximum number of key/value pairs per global key/value store
    //v24.max_global_schema_entries = 64

    //// maximum cost of approval_program/clear_state_program
    //v24.max_app_program_cost = 700

    //// maximum number of apps a single account can create
    //v24.max_apps_created = 10

    //// maximum number of apps a single account can opt into
    //v24.max_apps_opted_in = 10
    //consensus[protocol.consensus_v24] = v24

    //// v23 can be upgraded to v24, with an update delay of 7 days ( see calculation above )
    //v23.approved_upgrades[protocol.consensus_v24] = 140000

    //// v25 enables asset_close_amount in the apply_data
    //v25 := v24
    //v25.approved_upgrades = map[protocol.consensus_version]uint64{}

    //// enable asset_close_amount field
    //v25.enable_asset_close_amount = true
    //consensus[protocol.consensus_v25] = v25

    //// v26 adds support for teal3
    //v26 := v25
    //v26.approved_upgrades = map[protocol.consensus_version]uint64{}

    //// enable the initial_rewards_rate_calculation fix
    //v26.initial_rewards_rate_calculation = true

    //// enable transaction merkle tree.
    //v26.payset_commit = payset_commit_merkle

    //// enable teal3
    //v26.logic_sig_version = 3

    //consensus[protocol.consensus_v26] = v26

    //// v25 or v24 can be upgraded to v26, with an update delay of 7 days ( see calculation above )
    //v25.approved_upgrades[protocol.consensus_v26] = 140000
    //v24.approved_upgrades[protocol.consensus_v26] = 140000

    //// v27 updates apply_delta.eval_delta.local_deltas format
    //v27 := v26
    //v27.approved_upgrades = map[protocol.consensus_version]uint64{}

    //// enable the apply_delta.eval_delta.local_deltas fix
    //v27.no_empty_local_deltas = true

    //consensus[protocol.consensus_v27] = v27

    //// v26 can be upgraded to v27, with an update delay of 3 days
    //// 60279 = (3 * 24 * 60 * 60 / 4.3)
    //// for the sake of future manual calculations, we'll round that down
    //// a bit :
    //v26.approved_upgrades[protocol.consensus_v27] = 60000

    //// v28 introduces new teal features, larger program size, fee pooling and longer asset max url
    //v28 := v27
    //v28.approved_upgrades = map[protocol.consensus_version]uint64{}

    //// enable teal 4 / avm 0.9
    //v28.logic_sig_version = 4
    //// enable support for larger app program size
    //v28.max_extra_app_program_pages = 3
    //v28.max_app_program_len = 2048
    //// increase asset url length to allow for ipfs ur_ls
    //v28.max_asset_url_bytes = 96
    //// let the bytes value take more space. key+value is still limited to 128
    //v28.max_app_bytes_value_len = 128

    //// individual limits raised
    //v28.max_app_txn_foreign_apps = 8
    //v28.max_app_txn_foreign_assets = 8

    //// max_app_txn_accounts has not been raised yet.  it is already
    //// higher (4) and there is a multiplicative effect in
    //// "reachability" between accounts and creatables, so we
    //// retain 4 x 4 as worst case.

    //v28.enable_fee_pooling = true
    //v28.enable_keyreg_coherency_check = true

    //consensus[protocol.consensus_v28] = v28

    //// v27 can be upgraded to v28, with an update delay of 7 days ( see calculation above )
    //v27.approved_upgrades[protocol.consensus_v28] = 140000

    //// v29 fixes application update by using extra_program_pages in size calculations
    //v29 := v28
    //v29.approved_upgrades = map[protocol.consensus_version]uint64{}

    //// enable extra_program_pages for application update
    //v29.enable_extra_pages_on_app_update = true

    //consensus[protocol.consensus_v29] = v29

    //// v28 can be upgraded to v29, with an update delay of 3 days ( see calculation above )
    //v28.approved_upgrades[protocol.consensus_v29] = 60000

    //// v30 introduces avm 1.0 and teal 5, increases the app opt in limit to 50,
    //// and allows costs to be pooled in grouped stateful transactions.
    //v30 := v29
    //v30.approved_upgrades = map[protocol.consensus_version]uint64{}

    //// enable teal 5 / avm 1.0
    //v30.logic_sig_version = 5

    //// enable app calls to pool budget in grouped transactions
    //v30.enable_app_cost_pooling = true

    //// enable inner transactions, and set maximum number. 0 value is
    //// disabled.  value > 0 also activates storage of creatable i_ds in
    //// apply_data, as that is required to support rest api when inner
    //// transactions are activated.
    //v30.max_inner_transactions = 16

    //// allow 50 app opt ins
    //v30.max_apps_opted_in = 50

    //consensus[protocol.consensus_v30] = v30

    //// v29 can be upgraded to v30, with an update delay of 7 days ( see calculation above )
    //v29.approved_upgrades[protocol.consensus_v30] = 140000

    //v31 := v30
    //v31.approved_upgrades = map[protocol.consensus_version]uint64{}
    //v31.enable_batch_verification = true
    //v31.rewards_calculation_fix = true
    //v31.max_proposed_expired_online_accounts = 32

    //// enable teal 6 / avm 1.1
    //v31.logic_sig_version = 6
    //v31.enable_inner_transaction_pooling = true
    //v31.isolate_clear_state = true

    //// stat proof key registration
    //v31.enable_state_proof_keyreg_check = true

    //// maximum validity period for key registration, to prevent generating too many state_proof keys
    //v31.max_keyreg_valid_period = 256*(1<<16) - 1

    //consensus[protocol.consensus_v31] = v31

    //// v30 can be upgraded to v31, with an update delay of 7 days ( see calculation above )
    //v30.approved_upgrades[protocol.consensus_v31] = 140000

    //v32 := v31
    //v32.approved_upgrades = map[protocol.consensus_version]uint64{}

    //// enable extended application storage; binaries that contain support for this
    //// flag would already be restructuring their internal storage for extended
    //// application storage, and therefore would not produce catchpoints and/or
    //// catchpoint labels prior to this feature being enabled.
    //v32.enable_account_data_resource_separation = true

    //// remove limits on minimum_balance
    //v32.maximum_minimum_balance = 0

    //// remove limits on assets / account.
    //v32.max_assets_per_account = 0

    //// remove limits on maximum number of apps a single account can create
    //v32.max_apps_created = 0

    //// remove limits on maximum number of apps a single account can opt into
    //v32.max_apps_opted_in = 0

    //consensus[protocol.consensus_v32] = v32

    //// v31 can be upgraded to v32, with an update delay of 7 days ( see calculation above )
    //v31.approved_upgrades[protocol.consensus_v32] = 140000

    //// consensus_future is used to test features that are implemented
    //// but not yet released in a production protocol version.
    //v_future := v32
    //v_future.approved_upgrades = map[protocol.consensus_version]uint64{}

    //// filter_timeout for period 0 should take a new optimized, configured value, need to revisit this later
    //v_future.agreement_filter_timeout_period0 = 4 * time.second

    //// enable compact certificates.
    //v_future.compact_cert_rounds = 256
    //v_future.compact_cert_top_voters = 1024 * 1024
    //v_future.compact_cert_voters_lookback = 16
    //v_future.compact_cert_weight_threshold = (1 << 32) * 30 / 100
    //v_future.compact_cert_sec_kq = 128

    //v_future.logic_sig_version = 7
    //v_future.min_inner_appl_version = 4

    //v_future.unify_inner_tx_i_ds = true

    //v_future.enable_sha256_txn_commitment_header = true

    //consensus[protocol.consensus_future] = v_future
}

pub fn init() {
    let _ = CONSENSUS.get_or_init(|| {
        let mut consensus_map = ConsensusProtocols::new();
        consensus_map
    });
}
