use digest::Digest;
use serde::Serialize;

const DIGEST_SIZE: usize = 32;
pub type HashDigest = [u8; DIGEST_SIZE];

pub fn sha256(data: &[u8; 32]) -> HashDigest {
    let mut output = [0u8; 32];
    output.copy_from_slice(sha2::Sha256::digest(data).as_slice());
    output
}

pub fn hash_obj<T: Serialize>(obj: T) -> HashDigest {
    let bytes = bincode::serde::encode_to_vec(obj, bincode::config::standard()).unwrap();
    let mut output = [0u8; 32];
    output.copy_from_slice(sha2::Sha256::digest(bytes).as_slice());
    output
}
