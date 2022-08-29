use crate::bookkeeping::genesis;
use config::Local;
use crypto::util::HashDigest;
use ledger::BlockListener;
use protocol::ConsensusVersion;

pub fn load_ledger(
    db_filename_prefix: String,
    memory: bool,
    genesis_proto: ConsensusVersion,
    genesis_bal: genesis::GenesisBalances,
    genesis_id: String,
    genesis_hash: HashDigest,
    block_listeners: Vec<Box<dyn BlockListener>>,
    config: Local,
) {
    use msgp::Marshaler;
    let gen_block =
        genesis::make_genesis_block(genesis_proto, genesis_bal, genesis_id, genesis_hash).unwrap();
    let mut buffer = vec![];
    gen_block.marshal_msg(&mut buffer);
    println!("{:?}", buffer);
    todo!()
}
