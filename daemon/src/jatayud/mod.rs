mod api;
use config::Local;
use std::path::{Path, PathBuf};
use tokio::sync::oneshot;

use data::bookkeeping;
use node::AlgorandFullNode;

pub struct Server {
    pub root_path: PathBuf,
    pub genesis: bookkeeping::genesis::Genesis,
    pub pid_file: PathBuf,
    pub net_file: PathBuf,
    pub net_listen_file: PathBuf,
    pub node: node::AlgorandFullNode,
    pub stopping: oneshot::Receiver<()>,
    pub router_stop_sender: oneshot::Sender<()>,
}

pub struct ServerInit {
    pub root_path: PathBuf,
    pub genesis: bookkeeping::genesis::Genesis,
    pub genesis_text: String,
    pub cfg: Local,
}

impl Server {
    pub fn new(server_init: ServerInit) {
        let ServerInit {
            root_path,
            genesis,
            genesis_text,
            cfg,
        } = server_init;
        api::server::set_genesis_text(genesis_text);

        let _live_log = Path::join(&root_path, "node.log");
        let _archive = Path::join(&root_path, &cfg.log_archive_name);
        println!("logging to: {:?}", _live_log);
        let node = AlgorandFullNode::new(root_path.clone(), cfg.clone(), &genesis);
    }
}
