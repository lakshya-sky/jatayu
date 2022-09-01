use std::io::Read;
use std::path::Path;
use std::{env, fs, path::PathBuf};

use algod_config::consensus::{self};
use clap::Parser;
use data::bookkeeping::genesis;
use rand::prelude::*;
use rand::SeedableRng;

mod config;

#[derive(Parser, Debug)]
#[clap(name = "jatayud")]
struct Args {
    #[clap(short, long, value_parser)]
    data_dir: Option<PathBuf>,
    #[clap(short, long, value_parser)]
    genesis: Option<String>,
    #[clap(short('G'), long, value_parser)]
    genesis_print: bool,
    #[clap(short, long, value_parser)]
    version_check: bool,
    #[clap(short('n'), long, value_parser)]
    brach_check: bool,
    #[clap(short, long, value_parser)]
    channel_check: bool,
    #[clap(short('x'), long, value_parser)]
    init_and_exit: bool,
    #[clap(short('o'), long, value_parser)]
    log_to_stdout: bool,
    #[clap(short, long, value_parser)]
    peer_override: Option<String>,
    #[clap(short, long, value_parser)]
    listen_ip: Option<String>,
    #[clap(short, long, value_parser)]
    session_guid: Option<String>,
    #[clap(short, long, value_parser)]
    telemetry_override: Option<String>,
    #[clap(long, value_parser)]
    seed: Option<u64>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    //dbg!(&args);
    run(args)?;
    Ok(())
}

fn run(args: Args) -> Result<(), Box<dyn std::error::Error>> {
    consensus::init();
    let data_dir = match resolve_data_dir(&args.data_dir) {
        Some(data_dir) => data_dir,
        None => {
            panic!("data directory not set use -d option or set ALGORAND_DATA in your environment")
        }
    };
    let data_dir = fs::canonicalize(data_dir).expect("Data directory does not appear to be valid");
    let _rng: Box<dyn RngCore> = if let Some(s) = args.seed {
        Box::new(rand::rngs::StdRng::seed_from_u64(s))
    } else {
        Box::new(rand::rngs::OsRng)
    };
    let genesis_path = data_dir.as_path().join(config::GENESIS_JSON_FILE);
    let mut genesis_file = fs::File::open(genesis_path).expect("cannot open genesis file");
    let mut genesis_text = "".to_string();
    genesis_file
        .read_to_string(&mut genesis_text)
        .expect("cannot read genesis file");
    let genesis: genesis::Genesis =
        serde_json::from_str(genesis_text.as_str()).expect("cannot parse genesis file");
    if args.genesis_print {
        println!("{}", genesis.id());
        return Ok(());
    }
    let lock_path = Path::join(&data_dir, "algod.lock");
    let mut file_lock =
        fslock::LockFile::open(&lock_path).expect("unexpected failure in establishing algod.lock");
    file_lock.lock().expect(
        "failed to lock algod.lock, is an instance of algod already running on this data directory?"
    );

    let config = match algod_config::load_config_from_disk(&data_dir) {
        Ok(c) => c,
        Err(e) => panic!("Cannot load config: {:?}", e),
    };
    algod_config::consensus::load_configurable_consensus_protocols(&data_dir)?;
    let init = daemon::jatayud::ServerInit {
        root_path: data_dir.clone(),
        genesis,
        genesis_text,
        cfg: config,
    };
    let _ = daemon::jatayud::Server::new(init);

    Ok(())
}

fn resolve_data_dir(data_dir: &Option<PathBuf>) -> Option<PathBuf> {
    match data_dir {
        Some(d) => Some(d.clone()),
        None => match env::var("ALGORAND_DATA") {
            Ok(d) => Some(PathBuf::from(d)),
            Err(_) => None,
        },
    }
}
