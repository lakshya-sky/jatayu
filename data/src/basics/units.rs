use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct MicroAlgos(u64);

pub type Round = u64;
