pub type Ed25519PublicKey = [u8; 32];
pub type Ed25519PrivateKey = [u8; 64];
pub type Ed25519Signature = [u8; 64];
pub type Ed25519Seed = [u8; 32];

const MASTER_DERIVATION_KEY_LEN: usize = 32;
pub type MaterDerivationKey = [u8; MASTER_DERIVATION_KEY_LEN];

type PrivateKey = Ed25519PrivateKey;
type PublicKey = Ed25519PublicKey;

pub fn ed25519Verify(
    _public: PublicKey,
    _data: &[u8],
    _sig: Ed25519Signature,
    _use_batch_verification_compatible_version: bool,
) -> bool {
    todo!()
}

pub fn ed25519Sign(_public: PrivateKey, _data: &[u8]) -> bool {
    todo!()
}
