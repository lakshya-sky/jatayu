pub trait Marshaler: serde::Serialize {
    fn marshal_msg(&self, bytes: &mut Vec<u8>);
}

impl<T: serde::Serialize> Marshaler for T {
    fn marshal_msg(&self, bytes: &mut Vec<u8>) {
        bytes.extend(rmp_serde::to_vec_named(self).unwrap());
    }
}
