pub mod consensus;
mod local_template;

pub use local_template::{default_local, Local};

use std::path::Path;

type ConfigResult<T> = Result<T, Box<dyn std::error::Error>>;
// Devnet identifies the 'development network' use for development and not generally accessible publicly
//const Devnet protocol.NetworkID = "devnet"

// Betanet identifies the 'beta network' use for early releases of feature to the public prior to releasing these to mainnet/testnet
//const Betanet protocol.NetworkID = "betanet"

// Devtestnet identifies the 'development network for tests' use for running tests against development and not generally accessible publicly
//const Devtestnet protocol.NetworkID = "devtestnet"

// Testnet identifies the publicly-available test network
//const Testnet protocol.NetworkID = "testnet"

// Mainnet identifies the publicly-available real-money network
//const Mainnet protocol.NetworkID = "mainnet"

// GenesisJSONFile is the name of the genesis.json file
const GenesisJSONFile: &str = "genesis.json";

// Filenames of config files within the configdir (e.g. ~/.algorand)

// ConfigFilename is the name of the config.json file where we store per-algod-instance settings
const ConfigFilename: &str = "config.json";

// PhonebookFilename is the name of the phonebook configuration files - no longer used
const PhonebookFilename: &str = "phonebook.json"; // No longer used in product - still in tests

// LedgerFilenamePrefix is the prefix of the name of the ledger database files
pub const LEDGER_FILENAME_PREFIX: &str = "ledger";

// CrashFilename is the name of the agreement database file.
// It is used to recover from node crashes.
const CrashFilename: &str = "crash.sqlite";

// CompactCertFilename is the name of the compact certificate database file.
// It is used to track in-progress compact certificates.
const CompactCertFilename: &str = "compactcert.sqlite";

// ParticipationRegistryFilename is the name of the participation registry database file.
// It is used for tracking participation key metadata.
const ParticipationRegistryFilename: &str = "partregistry.sqlite";

// ConfigurableConsensusProtocolsFilename defines a set of consensus prototocols that
// are to be loaded from the data directory ( if present ), to override the
// built-in supported consensus protocols.
const ConfigurableConsensusProtocolsFilename: &str = "consensus.json";

// LoadConfigFromDisk returns a Local config structure based on merging the defaults
// with settings loaded from the config file from the custom dir.  If the custom file
// cannot be loaded, the default config is returned (with the error from loading the
// custom file).
pub fn load_config_from_disk<S: AsRef<Path>>(custom: S) -> ConfigResult<Local> {
    load_config_from_file(Path::join(custom.as_ref(), ConfigFilename))
}

fn load_config_from_file<P: AsRef<Path>>(config_file: P) -> ConfigResult<Local> {
    let mut c = default_local();
    c = merge_config_file(config_file.as_ref(), c)?;
    c = migrate(c)?;
    Ok(c)
}

fn merge_config_file(config_file: &Path, c: Local) -> ConfigResult<Local> {
    match std::fs::File::open(config_file) {
        Ok(f) => load_config(&f, c),
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => Ok(c),
            _ => Err(e.into()),
        },
    }
}

fn load_config(config_file: &std::fs::File, c: Local) -> ConfigResult<Local> {
    let _value: serde_json::Value = serde_json::from_reader(config_file)?;
    Ok(c)
}

fn migrate(c: Local) -> ConfigResult<Local> {
    Ok(c)
}
