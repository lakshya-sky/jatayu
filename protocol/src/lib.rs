pub type NetworkId = String;
pub type ConsensusVersion = String;

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConsensusVersions {
    V7,
    V8,
    V9,
    V10,
    V11,
}
