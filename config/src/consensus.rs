use crate::ConfigResult;
use once_cell::sync::OnceCell;
use protocol::*;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use std::collections::HashMap;
use std::fs;
use std::ops::Deref;
use std::path::Path;
use std::sync::RwLock;
use std::time::Duration;

#[derive(Clone, Copy, Debug, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum PaysetCommitType {
    #[default]
    PaysetCommitUnsupported,
    PaysetCommitFlat,
    PaysetCommitMerkle,
}

pub type ConsensusProtocols = HashMap<ConsensusVersion, ConsensusParams>;
pub static CONSENSUS: OnceCell<RwLock<ConsensusProtocols>> = OnceCell::new();
const ConfigurableConsensusProtocolsFilename: &str = "consensus.json";

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct ConsensusParams {
    pub upgrade_vote_rounds: u64,
    pub upgrade_threshold: u64,
    pub default_upgrade_wait_rounds: u64,
    pub min_upgrade_wait_rounds: u64,
    pub max_upgrade_wait_rounds: u64,
    pub max_version_string_len: i32,
    pub max_txn_bytes_per_block: i32,
    pub max_txn_note_bytes: i32,
    pub max_txn_life: u64,
    pub approved_upgrades: HashMap<ConsensusVersion, u64>,
    pub support_genesis_hash: bool,
    pub require_genesis_hash: bool,
    pub default_key_dilution: u64,
    pub min_balance: u64,
    pub min_txn_fee: u64,
    pub enable_fee_pooling: bool,
    pub enable_app_cost_pooling: bool,
    pub reward_unit: u64,
    pub rewards_rate_refresh_interval: u64,
    pub seed_lookback: u64,
    pub seed_refresh_interval: u64,
    pub max_bal_lookback: u64,
    pub num_proposers: u64,
    pub soft_committee_size: u64,
    pub soft_committee_threshold: u64,
    pub cert_committee_size: u64,
    pub cert_committee_threshold: u64,
    pub next_committee_size: u64,
    pub next_committee_threshold: u64,
    pub late_committee_size: u64,
    pub late_committee_threshold: u64,
    pub redo_committee_size: u64,
    pub redo_committee_threshold: u64,
    pub down_committee_size: u64,
    pub down_committee_threshold: u64,
    #[serde(with = "from_nano_sec")]
    pub agreement_filter_timeout: Duration,
    #[serde(with = "from_nano_sec")]
    pub agreement_filter_timeout_period0: Duration,
    #[serde(with = "from_nano_sec")]
    pub fast_recovery_lambda: Duration,
    pub payset_commit: PaysetCommitType,
    pub max_timestamp_increment: i64,
    pub support_signed_txn_in_block: bool,
    pub force_non_participating_fee_sink: bool,
    pub apply_data: bool,
    pub rewards_in_apply_data: bool,
    pub credential_domain_separation_enabled: bool,
    pub support_become_non_participating_transactions: bool,
    pub pending_residue_rewards: bool,
    pub asset: bool,
    pub max_assets_per_account: i32,
    pub max_asset_name_bytes: i32,
    pub max_asset_unit_name_bytes: i32,
    pub max_asset_url_bytes: i32,
    pub txn_counter: bool,
    pub support_tx_groups: bool,
    pub max_tx_group_size: i32,
    pub support_transaction_leases: bool,
    pub fix_transaction_leases: bool,
    pub logic_sig_version: u64,
    pub logic_sig_max_size: u64,
    pub logic_sig_max_cost: u64,
    pub max_asset_decimals: u32,
    pub support_rekeying: bool,
    pub application: bool,
    pub max_app_args: i32,
    pub max_app_total_arg_len: i32,
    pub max_app_program_len: i32,
    pub max_app_total_program_len: i32,
    pub max_extra_app_program_pages: i32,
    pub max_app_txn_accounts: i32,
    pub max_app_txn_foreign_apps: i32,
    pub max_app_txn_foreign_assets: i32,
    pub max_app_total_txn_references: i32,
    pub max_app_program_cost: i32,
    pub max_app_key_len: i32,
    pub max_app_bytes_value_len: i32,
    pub max_app_sum_key_value_lens: i32,
    pub max_inner_transactions: i32,
    pub enable_inner_transaction_pooling: bool,
    pub isolate_clear_state: bool,
    pub min_inner_appl_version: u64,
    pub max_apps_created: i32,
    pub max_apps_opted_in: i32,
    pub app_flat_params_min_balance: u64,
    pub app_flat_opt_in_min_balance: u64,
    pub schema_min_balance_per_entry: u64,
    pub schema_uint_min_balance: u64,
    pub schema_bytes_min_balance: u64,
    pub max_local_schema_entries: u64,
    pub max_global_schema_entries: u64,
    pub maximum_minimum_balance: u64,
    pub compact_cert_rounds: u64,
    pub compact_cert_top_voters: u64,
    pub compact_cert_voters_lookback: u64,
    pub compact_cert_weight_threshold: u64,
    pub compact_cert_sec_kq: u64,
    pub enable_asset_close_amount: bool,
    pub initial_rewards_rate_calculation: bool,
    pub no_empty_local_deltas: bool,
    pub enable_keyreg_coherency_check: bool,
    pub enable_extra_pages_on_app_update: bool,
    pub max_proposed_expired_online_accounts: i32,
    pub enable_account_data_resource_separation: bool,
    pub enable_batch_verification: bool,
    pub rewards_calculation_fix: bool,
    pub enable_state_proof_keyreg_check: bool,
    pub max_keyreg_valid_period: u64,
    pub unify_inner_tx_i_ds: bool,
    pub enable_sha256_txn_commitment_header: bool,
}

