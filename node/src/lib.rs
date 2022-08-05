mod top_account_listener;
use data::bookkeeping;
use std::{
    fs,
    path::{Path, PathBuf},
};
use top_account_listener::TopAccountListener;
use util::execpool::{Backlog, DedicatedExecutor};

pub struct AlgorandFullNode {
    pub config: config::Local,
    pub root_dir: PathBuf,
    pub genesis_id: String,
    pub genesis_hash: [u8; 32],
    pub dev_mode: bool,
    crypto_pool: DedicatedExecutor,
    low_priority_verification_pool: Backlog,
    high_priority_verification_pool: Backlog,
}
pub type NodeResult<T> = Result<T, Box<dyn std::error::Error>>;

impl AlgorandFullNode {
    pub fn new(
        root_dir: PathBuf,
        mut config: config::Local,
        genesis: &bookkeeping::genesis::Genesis,
    ) -> NodeResult<Self> {
        let genesis_id = genesis.id();
        let genesis_hash = crypto::util::hash_obj(genesis);
        let dev_mode = genesis.dev_mode;
        if dev_mode {
            config.disable_networking = true;
        }
        //let p2p_node = network::new_web_socket_network();
        let account_listner = TopAccountListener::new();
        let genesis_dir = Path::join(&root_dir, &genesis_id);
        let ledger_pathname_prefix = Path::join(&genesis_dir, config::LEDGER_FILENAME_PREFIX);
        let _ = fs::create_dir(&genesis_dir).expect("Unable to create genesis directory");
        let gen_alloc = genesis.balances()?;
        let crypto_pool = DedicatedExecutor::new("node_crypto_pool", None);
        let low_priority_backlog = util::execpool::Backlog::new(
            crypto_pool.clone(),
            util::execpool::Priority::LowPriority,
        );
        let high_priority_backlog = util::execpool::Backlog::new(
            crypto_pool.clone(),
            util::execpool::Priority::HighPriority,
        );

        Ok(Self {
            config,
            root_dir,
            genesis_id,
            genesis_hash,
            dev_mode,
            crypto_pool,
            low_priority_verification_pool: low_priority_backlog,
            high_priority_verification_pool: high_priority_backlog,
        })
    }
}
