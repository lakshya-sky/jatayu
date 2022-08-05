use msgp::Marshaler;

fn encode_msgp<M: Marshaler>(m: &M) -> Vec<u8> {
    m.marshal_msg(None)
}

pub fn encode<M: Marshaler>(m: &M) -> Vec<u8> {
    encode_msgp(m)
}
