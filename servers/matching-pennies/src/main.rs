extern crate zmq;
use std::thread::sleep;
use std::time::Duration;

extern crate rmp_serialize as msgpack;
extern crate rustc_serialize;

use rustc_serialize::{Encodable, Decodable};
use msgpack::{Encoder, Decoder};


fn main() {
    #[derive(RustcEncodable, RustcDecodable, PartialEq, Debug)]
    struct Pick{
        p: char 
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

    let init_pick = Pick { p: '^' };

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
