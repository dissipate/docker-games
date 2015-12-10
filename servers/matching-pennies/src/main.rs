extern crate zmq;
use std::thread::sleep;
use std::time::Duration;

extern crate rmp_serialize as msgpack;
extern crate rustc_serialize;

use rustc_serialize::{Encodable, Decodable};
use msgpack::{Encoder, Decoder};

//fn main() {
//    let val = Custom { id: 42u32, key: "the Answer".to_string() };

//    let mut buf = [0u8; 13];

//    val.encode(&mut Encoder::new(&mut &mut buf[..]));

//    assert_eq!([0x92, 0x2a, 0xaa, 0x74, 0x68, 0x65, 0x20, 0x41, 0x6e, 0x73, 0x77, 0x65, 0x72], buf);

    // Now try to unpack the buffer into the initial struct.
//    let mut decoder = Decoder::new(&buf[..]);
//    let res: Custom = Decodable::decode(&mut decoder).ok().unwrap();

//    assert_eq!(val, res);
//}


fn main() {
    #[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
    struct Pick{
        p: String
    }    

    let mut context = zmq::Context::new();
    let mut mpb1 = context.socket(zmq::REQ).unwrap();
    let mut mpb2 = context.socket(zmq::REQ).unwrap();

    mpb1.set_rcvhwm(1).unwrap();
    mpb2.set_rcvhwm(1).unwrap();

    assert!(mpb1.connect("tcp://mpb1:5555").is_ok());

    assert!(mpb2.connect("tcp://mpb2:5555").is_ok());

    let mut msg_req1 = zmq::Message::new().unwrap();
    let mut msg_req2 = zmq::Message::new().unwrap();

    let init_pick = Pick { p: "^".to_string() };

    let mut pick_buf = Vec::with_capacity(20);

    init_pick.encode(&mut Encoder::new(&mut &mut pick_buf)).unwrap();

    loop {

        println!("Sending ^");

        mpb1.send(&pick_buf, 0).unwrap();

        mpb1.recv(&mut msg_req1, 0).unwrap();
        println!("Received {}", msg_req1.as_str().unwrap());

        println!("Sending ^");
        mpb2.send(&pick_buf, 0).unwrap();

        mpb2.recv(&mut msg_req2, 0).unwrap();
        println!("Received {}", msg_req2.as_str().unwrap());


        std::thread::sleep(Duration::new(1,0));
    }

}
