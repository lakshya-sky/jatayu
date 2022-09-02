use digest::Digest;
use protocol;
use serde::{Deserialize, Serialize};

pub const DIGEST_SIZE: usize = 32;

#[derive(Serialize, Deserialize, Clone, Copy, Default, Debug, Hash, PartialEq, Eq)]
pub struct HashDigest(pub [u8; DIGEST_SIZE]);

impl HashDigest {
    pub const fn len(&self) -> usize {
        self.0.len()
    }
}
impl From<[u8; DIGEST_SIZE]> for HashDigest {
    fn from(f: [u8; DIGEST_SIZE]) -> Self {
        Self(f)
    }
}

pub trait MsgpHashable: msgp::Marshaler {
    fn to_be_hashed(&self) -> (protocol::HashId, Vec<u8>)
    where
        Self: Sized,
    {
        (self.hash_id(), protocol::encode(self))
    }

    fn hash_id(&self) -> protocol::HashId;
}

pub fn hash_rep(hashable: &impl MsgpHashable) -> Vec<u8> {
    let (hash_id, data) = hashable.to_be_hashed();
    let hash_id_bytes = hash_id.bytes();
    let mut hashed = Vec::with_capacity(hash_id_bytes.len() + data.len());
    hashed.extend(hash_id_bytes);
    hashed.extend(data);
    hashed
}

pub fn sha256(data: &[u8; 32]) -> HashDigest {
    let mut output = [0u8; 32];
    output.copy_from_slice(sha2::Sha256::digest(data).as_slice());
    output.into()
}

pub fn hash_obj(hashable: &impl MsgpHashable) -> HashDigest {
    hash(&hash_rep(hashable))
}

pub fn hash(data: &[u8]) -> HashDigest {
    let dg = sha2::Sha512_256::digest(data);
    HashDigest(dg.into())
}
