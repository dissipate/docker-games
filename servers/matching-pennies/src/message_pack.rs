extern crate rmp_serialize as msgpack;
extern crate rustc_serialize;

use rustc_serialize::{Encodable, Decodable};
use msgpack::{Encoder, Decoder};

#[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
struct Custom {
    id: u32,
    key: String,
}

fn main() {
    let val = Custom { id: 42u32, key: "the Answer".to_string() };

    let mut buf = [0u8; 13];

    val.encode(&mut Encoder::new(&mut &mut buf[..]));

    assert_eq!([0x92, 0x2a, 0xaa, 0x74, 0x68, 0x65, 0x20, 0x41, 0x6e, 0x73, 0x77, 0x65, 0x72], buf);

    // Now try to unpack the buffer into the initial struct.
    let mut decoder = Decoder::new(&buf[..]);
    let res: Custom = Decodable::decode(&mut decoder).ok().unwrap();

    assert_eq!(val, res);
}
