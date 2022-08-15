use serde::{Deserialize, Serialize};

use super::units;
use crypto::{onetimesig, vrf};

#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct AccountData {
    #[serde(rename = "onl")]
    #[serde(skip_serializing_if = "util::is_default")]
    pub status: u8,
    #[serde(rename = "algo")]
    #[serde(skip_serializing_if = "util::is_default")]
    pub microalgos: units::MicroAlgos,
    #[serde(rename = "ebase")]
    #[serde(skip_serializing_if = "util::is_default")]
    pub rewards_base: u64,
    #[serde(rename = "ern")]
    #[serde(skip_serializing_if = "util::is_default")]
    pub rewarded_micro_algos: units::MicroAlgos,
    #[serde(rename = "vote", with = "base64_bytes")]
    #[serde(skip_serializing_if = "util::is_default")]
    pub vote_id: onetimesig::OneTimeSignatureVerifier,
    #[serde(rename = "sel", with = "base64_bytes")]
    #[serde(skip_serializing_if = "util::is_default")]
    pub selection_id: vrf::VRFVerifier,
    #[serde(rename = "voteFst")]
    #[serde(skip_serializing_if = "util::is_default")]
    pub vote_first_valid: units::Round,
    #[serde(rename = "voteLst")]
    #[serde(skip_serializing_if = "util::is_default")]
    pub vote_last_valid: units::Round,
    #[serde(rename = "voteKD")]
    #[serde(skip_serializing_if = "util::is_default")]
    pub vote_key_dilution: u64,
}

mod base64_bytes {
    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};
    pub fn serialize<S: Serializer>(v: &[u8; 32], s: S) -> Result<S::Ok, S::Error> {
        v.serialize(s)
    }
    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 32], D::Error> {
        let b = String::deserialize(d)?;
        let b = base64::decode(b.as_bytes()).unwrap().try_into().unwrap();
        Ok(b)
    }
}

#[derive(Debug, Default)]
pub enum Status {
    #[default]
    Offline,
    Online,
    NotParticipating,
}

#[derive(Debug, Default)]
pub struct AccountDetails {
    pub address: super::Address,
    pub algos: super::MicroAlgos,
    pub status: Status,
}
