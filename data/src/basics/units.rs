use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
pub struct MicroAlgos(pub u64);

pub type Round = u64;
