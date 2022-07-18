use crypto::util::{hash, HashDigest, DIGEST_SIZE};

const CHECKSUM_LENGTH: usize = 4;

#[derive(Debug, Default, Hash, PartialEq, Eq)]
pub struct Address(HashDigest);

impl Address {
    pub fn get_checksum(&self) -> [u8; CHECKSUM_LENGTH] {
        let short_addr_hash = hash(&self.0);
        let mut check_sum = [0u8; CHECKSUM_LENGTH];
        check_sum.clone_from_slice(&short_addr_hash[(short_addr_hash.len() - CHECKSUM_LENGTH)..]);
        check_sum
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn clone_from_slice(&mut self, slice: &[u8]) {
        self.0.clone_from_slice(slice);
    }

    pub fn string(&self) -> String {
        let mut addr_with_checksum = [0u8; DIGEST_SIZE + CHECKSUM_LENGTH];
        addr_with_checksum[..DIGEST_SIZE].copy_from_slice(&self.0);
        let short_addr_hash = hash(&self.0);
        addr_with_checksum[DIGEST_SIZE..]
            .copy_from_slice(&short_addr_hash[short_addr_hash.len() - CHECKSUM_LENGTH..]);
        String::from_utf8(addr_with_checksum.to_vec()).expect("failed to make string from address bytes")
    }
}
pub type AddressResult<T> = Result<T, Box<dyn std::error::Error>>;

pub fn unmarshal_checksum_address(address: &String) -> AddressResult<Address> {
    if let Some(decoded) = base32::decode(
        base32::Alphabet::RFC4648 { padding: true },
        address.as_str(),
    ) {
        let mut short = Address::default();
        if decoded.len() < short.len() {
            return Err(format!("decoded bad addr: {}", address).into());
        }
        short.clone_from_slice(decoded.as_slice());
        let incoming_checksum = &decoded[decoded.len() - CHECKSUM_LENGTH..];
        let calculated_checksum = short.get_checksum();
        if !calculated_checksum.eq(incoming_checksum) {
            return Err(format!(
                "address {} is malformed, checksum verification failed",
                address
            )
            .into());
        }
        Ok(short)
    } else {
        Err(format!("failed to decode address {} to base 32", address).into())
    }
}
