use crypto::util::{HashDigest, MsgpHashable};
use serde::{Deserialize, Serialize};

use super::signedtxn::SignedTxnInBlock;

#[skip_serializing_default]
#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PaySet(pub Vec<SignedTxnInBlock>);

impl MsgpHashable for PaySet {
    fn hash_id(&self) -> protocol::HashId {
        protocol::PAYSET_FLAT
    }
}

impl PaySet {
    pub fn commit_genesis(&self) -> HashDigest {
        self.commit(true)
    }
    pub fn commit(&self, genesis: bool) -> HashDigest {
        let mut payset = Self(self.0.clone());
        if !genesis && self.0.len() == 0 {
            payset = Self(Default::default());
        }
        crypto::util::hash_obj(&payset)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use msgp::Marshaler;
    #[test]
    fn marshal_payset() {
        let payset = PaySet::default();
        let mut buffer = vec![];
        payset.marshal_msg(&mut buffer);
        assert_eq!(hex::encode(buffer), "90");
    }
}
