use crypto::util::HashDigest;

#[derive(Debug, Default)]
pub struct Address(HashDigest);

impl Address {
    pub fn get_checksum(&self) {}
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn clone_from_slice(&mut self, slice: &[u8]) {
        self.0.clone_from_slice(slice);
    }
}
pub type AddressResult<T> = Result<T, Box<dyn std::error::Error>>;

const CHECKSUM_LENGTH: usize = 4;

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
        todo!();
    } else {
        Err(format!("failed to decode address {} to base 32", address).into())
    }
}
