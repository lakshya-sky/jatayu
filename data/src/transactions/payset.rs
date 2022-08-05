use crypto::util::HashDigest;

use super::signedtxn::SignedTxnInBlock;

#[derive(Default, Debug)]
pub struct PaySet(pub Vec<SignedTxnInBlock>);

impl PaySet {
    pub fn commit_genesis() -> HashDigest {}
    pub fn commit(&self, genesis: bool) -> HashDigest {
        let mut payset = Self(self.0.clone());
        if !genesis && self.0.len() == 0 {
            payset = Self(Default::default());
        }
        crypto::util::hash_obj(payset)
    }
}