mod from_nano_sec {
    use core::time::Duration;
    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};
    pub fn serialize<S: Serializer>(v: &Duration, s: S) -> Result<S::Ok, S::Error> {
        v.as_nanos().serialize(s)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Duration, D::Error> {
        Ok(Duration::from_nanos(u64::deserialize(d)?))
    }
}

pub fn load_configurable_consensus_protocols<P: AsRef<Path>>(
    data_directory: P,
) -> ConfigResult<()> {
    let new_consensus = preload_configurable_consensus_protocols(data_directory)?;
    let mut w = CONSENSUS.get().unwrap().write()?;
    *w = new_consensus;
    Ok(())
}

fn preload_configurable_consensus_protocols<P: AsRef<Path>>(
    data_directory: P,
) -> ConfigResult<ConsensusProtocols> {
    let consensus_protocol_path = data_directory
        .as_ref()
        .join(ConfigurableConsensusProtocolsFilename);
    match fs::File::open(consensus_protocol_path) {
        Ok(file) => {
            let configurable_consensus = serde_json::from_reader(file)?;
            return Ok(merge(
                CONSENSUS.get().unwrap().read()?.deref(),
                configurable_consensus,
            ));
        }
        Err(err) => match err.kind() {
            std::io::ErrorKind::NotFound => {
                return Ok(CONSENSUS.get().unwrap().read()?.clone());
            }
            _ => return Err(err.into()),
        },
    }
}

fn merge(c: &ConsensusProtocols, t: ConsensusProtocols) -> ConsensusProtocols {
    let mut static_consensus = c.clone();
    for (consensus_version, consensus_params) in t {
        if consensus_params.approved_upgrades.len() == 0 {
            for (c_ver, c_param) in static_consensus.clone() {
                if c_ver.eq(&consensus_version) {
                    static_consensus.remove(&c_ver);
                } else if c_param.approved_upgrades.get(&consensus_version).is_some() {
                    static_consensus
                        .get_mut(&c_ver)
                        .unwrap()
                        .approved_upgrades
                        .remove(&consensus_version);
                }
            }
        } else {
            static_consensus.insert(consensus_version.clone(), consensus_params);
        }
    }
    static_consensus
}

