use crypto::util::{HashDigest, MsgpHashable};

use super::signedtxn::SignedTxnInBlock;

#[derive(Default, Debug)]
pub struct PaySet(pub Vec<SignedTxnInBlock>);

impl msgp::Marshaler for PaySet {
    fn marshal_msg(&self, buf: Option<Vec<u8>>) -> Vec<u8> {
        todo!()
    }
}

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
