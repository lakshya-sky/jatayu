pub type NetworkId = String;
pub type ConsensusVersion = String;

pub const CONSENSUS_V7: &str = "v7";
pub const CONSENSUS_V8: &str = "v8";
pub const CONSENSUS_V9: &str = "v9";
pub const CONSENSUS_V10: &str = "v10";
pub const CONSENSUS_V11: &str = "v11";
pub const CONSENSUS_V12: &str = "v12";
pub const CONSENSUS_V13: &str =
    "https://github.com/algorand/spec/tree/0c8a9dc44d7368cc266d5407b79fb3311f4fc795";
pub const CONSENSUS_V14: &str =
    "https://github.com/algorand/spec/tree/2526b6ae062b4fe5e163e06e41e1d9b9219135a9";
pub const CONSENSUS_V15: &str =
    "https://github.com/algorand/spec/tree/a26ed78ed8f834e2b9ccb6eb7d3ee9f629a6e622";
pub const CONSENSUS_V16: &str =
    "https://github.com/algorand/spec/tree/22726c9dcd12d9cddce4a8bd7e8ccaa707f74101";
pub const CONSENSUS_V17: &str =
    "https://github.com/algorandfoundation/specs/tree/5615adc36bad610c7f165fa2967f4ecfa75125f0";
pub const CONSENSUS_V18: &str =
    "https://github.com/algorandfoundation/specs/tree/6c6bd668be0ab14098e51b37e806c509f7b7e31f";
pub const CONSENSUS_V19: &str =
    "https://github.com/algorandfoundation/specs/tree/0e196e82bfd6e327994bec373c4cc81bc878ef5c";
pub const CONSENSUS_V20: &str =
    "https://github.com/algorandfoundation/specs/tree/4a9db6a25595c6fd097cf9cc137cc83027787eaa";
pub const CONSENSUS_V21: &str =
    "https://github.com/algorandfoundation/specs/tree/8096e2df2da75c3339986317f9abe69d4fa86b4b";
pub const CONSENSUS_V22: &str =
    "https://github.com/algorandfoundation/specs/tree/57016b942f6d97e6d4c0688b373bb0a2fc85a1a2";
pub const CONSENSUS_V23: &str =
    "https://github.com/algorandfoundation/specs/tree/e5f565421d720c6f75cdd186f7098495caf9101f";
pub const CONSENSUS_V24: &str =
    "https://github.com/algorandfoundation/specs/tree/3a83c4c743f8b17adfd73944b4319c25722a6782";
pub const CONSENSUS_V25: &str =
    "https://github.com/algorandfoundation/specs/tree/bea19289bf41217d2c0af30522fa222ef1366466";
pub const CONSENSUS_V26: &str =
    "https://github.com/algorandfoundation/specs/tree/ac2255d586c4474d4ebcf3809acccb59b7ef34ff";
pub const CONSENSUS_V27: &str =
    "https://github.com/algorandfoundation/specs/tree/d050b3cade6d5c664df8bd729bf219f179812595";
pub const CONSENSUS_V28: &str =
    "https://github.com/algorandfoundation/specs/tree/65b4ab3266c52c56a0fa7d591754887d68faad0a";
pub const CONSENSUS_V29: &str =
    "https://github.com/algorandfoundation/specs/tree/abc54f79f9ad679d2d22f0fb9909fb005c16f8a1";
pub const CONSENSUS_V30: &str =
    "https://github.com/algorandfoundation/specs/tree/bc36005dbd776e6d1eaf0c560619bb183215645c";
pub const CONSENSUS_V31: &str =
    "https://github.com/algorandfoundation/specs/tree/85e6db1fdbdef00aa232c75199e10dc5fe9498f6";
pub const CONSENSUS_V32: &str =
    "https://github.com/algorandfoundation/specs/tree/d5ac876d7ede07367dbaa26e149aa42589aac1f7";
pub const CONSENSUS_VFUTURE: &str = "future";

//#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)/
//pub enum ConsensusVersions {
//    #[serde(rename = "v7")]
//    V7,
//    #[serde(rename = "v8")]
//    V8,
//    #[serde(rename = "v9")]
//    V9,
//    #[serde(rename = "v10")]
//    V10,
//    #[serde(rename = "v11")]
//    V11,
//    #[serde(rename = "v12")]
//    V12,
//    #[serde(rename = "v13")]
//    V13,
//    #[serde(rename = "v14")]
//    V14,
//    #[serde(rename = "v15")]
//    V15,
//    #[serde(rename = "v16")]
//    V16,
//    #[serde(rename = "v17")]
//    V17,
//    #[serde(rename = "v18")]
//    V18,
//    #[serde(rename = "v19")]
//    V19,
//    #[serde(rename = "v20")]
//    V20,
//    #[serde(rename = "v21")]
//    V21,
//    #[serde(rename = "v22")]
//    V22,
//    #[serde(rename = "v23")]
//    V23,
//    #[serde(rename = "v24")]
//    V24,
//    #[serde(rename = "v25")]
//    V25,
//    #[serde(rename = "v26")]
//    V26,
//    #[serde(rename = "v27")]
//    V27,
//    #[serde(rename = "v28")]
//    V28,
//    #[serde(rename = "v29")]
//    V29,
//    #[serde(rename = "v30")]
//    V30,
//    #[serde(rename = "v31")]
//    V31,
//    #[serde(rename = "v32")]
//    V32,
//    #[serde(rename = "future")]
//    VFuture,
//}
