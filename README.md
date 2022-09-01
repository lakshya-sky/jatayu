# Jatayu
### Algorand Blockchain Participation Node written in Rust.
#### This repository is under development. Do not use it on Algorand Mainnet.
### Todo
- [ ] implement `skip_serializing_if_default` similar to [`skip_serializing_none`](https://docs.rs/serde_with/2.0.0/serde_with/attr.skip_serializing_none.html) 

### Why Rust Implementation:
- Having different implementations would improve network resilience meaning that if there’s an issue with one client, it won’t impact network availability

