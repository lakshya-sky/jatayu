use serde::{Deserialize, Serialize};

use super::units;
use crypto::{curve25519, onetimesig, vrf};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct AccountData {
    #[serde(rename = "onl")]
    status: u8,
    #[serde(rename = "algo")]
    microalgos: units::MicroAlgos,
    #[serde(rename = "vote", with = "base64_bytes")]
    vote_id: onetimesig::OneTimeSignatureVerifier,
    #[serde(rename = "sel", with = "base64_bytes")]
    selection_id: vrf::VRFVerifier,
    #[serde(rename = "voteFst")]
    vote_first_valid: units::Round,
    #[serde(rename = "voteLst")]
    vote_last_valid: units::Round,
    #[serde(rename = "voteKD")]
    vote_key_dilution: u64,
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

/*
mod base64Bytes {
    use base64::{decode, encode};
    use serde::{Deserialize, Serialize};
    use serde::{Deserializer, Serializer};

    pub fn serialize<S: Serializer>(v: &[u8], s: S) -> Result<S::Ok, S::Error> {
        let base64 = encode(v);
        String::serialize(&base64, s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<[u8; 32], D::Error> {
        let base64 = String::deserialize(d)?;
        decode(base64.as_bytes()).map().map_err(|e| serde::de::Error::custom(e))
    }
}*/
