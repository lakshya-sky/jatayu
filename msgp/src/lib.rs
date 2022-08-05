pub trait Marshaler {
    fn marshal_msg(&self, bytes: Option<Vec<u8>>) -> Vec<u8>;
}
