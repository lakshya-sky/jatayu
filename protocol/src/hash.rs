pub type HashId = &'static str;

// Hash IDs for specific object types, in lexicographic order.
// Hash IDs must be PREFIX-FREE (no hash ID is a prefix of another).
pub const APP_INDEX: HashId = "appID";

// ARCReserved is used to reserve prefixes starting with `arc` to
// ARCs-related hashes https://github.com/algorandfoundation/ARCs
// The prefix for ARC-XXXX should start with:
// "arcXXXX" (where "XXXX" is the 0-padded number of the ARC)
// For example ARC-0003 can use any prefix starting with "arc0003"
pub const ARC_RESERVED: HashId = "arc";

pub const AUCTION_BID: HashId = "aB";
pub const AUCTION_DEPOSIT: HashId = "aD";
pub const AUCTION_OUTCOMES: HashId = "aO";
pub const AUCTION_PARAMS: HashId = "aP";
pub const AUCTION_SETTLEMENT: HashId = "aS";

pub const COMPACT_CERT_COIN: HashId = "ccc";
pub const COMPACT_CERT_PART: HashId = "ccp";
pub const COMPACT_CERT_SIG: HashId = "ccs";
pub const AGREEMENT_SELECTOR: HashId = "AS";
pub const BLOCK_HEADER: HashId = "BH";
pub const BALANCE_RECORD: HashId = "BR";
pub const CREDENTIAL: HashId = "CR";
pub const GENESIS: HashId = "GE";
pub const KEYS_IN_MSS: HashId = "KP";
pub const MERKLE_ARRAY_NODE: HashId = "MA";
pub const MERKLE_VECTOR_COMMITMENT_BOTTOM_LEAF: HashId = "MB";
pub const MESSAGE: HashId = "MX";
pub const NET_PRIO_RESPONSE: HashId = "NPR";
pub const ONE_TIME_SIG_KEY1: HashId = "OT1";
pub const ONE_TIME_SIG_KEY2: HashId = "OT2";
pub const PAYSET_FLAT: HashId = "PF";
pub const PAYLOAD: HashId = "PL";
pub const PROGRAM: HashId = "Program";
pub const PROGRAM_DATA: HashId = "ProgData";
pub const PROPOSER_SEED: HashId = "PS";
pub const PARTICIPATION_KEYS: HashId = "PK";
pub const SEED: HashId = "SD";
pub const SPECIAL_ADDR: HashId = "SpecialAddr";
pub const SIGNED_TXN_IN_BLOCK: HashId = "STIB";
pub const TEST_HASHABLE: HashId = "TE";
pub const TX_GROUP: HashId = "TG";
pub const TXN_MERKLE_LEAF: HashId = "TL";
pub const TRANSACTION: HashId = "TX";
pub const VOTE: HashId = "VO";
