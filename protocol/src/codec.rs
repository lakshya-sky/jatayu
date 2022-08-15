use msgp::Marshaler;

fn encode_msgp<M: Marshaler>(m: &M) -> Vec<u8> {
    let mut buffer = vec![];
    m.marshal_msg(&mut buffer);
    buffer
}

pub fn encode<M: Marshaler>(m: &M) -> Vec<u8> {
    encode_msgp(m)
}