fn init_consensus_protocols(consensus: &mut ConsensusProtocols) {
    let mut v7 = ConsensusParams {
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
    let mut v8 = v7.clone();
    v8.seed_refresh_interval = 80;
    v8.num_proposers = 9;
    v8.soft_committee_size = 2990;
    v8.soft_committee_threshold = 2267;
    v8.cert_committee_size = 1500;
    v8.cert_committee_threshold = 1112;
    v8.next_committee_size = 5000;
    v8.next_committee_threshold = 3838;
    v8.late_committee_size = 5000;
    v8.late_committee_threshold = 3838;
    v8.redo_committee_size = 5000;
    v8.redo_committee_threshold = 3838;
    v8.down_committee_size = 5000;
    v8.down_committee_threshold = 3838;
    v8.approved_upgrades = Default::default();
    v7.approved_upgrades.insert(CONSENSUS_V8.to_string(), 0);
    let mut v9 = v8.clone();
    v9.min_balance = 100000;
    v9.approved_upgrades = Default::default();
    v8.approved_upgrades.insert(CONSENSUS_V9.to_string(), 0);
    let mut v10 = v9.clone();
    v10.num_proposers = 20;
    v10.late_committee_size = 500;
    v10.late_committee_threshold = 320;
    v10.redo_committee_size = 2400;
    v10.redo_committee_threshold = 1768;
    v10.down_committee_size = 6000;
    v10.down_committee_threshold = 4560;
    v10.approved_upgrades = Default::default();
    v9.approved_upgrades.insert(CONSENSUS_V10.to_string(), 0);
    let mut v11 = v10.clone();
    v11.support_signed_txn_in_block = true;
    v11.payset_commit = PaysetCommitType::PaysetCommitFlat;
    v11.approved_upgrades = Default::default();
    v10.approved_upgrades.insert(CONSENSUS_V11.to_string(), 0);
    let mut v12 = v11.clone();
    v12.max_version_string_len = 128;
    v12.approved_upgrades = Default::default();
    v11.approved_upgrades.insert(CONSENSUS_V12.to_string(), 0);
    let mut v13 = v12.clone();
    v13.approved_upgrades = Default::default();
    v12.approved_upgrades.insert(CONSENSUS_V13.to_string(), 0);
    let mut v14 = v13.clone();
    v14.apply_data = true;
    v14.support_genesis_hash = true;
    v14.approved_upgrades = Default::default();
    v13.approved_upgrades.insert(CONSENSUS_V14.to_string(), 0);
    let mut v15 = v14.clone();
    v15.rewards_in_apply_data = true;
    v15.force_non_participating_fee_sink = true;
    v15.approved_upgrades = Default::default();
    v14.approved_upgrades.insert(CONSENSUS_V15.to_string(), 0);
    let mut v16 = v15.clone();
    v16.credential_domain_separation_enabled = true;
    v16.require_genesis_hash = true;
    v16.approved_upgrades = Default::default();
    v15.approved_upgrades.insert(CONSENSUS_V16.to_string(), 0);
    let mut v17 = v16.clone();
    v17.approved_upgrades = Default::default();
    v16.approved_upgrades.insert(CONSENSUS_V17.to_string(), 0);
    let mut v18 = v17.clone();
    v18.pending_residue_rewards = true;
    v18.approved_upgrades = Default::default();
    v18.txn_counter = true;
    v18.asset = true;
    v18.logic_sig_version = 1;
    v18.logic_sig_max_size = 1000;
    v18.logic_sig_max_cost = 20000;
    v18.max_assets_per_account = 1000;
    v18.support_tx_groups = true;
    v18.max_tx_group_size = 16;
    v18.support_transaction_leases = true;
    v18.support_become_non_participating_transactions = true;
    v18.max_asset_name_bytes = 32;
    v18.max_asset_unit_name_bytes = 8;
    v18.max_asset_url_bytes = 32;
    let mut v19 = v18.clone();
    v19.approved_upgrades = Default::default();
    v18.approved_upgrades.insert(CONSENSUS_V19.to_string(), 0);
    v17.approved_upgrades.insert(CONSENSUS_V19.to_string(), 0);
    let mut v20 = v19.clone();
    v20.approved_upgrades = Default::default();
    v20.max_asset_decimals = 19;
    v20.default_upgrade_wait_rounds = 140000;
    v19.approved_upgrades.insert(CONSENSUS_V20.to_string(), 0);
    let mut v21 = v20.clone();
    v21.approved_upgrades = Default::default();
    v20.approved_upgrades.insert(CONSENSUS_V21.to_string(), 0);
    let mut v22 = v21.clone();
    v22.approved_upgrades = Default::default();
    v22.min_upgrade_wait_rounds = 10000;
    v22.max_upgrade_wait_rounds = 150000;
    let mut v23 = v22.clone();
    v23.approved_upgrades = Default::default();
    v23.fix_transaction_leases = true;
    v22.approved_upgrades
        .insert(CONSENSUS_V23.to_string(), 10000);
    v21.approved_upgrades.insert(CONSENSUS_V23.to_string(), 0);
    let mut v24 = v23.clone();
    v24.approved_upgrades = Default::default();
    v24.logic_sig_version = 2;
    v24.application = true;
    v24.min_inner_appl_version = 6;
    v24.support_rekeying = true;
    v24.maximum_minimum_balance = 100100000;
    v24.max_app_args = 16;
    v24.max_app_total_arg_len = 2048;
    v24.max_app_program_len = 1024;
    v24.max_app_total_program_len = 2048;
    v24.max_app_key_len = 64;
    v24.max_app_bytes_value_len = 64;
    v24.max_app_sum_key_value_lens = 128;
    v24.app_flat_params_min_balance = 100000;
    v24.app_flat_opt_in_min_balance = 100000;
    v24.max_app_txn_accounts = 4;
    v24.max_app_txn_foreign_apps = 2;
    v24.max_app_txn_foreign_assets = 2;
    v24.max_app_total_txn_references = 8;
    v24.schema_min_balance_per_entry = 25000;
    v24.schema_uint_min_balance = 3500;
    v24.schema_bytes_min_balance = 25000;
    v24.max_local_schema_entries = 16;
    v24.max_global_schema_entries = 64;
    v24.max_app_program_cost = 700;
    v24.max_apps_created = 10;
    v24.max_apps_opted_in = 10;
    v23.approved_upgrades
        .insert(CONSENSUS_V24.to_string(), 1400000);
    let mut v25 = v24.clone();
    v25.approved_upgrades = Default::default();
    v25.enable_asset_close_amount = true;
    let mut v26 = v25.clone();
    v26.approved_upgrades = Default::default();
    v26.initial_rewards_rate_calculation = true;
    v26.payset_commit = PaysetCommitType::PaysetCommitMerkle;
    v26.logic_sig_version = 3;
    v25.approved_upgrades
        .insert(CONSENSUS_V26.to_string(), 140000);
    v24.approved_upgrades
        .insert(CONSENSUS_V26.to_string(), 140000);
    let mut v27 = v26.clone();
    v27.approved_upgrades = Default::default();
    v27.no_empty_local_deltas = true;
    v26.approved_upgrades
        .insert(CONSENSUS_V27.to_string(), 60000);
    let mut v28 = v27.clone();
    v28.approved_upgrades = Default::default();
    v28.logic_sig_version = 4;
    v28.max_extra_app_program_pages = 3;
    v28.max_app_program_len = 2048;
    v28.max_asset_url_bytes = 96;
    v28.max_app_bytes_value_len = 128;
    v28.max_app_txn_foreign_apps = 8;
    v28.max_app_txn_foreign_assets = 8;
    v28.enable_fee_pooling = true;
    v28.enable_keyreg_coherency_check = true;
    v27.approved_upgrades
        .insert(CONSENSUS_V28.to_string(), 140000);
    let mut v29 = v28.clone();
    v29.approved_upgrades = Default::default();
    v29.enable_extra_pages_on_app_update = true;
    v28.approved_upgrades
        .insert(CONSENSUS_V29.to_string(), 60000);
    let mut v30 = v29.clone();
    v30.approved_upgrades = Default::default();
    v30.logic_sig_version = 5;
    v30.enable_app_cost_pooling = true;
    v30.max_inner_transactions = 16;
    v30.max_apps_opted_in = 50;
    v29.approved_upgrades
        .insert(CONSENSUS_V30.to_string(), 140000);
    let mut v31 = v30.clone();
    v31.approved_upgrades = Default::default();
    v31.enable_batch_verification = true;
    v31.rewards_calculation_fix = true;
    v31.max_proposed_expired_online_accounts = 32;
    v31.logic_sig_version = 6;
    v31.enable_inner_transaction_pooling = true;
    v31.isolate_clear_state = true;
    v31.enable_state_proof_keyreg_check = true;
    v31.max_keyreg_valid_period = 256 * (1 << 16) - 1;
    v30.approved_upgrades
        .insert(CONSENSUS_V31.to_string(), 140000);
    let mut v32 = v31.clone();
    v32.approved_upgrades = Default::default();
    v32.enable_account_data_resource_separation = true;
    v32.maximum_minimum_balance = 0;
    v32.max_assets_per_account = 0;
    v32.max_apps_created = 0;
    v32.max_apps_opted_in = 0;
    v31.approved_upgrades
        .insert(CONSENSUS_V32.to_string(), 140000);
    let mut v_future = v32.clone();
    v_future.approved_upgrades = Default::default();
    v_future.agreement_filter_timeout_period0 = Duration::from_secs(4);
    v_future.compact_cert_rounds = 256;
    v_future.compact_cert_top_voters = 1024 * 1024;
    v_future.compact_cert_voters_lookback = 16;
    v_future.compact_cert_weight_threshold = (1u64 << 32) * 30 / 100;
    v_future.compact_cert_sec_kq = 128;
    v_future.logic_sig_version = 7;
    v_future.min_inner_appl_version = 4;
    v_future.unify_inner_tx_i_ds = true;
    v_future.enable_sha256_txn_commitment_header = true;
    consensus.insert(CONSENSUS_V7.to_string(), v7);
    consensus.insert(CONSENSUS_V8.to_string(), v8);
    consensus.insert(CONSENSUS_V9.to_string(), v9);
    consensus.insert(CONSENSUS_V10.to_string(), v10);
    consensus.insert(CONSENSUS_V11.to_string(), v11);
    consensus.insert(CONSENSUS_V12.to_string(), v12);
    consensus.insert(CONSENSUS_V13.to_string(), v13);
    consensus.insert(CONSENSUS_V14.to_string(), v14);
    consensus.insert(CONSENSUS_V15.to_string(), v15);
    consensus.insert(CONSENSUS_V16.to_string(), v16);
    consensus.insert(CONSENSUS_V17.to_string(), v17);
    consensus.insert(CONSENSUS_V18.to_string(), v18);
    consensus.insert(CONSENSUS_V19.to_string(), v19);
    consensus.insert(CONSENSUS_V20.to_string(), v20);
    consensus.insert(CONSENSUS_V21.to_string(), v21);
    consensus.insert(CONSENSUS_V22.to_string(), v22);
    consensus.insert(CONSENSUS_V23.to_string(), v23);
    consensus.insert(CONSENSUS_V24.to_string(), v24);
    consensus.insert(CONSENSUS_V25.to_string(), v25);
    consensus.insert(CONSENSUS_V26.to_string(), v26);
    consensus.insert(CONSENSUS_V27.to_string(), v27);
    consensus.insert(CONSENSUS_V28.to_string(), v28);
    consensus.insert(CONSENSUS_V29.to_string(), v29);
    consensus.insert(CONSENSUS_V30.to_string(), v30);
    consensus.insert(CONSENSUS_V31.to_string(), v31);
    consensus.insert(CONSENSUS_V32.to_string(), v32);
    consensus.insert(CONSENSUS_VFUTURE.to_string(), v_future);
}
fn check_set_max<T: PartialOrd>(v: T, c: &mut T) {
    if v > *c {
        *c = v;
    }
}
static MAX_VOTE_THRESHOLD: OnceCell<u64> = OnceCell::new();
static MAX_EVAL_DELTA_KEYS: OnceCell<i32> = OnceCell::new();
static MAX_EVAL_DELTA_ACCOUNTS: OnceCell<i32> = OnceCell::new();
static MAX_APP_PROGRAM_LEN: OnceCell<i32> = OnceCell::new();
static MAX_LOGIC_SIG_MAX_SIZE: OnceCell<u64> = OnceCell::new();
static MAX_TXN_NOTE_BYTES: OnceCell<i32> = OnceCell::new();
static MAX_TX_GROUP_SIZE: OnceCell<i32> = OnceCell::new();
static MAX_BYTES_KEY_VALUE_LEN: OnceCell<i32> = OnceCell::new();
static MAX_EXTRA_APP_PROGRAM_LEN: OnceCell<i32> = OnceCell::new();
static MAX_AVAILABLE_APP_PROGRAM_LEN: OnceCell<i32> = OnceCell::new();
static MAX_LOG_CALLS: OnceCell<i32> = OnceCell::new();
static MAX_INNER_TRANSACTIONS_PER_DELTA: OnceCell<i32> = OnceCell::new();
static MAX_PROPOSED_EXPIRED_ONLINE_ACCOUNTS: OnceCell<i32> = OnceCell::new();
fn check_set_alloc_bounds(consensus: &ConsensusProtocols) {
    let mut max_vote_threshold = 0u64;
    let mut max_eval_delta_keys = 0i32;
    let mut max_eval_delta_accounts = 0i32;
    let mut max_app_program_len = 0i32;
    let mut max_logic_sig_max_size = 0u64;
    let mut max_txn_note_bytes = 0i32;
    let mut max_tx_group_size = 0i32;
    let mut max_bytes_key_value_len = 0i32;
    let mut max_extra_app_program_len = 0i32;
    let mut max_available_app_program_len = 0i32;
    let mut max_log_calls = 0;
    let mut max_inner_transactions_per_delta = 0;
    let mut max_proposed_expired_online_accounts = 0;
    for (_, p) in consensus {
        check_set_max(p.soft_committee_threshold, &mut max_vote_threshold);
        check_set_max(p.cert_committee_threshold, &mut max_vote_threshold);
        check_set_max(p.next_committee_threshold, &mut max_vote_threshold);
        check_set_max(p.late_committee_threshold, &mut max_vote_threshold);
        check_set_max(p.redo_committee_threshold, &mut max_vote_threshold);
        check_set_max(p.down_committee_threshold, &mut max_vote_threshold);
        check_set_max(p.max_app_program_len, &mut max_eval_delta_keys);
        check_set_max(p.max_app_program_len, &mut max_eval_delta_accounts);
        check_set_max(p.max_app_program_len, &mut max_app_program_len);
        check_set_max(p.logic_sig_max_size, &mut max_logic_sig_max_size);
        check_set_max(p.max_txn_note_bytes, &mut max_txn_note_bytes);
        check_set_max(p.max_tx_group_size, &mut max_tx_group_size);
        check_set_max(p.max_app_key_len, &mut max_bytes_key_value_len);
        check_set_max(p.max_app_bytes_value_len, &mut max_bytes_key_value_len);
        check_set_max(
            p.max_extra_app_program_pages,
            &mut max_extra_app_program_len,
        );
        max_available_app_program_len = max_app_program_len * (1 + max_extra_app_program_len);
        check_set_max(p.max_app_program_len, &mut max_log_calls);
        check_set_max(
            p.max_inner_transactions * p.max_tx_group_size,
            &mut max_inner_transactions_per_delta,
        );
        check_set_max(
            p.max_proposed_expired_online_accounts,
            &mut max_proposed_expired_online_accounts,
        );
        MAX_VOTE_THRESHOLD.get_or_init(|| max_vote_threshold);
        MAX_EVAL_DELTA_KEYS.get_or_init(|| max_eval_delta_keys);
        MAX_EVAL_DELTA_ACCOUNTS.get_or_init(|| max_eval_delta_accounts);
        MAX_APP_PROGRAM_LEN.get_or_init(|| max_app_program_len);
        MAX_LOGIC_SIG_MAX_SIZE.get_or_init(|| max_logic_sig_max_size);
        MAX_TXN_NOTE_BYTES.get_or_init(|| max_txn_note_bytes);
        MAX_TX_GROUP_SIZE.get_or_init(|| max_tx_group_size);
        MAX_BYTES_KEY_VALUE_LEN.get_or_init(|| max_bytes_key_value_len);
        MAX_EXTRA_APP_PROGRAM_LEN.get_or_init(|| max_extra_app_program_len);
        MAX_AVAILABLE_APP_PROGRAM_LEN.get_or_init(|| max_available_app_program_len);
        MAX_LOG_CALLS.get_or_init(|| max_log_calls);
        MAX_INNER_TRANSACTIONS_PER_DELTA.get_or_init(|| max_inner_transactions_per_delta);
        MAX_PROPOSED_EXPIRED_ONLINE_ACCOUNTS.get_or_init(|| max_proposed_expired_online_accounts);
    }
}
pub fn init() {
    let _ = CONSENSUS.get_or_init(|| {
        let mut consensus_map = ConsensusProtocols::new();
        init_consensus_protocols(&mut consensus_map);
        consensus_map.into()
    });
    check_set_alloc_bounds(&*(CONSENSUS.get().unwrap().read().unwrap()));
}
