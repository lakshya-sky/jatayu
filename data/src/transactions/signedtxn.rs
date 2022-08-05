use crate::basics;
use crypto::{curve25519::Signature, logicsig::LogicSig, multisig::MultiSig};

use super::transaction::{ApplyData, Transaction};

#[derive(Debug, Clone)]
pub enum SignatureType {
    Sig(Signature),
    MutltiSig(MultiSig),
    LogicSig(LogicSig),
}

impl Default for SignatureType {
    fn default() -> Self {
        SignatureType::Sig(Signature::default())
    }
}

#[derive(Default, Debug, Clone)]
pub struct SignedTxn {
    pub sig: Signature,
    pub msig: MultiSig,
    pub lsig: LogicSig,
    pub txn: Transaction,
    pub auth_addr: basics::Address,
}

#[derive(Default, Debug, Clone)]
pub struct SignedTxnWithAD {
    pub signed_txn: SignedTxn,
    pub apply_data: ApplyData,
}

#[derive(Default, Debug, Clone)]
pub struct SignedTxnInBlock {
    pub sigend_txn_with_ad: SignedTxnWithAD,
    pub has_genesis_id: bool,
    pub has_genesis_hash: bool,
}
